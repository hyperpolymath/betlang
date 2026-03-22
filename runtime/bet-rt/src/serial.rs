// SPDX-License-Identifier: MIT OR Apache-2.0
//! Serialization module for Betlang runtime
//!
//! Supports JSON, MessagePack, and Apache Arrow formats.

use crate::value::{Ternary, Value};
use im::{HashMap, Vector};
use std::sync::Arc;

/// Serialization error types
#[derive(Debug, Clone)]
pub enum SerialError {
    Json(String),
    MessagePack(String),
    Arrow(String),
    UnsupportedType(String),
    InvalidFormat(String),
}

impl std::fmt::Display for SerialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerialError::Json(s) => write!(f, "JSON error: {}", s),
            SerialError::MessagePack(s) => write!(f, "MessagePack error: {}", s),
            SerialError::Arrow(s) => write!(f, "Arrow error: {}", s),
            SerialError::UnsupportedType(s) => write!(f, "Unsupported type: {}", s),
            SerialError::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
        }
    }
}

impl std::error::Error for SerialError {}

pub type SerialResult<T> = Result<T, SerialError>;

// ============================================================================
// JSON Serialization
// ============================================================================

/// JSON serialization and deserialization
pub mod json {
    use super::*;
    use serde_json::{self, Map, Number, Value as JsonValue};

    /// Serialize Value to JSON string
    pub fn to_string(value: &Value) -> SerialResult<String> {
        let json_value = value_to_json(value)?;
        serde_json::to_string(&json_value).map_err(|e| SerialError::Json(e.to_string()))
    }

    /// Serialize Value to pretty-printed JSON string
    pub fn to_string_pretty(value: &Value) -> SerialResult<String> {
        let json_value = value_to_json(value)?;
        serde_json::to_string_pretty(&json_value).map_err(|e| SerialError::Json(e.to_string()))
    }

    /// Serialize Value to JSON bytes
    pub fn to_bytes(value: &Value) -> SerialResult<Vec<u8>> {
        let json_value = value_to_json(value)?;
        serde_json::to_vec(&json_value).map_err(|e| SerialError::Json(e.to_string()))
    }

    /// Deserialize JSON string to Value
    pub fn from_str(s: &str) -> SerialResult<Value> {
        let json_value: JsonValue =
            serde_json::from_str(s).map_err(|e| SerialError::Json(e.to_string()))?;
        json_to_value(&json_value)
    }

    /// Deserialize JSON bytes to Value
    pub fn from_bytes(bytes: &[u8]) -> SerialResult<Value> {
        let json_value: JsonValue =
            serde_json::from_slice(bytes).map_err(|e| SerialError::Json(e.to_string()))?;
        json_to_value(&json_value)
    }

    /// Convert betlang Value to serde_json Value
    fn value_to_json(value: &Value) -> SerialResult<JsonValue> {
        match value {
            Value::Unit => Ok(JsonValue::Null),
            Value::Bool(b) => Ok(JsonValue::Bool(*b)),
            Value::Ternary(t) => match t {
                Ternary::True => Ok(JsonValue::Bool(true)),
                Ternary::False => Ok(JsonValue::Bool(false)),
                Ternary::Unknown => Ok(JsonValue::Null), // JSON has no unknown, use null
            },
            Value::Int(i) => Ok(JsonValue::Number(Number::from(*i))),
            Value::Float(f) => {
                if f.is_finite() {
                    Number::from_f64(*f)
                        .map(JsonValue::Number)
                        .ok_or_else(|| SerialError::Json("Invalid float value".to_string()))
                } else {
                    // Represent Infinity/NaN as null or string
                    if f.is_nan() {
                        Ok(JsonValue::String("NaN".to_string()))
                    } else if *f > 0.0 {
                        Ok(JsonValue::String("Infinity".to_string()))
                    } else {
                        Ok(JsonValue::String("-Infinity".to_string()))
                    }
                }
            }
            Value::String(s) => Ok(JsonValue::String(s.as_ref().clone())),
            Value::Bytes(b) => {
                // Encode bytes as base64 string
                use base64::Engine;
                let encoded = base64::engine::general_purpose::STANDARD.encode(b.as_ref());
                Ok(JsonValue::String(encoded))
            }
            Value::List(l) => {
                let arr: Result<Vec<_>, _> = l.iter().map(value_to_json).collect();
                Ok(JsonValue::Array(arr?))
            }
            Value::Map(m) => {
                let mut obj = Map::new();
                for (k, v) in m.iter() {
                    obj.insert(k.clone(), value_to_json(v)?);
                }
                Ok(JsonValue::Object(obj))
            }
            Value::Set(s) => {
                // Serialize set as array
                let arr: Result<Vec<_>, _> = s.keys().map(value_to_json).collect();
                Ok(JsonValue::Array(arr?))
            }
            Value::Tuple(t) => {
                let arr: Result<Vec<_>, _> = t.iter().map(value_to_json).collect();
                Ok(JsonValue::Array(arr?))
            }
            Value::Error(e) => Ok(JsonValue::Object({
                let mut m = Map::new();
                m.insert("error".to_string(), JsonValue::String(e.as_ref().clone()));
                m
            })),
            // Non-serializable types
            Value::Dist(_) => Err(SerialError::UnsupportedType(
                "Distribution cannot be serialized".to_string(),
            )),
            Value::Closure(_) => Err(SerialError::UnsupportedType(
                "Closure cannot be serialized".to_string(),
            )),
            Value::Native(_) => Err(SerialError::UnsupportedType(
                "Native function cannot be serialized".to_string(),
            )),
            Value::File(_) => Err(SerialError::UnsupportedType(
                "File handle cannot be serialized".to_string(),
            )),
        }
    }

    /// Convert serde_json Value to betlang Value
    fn json_to_value(json: &JsonValue) -> SerialResult<Value> {
        match json {
            JsonValue::Null => Ok(Value::Unit),
            JsonValue::Bool(b) => Ok(Value::Bool(*b)),
            JsonValue::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(Value::Int(i))
                } else if let Some(f) = n.as_f64() {
                    Ok(Value::Float(f))
                } else {
                    Err(SerialError::Json("Invalid number".to_string()))
                }
            }
            JsonValue::String(s) => {
                // Check for special float values
                match s.as_str() {
                    "NaN" => Ok(Value::Float(f64::NAN)),
                    "Infinity" => Ok(Value::Float(f64::INFINITY)),
                    "-Infinity" => Ok(Value::Float(f64::NEG_INFINITY)),
                    _ => Ok(Value::String(Arc::new(s.clone()))),
                }
            }
            JsonValue::Array(arr) => {
                let values: Result<Vector<_>, _> = arr.iter().map(json_to_value).collect();
                Ok(Value::List(values?))
            }
            JsonValue::Object(obj) => {
                // Check if it's an error object
                if obj.len() == 1 && obj.contains_key("error") {
                    if let Some(JsonValue::String(e)) = obj.get("error") {
                        return Ok(Value::Error(Arc::new(e.clone())));
                    }
                }
                let mut map = HashMap::new();
                for (k, v) in obj.iter() {
                    map.insert(k.clone(), json_to_value(v)?);
                }
                Ok(Value::Map(map))
            }
        }
    }

    /// Parse JSON and extract field by path (e.g., "data.items[0].name")
    pub fn get_path(json_str: &str, path: &str) -> SerialResult<Value> {
        let value = from_str(json_str)?;
        get_value_path(&value, path)
    }

    /// Extract value from nested structure by path
    fn get_value_path(value: &Value, path: &str) -> SerialResult<Value> {
        if path.is_empty() {
            return Ok(value.clone());
        }

        let parts: Vec<&str> = path.splitn(2, '.').collect();
        let (current, rest) = if parts.len() == 2 {
            (parts[0], parts[1])
        } else {
            (parts[0], "")
        };

        // Handle array index notation: field[index]
        if let Some(bracket_pos) = current.find('[') {
            let field = &current[..bracket_pos];
            let index_str = &current[bracket_pos + 1..current.len() - 1];
            let index: usize = index_str
                .parse()
                .map_err(|_| SerialError::InvalidFormat("Invalid array index".to_string()))?;

            let intermediate = if field.is_empty() {
                value.clone()
            } else {
                get_field(value, field)?
            };

            let element = get_index(&intermediate, index)?;
            get_value_path(&element, rest)
        } else {
            let field_value = get_field(value, current)?;
            get_value_path(&field_value, rest)
        }
    }

    fn get_field(value: &Value, field: &str) -> SerialResult<Value> {
        match value {
            Value::Map(m) => m
                .get(field)
                .cloned()
                .ok_or_else(|| SerialError::InvalidFormat(format!("Field '{}' not found", field))),
            _ => Err(SerialError::InvalidFormat(
                "Cannot access field on non-map".to_string(),
            )),
        }
    }

    fn get_index(value: &Value, index: usize) -> SerialResult<Value> {
        match value {
            Value::List(l) => l
                .get(index)
                .cloned()
                .ok_or_else(|| SerialError::InvalidFormat(format!("Index {} out of bounds", index))),
            Value::Tuple(t) => t
                .get(index)
                .cloned()
                .ok_or_else(|| SerialError::InvalidFormat(format!("Index {} out of bounds", index))),
            _ => Err(SerialError::InvalidFormat(
                "Cannot index non-list".to_string(),
            )),
        }
    }
}

// ============================================================================
// MessagePack Serialization
// ============================================================================

/// MessagePack serialization and deserialization
pub mod msgpack {
    use super::*;
    use rmp_serde::{self, Deserializer, Serializer};
    use serde::{Deserialize, Serialize};

    /// Intermediate representation for serde
    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    enum MsgPackValue {
        Null,
        Bool(bool),
        Int(i64),
        Float(f64),
        String(String),
        Bytes(Vec<u8>),
        Array(Vec<MsgPackValue>),
        Map(std::collections::HashMap<String, MsgPackValue>),
    }

    /// Serialize Value to MessagePack bytes
    pub fn to_bytes(value: &Value) -> SerialResult<Vec<u8>> {
        let mp_value = value_to_msgpack(value)?;
        let mut buf = Vec::new();
        mp_value
            .serialize(&mut Serializer::new(&mut buf))
            .map_err(|e| SerialError::MessagePack(e.to_string()))?;
        Ok(buf)
    }

    /// Deserialize MessagePack bytes to Value
    pub fn from_bytes(bytes: &[u8]) -> SerialResult<Value> {
        let mut de = Deserializer::new(bytes);
        let mp_value: MsgPackValue =
            Deserialize::deserialize(&mut de).map_err(|e| SerialError::MessagePack(e.to_string()))?;
        msgpack_to_value(&mp_value)
    }

    fn value_to_msgpack(value: &Value) -> SerialResult<MsgPackValue> {
        match value {
            Value::Unit => Ok(MsgPackValue::Null),
            Value::Bool(b) => Ok(MsgPackValue::Bool(*b)),
            Value::Ternary(t) => match t {
                Ternary::True => Ok(MsgPackValue::Bool(true)),
                Ternary::False => Ok(MsgPackValue::Bool(false)),
                Ternary::Unknown => Ok(MsgPackValue::Null),
            },
            Value::Int(i) => Ok(MsgPackValue::Int(*i)),
            Value::Float(f) => Ok(MsgPackValue::Float(*f)),
            Value::String(s) => Ok(MsgPackValue::String(s.as_ref().clone())),
            Value::Bytes(b) => Ok(MsgPackValue::Bytes(b.as_ref().clone())),
            Value::List(l) => {
                let arr: Result<Vec<_>, _> = l.iter().map(value_to_msgpack).collect();
                Ok(MsgPackValue::Array(arr?))
            }
            Value::Map(m) => {
                let mut map = std::collections::HashMap::new();
                for (k, v) in m.iter() {
                    map.insert(k.clone(), value_to_msgpack(v)?);
                }
                Ok(MsgPackValue::Map(map))
            }
            Value::Set(s) => {
                let arr: Result<Vec<_>, _> = s.keys().map(value_to_msgpack).collect();
                Ok(MsgPackValue::Array(arr?))
            }
            Value::Tuple(t) => {
                let arr: Result<Vec<_>, _> = t.iter().map(value_to_msgpack).collect();
                Ok(MsgPackValue::Array(arr?))
            }
            Value::Error(e) => {
                let mut map = std::collections::HashMap::new();
                map.insert("error".to_string(), MsgPackValue::String(e.as_ref().clone()));
                Ok(MsgPackValue::Map(map))
            }
            // Non-serializable types
            Value::Dist(_) | Value::Closure(_) | Value::Native(_) | Value::File(_) => {
                Err(SerialError::UnsupportedType(format!(
                    "{} cannot be serialized",
                    value.type_name()
                )))
            }
        }
    }

    fn msgpack_to_value(mp: &MsgPackValue) -> SerialResult<Value> {
        match mp {
            MsgPackValue::Null => Ok(Value::Unit),
            MsgPackValue::Bool(b) => Ok(Value::Bool(*b)),
            MsgPackValue::Int(i) => Ok(Value::Int(*i)),
            MsgPackValue::Float(f) => Ok(Value::Float(*f)),
            MsgPackValue::String(s) => Ok(Value::String(Arc::new(s.clone()))),
            MsgPackValue::Bytes(b) => Ok(Value::Bytes(Arc::new(b.clone()))),
            MsgPackValue::Array(arr) => {
                let values: Result<Vector<_>, _> = arr.iter().map(msgpack_to_value).collect();
                Ok(Value::List(values?))
            }
            MsgPackValue::Map(m) => {
                // Check for error
                if m.len() == 1 {
                    if let Some(MsgPackValue::String(e)) = m.get("error") {
                        return Ok(Value::Error(Arc::new(e.clone())));
                    }
                }
                let mut map = HashMap::new();
                for (k, v) in m.iter() {
                    map.insert(k.clone(), msgpack_to_value(v)?);
                }
                Ok(Value::Map(map))
            }
        }
    }
}

// ============================================================================
// Apache Arrow Serialization
// ============================================================================

/// Apache Arrow serialization for columnar data
pub mod arrow {
    use super::*;

    /// Column types supported by Arrow serialization
    #[derive(Debug, Clone)]
    pub enum ColumnType {
        Int64,
        Float64,
        Utf8,
        Boolean,
    }

    /// A simple tabular data structure for Arrow serialization
    #[derive(Debug, Clone)]
    pub struct Table {
        pub columns: Vec<(String, ColumnType, Vec<Value>)>,
        pub row_count: usize,
    }

    impl Table {
        /// Create empty table
        pub fn new() -> Self {
            Table {
                columns: Vec::new(),
                row_count: 0,
            }
        }

        /// Add column
        pub fn add_column(&mut self, name: &str, col_type: ColumnType, data: Vec<Value>) {
            if self.row_count == 0 {
                self.row_count = data.len();
            }
            self.columns.push((name.to_string(), col_type, data));
        }

        /// Get column by name
        pub fn get_column(&self, name: &str) -> Option<&Vec<Value>> {
            self.columns
                .iter()
                .find(|(n, _, _)| n == name)
                .map(|(_, _, data)| data)
        }

        /// Get row as map
        pub fn get_row(&self, index: usize) -> Option<HashMap<String, Value>> {
            if index >= self.row_count {
                return None;
            }
            let mut row = HashMap::new();
            for (name, _, data) in &self.columns {
                if let Some(value) = data.get(index) {
                    row.insert(name.clone(), value.clone());
                }
            }
            Some(row)
        }

        /// Convert to list of maps (row-oriented)
        pub fn to_records(&self) -> Vector<Value> {
            (0..self.row_count)
                .filter_map(|i| self.get_row(i).map(Value::Map))
                .collect()
        }

        /// Create from list of maps
        pub fn from_records(records: &Vector<Value>) -> SerialResult<Self> {
            if records.is_empty() {
                return Ok(Table::new());
            }

            // Get column names from first record
            let first = records.head().ok_or_else(|| {
                SerialError::Arrow("Empty records".to_string())
            })?;

            let column_names: Vec<String> = match first {
                Value::Map(m) => m.keys().cloned().collect(),
                _ => return Err(SerialError::Arrow("Records must be maps".to_string())),
            };

            // Infer column types from first record
            let mut table = Table::new();

            for name in &column_names {
                let mut col_data = Vec::new();
                let mut col_type = None;

                for record in records.iter() {
                    if let Value::Map(m) = record {
                        let value = m.get(name).cloned().unwrap_or(Value::Unit);

                        // Infer type from first non-unit value
                        if col_type.is_none() {
                            col_type = Some(match &value {
                                Value::Int(_) => ColumnType::Int64,
                                Value::Float(_) => ColumnType::Float64,
                                Value::String(_) => ColumnType::Utf8,
                                Value::Bool(_) => ColumnType::Boolean,
                                _ => ColumnType::Utf8, // Default to string
                            });
                        }
                        col_data.push(value);
                    }
                }

                table.add_column(name, col_type.unwrap_or(ColumnType::Utf8), col_data);
            }

            Ok(table)
        }
    }

    impl Default for Table {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Serialize table to Arrow IPC format bytes
    /// Note: This is a simplified implementation. Full Arrow support requires
    /// the arrow crate which is large. This provides basic functionality.
    pub fn to_bytes(table: &Table) -> SerialResult<Vec<u8>> {
        // For now, serialize as JSON with type hints
        // A full implementation would use arrow::ipc
        let mut result = Vec::new();

        // Header: column count, row count
        result.extend_from_slice(&(table.columns.len() as u32).to_le_bytes());
        result.extend_from_slice(&(table.row_count as u64).to_le_bytes());

        // Column metadata
        for (name, col_type, _) in &table.columns {
            // Name length and bytes
            let name_bytes = name.as_bytes();
            result.extend_from_slice(&(name_bytes.len() as u32).to_le_bytes());
            result.extend_from_slice(name_bytes);

            // Type tag
            let type_tag: u8 = match col_type {
                ColumnType::Int64 => 0,
                ColumnType::Float64 => 1,
                ColumnType::Utf8 => 2,
                ColumnType::Boolean => 3,
            };
            result.push(type_tag);
        }

        // Column data
        for (_, col_type, data) in &table.columns {
            match col_type {
                ColumnType::Int64 => {
                    for value in data {
                        let n = match value {
                            Value::Int(i) => *i,
                            _ => 0,
                        };
                        result.extend_from_slice(&n.to_le_bytes());
                    }
                }
                ColumnType::Float64 => {
                    for value in data {
                        let n = match value {
                            Value::Float(f) => *f,
                            Value::Int(i) => *i as f64,
                            _ => 0.0,
                        };
                        result.extend_from_slice(&n.to_le_bytes());
                    }
                }
                ColumnType::Utf8 => {
                    for value in data {
                        let s = match value {
                            Value::String(s) => s.as_ref().clone(),
                            _ => format!("{}", value),
                        };
                        let bytes = s.as_bytes();
                        result.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
                        result.extend_from_slice(bytes);
                    }
                }
                ColumnType::Boolean => {
                    for value in data {
                        let b = match value {
                            Value::Bool(b) => *b,
                            Value::Ternary(Ternary::True) => true,
                            _ => false,
                        };
                        result.push(if b { 1 } else { 0 });
                    }
                }
            }
        }

        Ok(result)
    }

    /// Deserialize Arrow IPC format bytes to table
    pub fn from_bytes(bytes: &[u8]) -> SerialResult<Table> {
        if bytes.len() < 12 {
            return Err(SerialError::Arrow("Invalid Arrow data".to_string()));
        }

        let mut pos = 0;

        // Read header
        let col_count = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize;
        pos += 4;
        let row_count = u64::from_le_bytes(bytes[pos..pos + 8].try_into().unwrap()) as usize;
        pos += 8;

        // Read column metadata
        let mut columns_meta: Vec<(String, ColumnType)> = Vec::new();
        for _ in 0..col_count {
            let name_len = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize;
            pos += 4;
            let name = String::from_utf8_lossy(&bytes[pos..pos + name_len]).to_string();
            pos += name_len;

            let type_tag = bytes[pos];
            pos += 1;
            let col_type = match type_tag {
                0 => ColumnType::Int64,
                1 => ColumnType::Float64,
                2 => ColumnType::Utf8,
                3 => ColumnType::Boolean,
                _ => return Err(SerialError::Arrow("Unknown column type".to_string())),
            };
            columns_meta.push((name, col_type));
        }

        // Read column data
        let mut table = Table::new();
        for (name, col_type) in columns_meta {
            let mut data = Vec::new();
            match col_type {
                ColumnType::Int64 => {
                    for _ in 0..row_count {
                        let n = i64::from_le_bytes(bytes[pos..pos + 8].try_into().unwrap());
                        pos += 8;
                        data.push(Value::Int(n));
                    }
                }
                ColumnType::Float64 => {
                    for _ in 0..row_count {
                        let n = f64::from_le_bytes(bytes[pos..pos + 8].try_into().unwrap());
                        pos += 8;
                        data.push(Value::Float(n));
                    }
                }
                ColumnType::Utf8 => {
                    for _ in 0..row_count {
                        let len = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize;
                        pos += 4;
                        let s = String::from_utf8_lossy(&bytes[pos..pos + len]).to_string();
                        pos += len;
                        data.push(Value::String(Arc::new(s)));
                    }
                }
                ColumnType::Boolean => {
                    for _ in 0..row_count {
                        let b = bytes[pos] != 0;
                        pos += 1;
                        data.push(Value::Bool(b));
                    }
                }
            }
            table.add_column(&name, col_type, data);
        }

        Ok(table)
    }
}

// ============================================================================
// CSV Support (text-based tabular)
// ============================================================================

/// CSV serialization for interoperability
pub mod csv {
    use super::*;

    /// Parse CSV string to list of maps
    pub fn parse(input: &str, has_header: bool) -> SerialResult<Vector<Value>> {
        let lines: Vec<&str> = input.lines().collect();
        if lines.is_empty() {
            return Ok(Vector::new());
        }

        let headers: Vec<String> = if has_header {
            parse_csv_row(lines[0])
        } else {
            (0..parse_csv_row(lines[0]).len())
                .map(|i| format!("col{}", i))
                .collect()
        };

        let data_start = if has_header { 1 } else { 0 };
        let mut records = Vector::new();

        for line in lines.iter().skip(data_start) {
            let values = parse_csv_row(line);
            let mut row = HashMap::new();
            for (i, value) in values.iter().enumerate() {
                if i < headers.len() {
                    row.insert(headers[i].clone(), parse_csv_value(value));
                }
            }
            records.push_back(Value::Map(row));
        }

        Ok(records)
    }

    /// Convert list of maps to CSV string
    pub fn stringify(records: &Vector<Value>, headers: Option<&[&str]>) -> SerialResult<String> {
        if records.is_empty() {
            return Ok(String::new());
        }

        // Get headers from first record if not provided
        let header_list: Vec<String> = match headers {
            Some(h) => h.iter().map(|s| s.to_string()).collect(),
            None => {
                if let Some(Value::Map(m)) = records.head() {
                    m.keys().cloned().collect()
                } else {
                    return Err(SerialError::InvalidFormat("Records must be maps".to_string()));
                }
            }
        };

        let mut result = String::new();

        // Write header
        result.push_str(&header_list.join(","));
        result.push('\n');

        // Write rows
        for record in records.iter() {
            if let Value::Map(m) = record {
                let values: Vec<String> = header_list
                    .iter()
                    .map(|h| {
                        m.get(h)
                            .map(|v| escape_csv_value(v))
                            .unwrap_or_default()
                    })
                    .collect();
                result.push_str(&values.join(","));
                result.push('\n');
            }
        }

        Ok(result)
    }

    fn parse_csv_row(line: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;

        for c in line.chars() {
            match c {
                '"' => in_quotes = !in_quotes,
                ',' if !in_quotes => {
                    result.push(current.trim().to_string());
                    current = String::new();
                }
                _ => current.push(c),
            }
        }
        result.push(current.trim().to_string());
        result
    }

    fn parse_csv_value(s: &str) -> Value {
        // Try to parse as number
        if let Ok(i) = s.parse::<i64>() {
            return Value::Int(i);
        }
        if let Ok(f) = s.parse::<f64>() {
            return Value::Float(f);
        }
        // Boolean
        if s.eq_ignore_ascii_case("true") {
            return Value::Bool(true);
        }
        if s.eq_ignore_ascii_case("false") {
            return Value::Bool(false);
        }
        // Default to string
        Value::String(Arc::new(s.to_string()))
    }

    fn escape_csv_value(v: &Value) -> String {
        match v {
            Value::String(s) => {
                if s.contains(',') || s.contains('"') || s.contains('\n') {
                    format!("\"{}\"", s.replace('"', "\"\""))
                } else {
                    s.as_ref().clone()
                }
            }
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            _ => format!("{}", v),
        }
    }
}

// ============================================================================
// Native function bindings
// ============================================================================

use crate::value::NativeFunction;

/// Get all serialization native functions
pub fn native_functions() -> Vec<NativeFunction> {
    vec![
        NativeFunction {
            name: "json_encode",
            arity: 1,
            func: |args| {
                if let Some(v) = args.first() {
                    match json::to_string(v) {
                        Ok(s) => Ok(Value::String(Arc::new(s))),
                        Err(e) => Ok(Value::Error(Arc::new(e.to_string()))),
                    }
                } else {
                    Err("json_encode expects a value".to_string())
                }
            },
        },
        NativeFunction {
            name: "json_decode",
            arity: 1,
            func: |args| {
                if let Some(Value::String(s)) = args.first() {
                    match json::from_str(s) {
                        Ok(v) => Ok(v),
                        Err(e) => Ok(Value::Error(Arc::new(e.to_string()))),
                    }
                } else {
                    Err("json_decode expects a string".to_string())
                }
            },
        },
        NativeFunction {
            name: "json_pretty",
            arity: 1,
            func: |args| {
                if let Some(v) = args.first() {
                    match json::to_string_pretty(v) {
                        Ok(s) => Ok(Value::String(Arc::new(s))),
                        Err(e) => Ok(Value::Error(Arc::new(e.to_string()))),
                    }
                } else {
                    Err("json_pretty expects a value".to_string())
                }
            },
        },
        NativeFunction {
            name: "msgpack_encode",
            arity: 1,
            func: |args| {
                if let Some(v) = args.first() {
                    match msgpack::to_bytes(v) {
                        Ok(b) => Ok(Value::Bytes(Arc::new(b))),
                        Err(e) => Ok(Value::Error(Arc::new(e.to_string()))),
                    }
                } else {
                    Err("msgpack_encode expects a value".to_string())
                }
            },
        },
        NativeFunction {
            name: "msgpack_decode",
            arity: 1,
            func: |args| {
                if let Some(Value::Bytes(b)) = args.first() {
                    match msgpack::from_bytes(b) {
                        Ok(v) => Ok(v),
                        Err(e) => Ok(Value::Error(Arc::new(e.to_string()))),
                    }
                } else {
                    Err("msgpack_decode expects bytes".to_string())
                }
            },
        },
        NativeFunction {
            name: "csv_parse",
            arity: 1,
            func: |args| {
                if let Some(Value::String(s)) = args.first() {
                    match csv::parse(s, true) {
                        Ok(v) => Ok(Value::List(v)),
                        Err(e) => Ok(Value::Error(Arc::new(e.to_string()))),
                    }
                } else {
                    Err("csv_parse expects a string".to_string())
                }
            },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_roundtrip() {
        let value = Value::Map({
            let mut m = HashMap::new();
            m.insert("name".to_string(), Value::String(Arc::new("test".to_string())));
            m.insert("count".to_string(), Value::Int(42));
            m
        });

        let json_str = json::to_string(&value).unwrap();
        let parsed = json::from_str(&json_str).unwrap();

        assert_eq!(value, parsed);
    }

    #[test]
    fn test_msgpack_roundtrip() {
        let value = Value::List(Vector::from(vec![
            Value::Int(1),
            Value::Int(2),
            Value::Int(3),
        ]));

        let bytes = msgpack::to_bytes(&value).unwrap();
        let parsed = msgpack::from_bytes(&bytes).unwrap();

        assert_eq!(value, parsed);
    }

    #[test]
    fn test_csv_parse() {
        let csv_data = "name,age\nAlice,30\nBob,25";
        let records = csv::parse(csv_data, true).unwrap();

        assert_eq!(records.len(), 2);
    }
}
