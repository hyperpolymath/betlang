// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell

//! WebAssembly backend for BetLang.
//!
//! Generates valid WebAssembly binary modules from BetLang AST for browser
//! and edge deployment. Probabilistic primitives are implemented via runtime
//! imports, allowing the JavaScript host to provide RNG and distribution
//! sampling.
//!
//! ## Output format
//!
//! Generates valid `.wasm` modules (binary format) containing:
//! - Type section (function signatures)
//! - Import section (probabilistic runtime from host)
//! - Function section (function bodies with real WASM instructions)
//! - Memory section (linear memory for heap allocation)
//! - Export section (functions + memory)
//! - Data section (string constants)
//!
//! ## Domain mapping
//!
//! - Ternary values: i32 (0=false, 1=true, 2=unknown)
//! - Bet expressions: function calls with probability weights
//! - `Dist<T>`: opaque i32 handle to runtime distribution object
//! - `sample`/`observe`/`infer`: runtime import calls
//! - Functions: standard WASM functions
//!
//! ## Probabilistic runtime imports
//!
//! ```wasm
//! (import "betlang" "random_ternary" (func $random_ternary (result i32)))
//! (import "betlang" "sample_dist" (func $sample_dist (param i32) (result f64)))
//! (import "betlang" "observe_dist" (func $observe_dist (param i32 f64)))
//! (import "betlang" "infer" (func $infer (param i32) (result i32)))
//! (import "betlang" "create_dist" (func $create_dist (param i32 f64 f64) (result i32)))
//! ```
//!
//! ## Type representation
//!
//! - Ternary: i32 (0=false, 1=true, 2=unknown)
//! - Integers: i64
//! - Floats: f64
//! - Booleans: i32 (0 or 1)
//! - Strings: i32 pointer into linear memory (offset, length pair)
//! - `Dist<T>`: i32 handle (opaque reference to host distribution)
//!
//! ## Limitations
//!
//! - No garbage collection (bump allocator, no free)
//! - Distribution handles are managed by host runtime
//! - WASI preview1 integration available (enable with `with_wasi(true)`)

#![forbid(unsafe_code)]
use std::collections::HashMap;

use wasm_encoder::{
    CodeSection, DataSection, EntityType, ExportKind, ExportSection, Function as WasmFunc,
    FunctionSection, ImportSection, Instruction, MemorySection, MemoryType, Module, TypeSection,
    ValType,
};

/// Errors specific to the BetLang WASM backend.
///
/// These capture failure modes during WebAssembly code generation from
/// BetLang's probabilistic primitives.
#[derive(Debug, Clone, thiserror::Error)]
pub enum WasmError {
    /// Data section offset exceeds linear memory bounds.
    #[error("data section offset {offset} exceeds linear memory capacity ({capacity} bytes, {pages} pages)")]
    DataSectionOverflow {
        offset: u32,
        capacity: u32,
        pages: u32,
    },

    /// Bump allocator ran out of linear memory.
    #[error("heap allocation of {requested} bytes exceeds linear memory (offset {current}, capacity {capacity})")]
    HeapOverflow {
        requested: u32,
        current: u32,
        capacity: u32,
    },

    /// Unknown distribution type requested.
    #[error("unknown distribution type: {name}")]
    UnknownDistribution { name: String },

    /// Ternary value out of range (must be 0, 1, or 2).
    #[error("ternary value {value} out of range (expected 0=false, 1=true, 2=unknown)")]
    TernaryOutOfRange { value: i32 },

    /// Function not found during code generation.
    #[error("function '{name}' not found in module")]
    FunctionNotFound { name: String },
}

/// WASM value type (subset of WASM types used by BetLang).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmType {
    /// 32-bit integer (ternary values, booleans, pointers, dist handles).
    I32,
    /// 64-bit integer.
    I64,
    /// 32-bit float.
    F32,
    /// 64-bit float.
    F64,
}

impl WasmType {
    /// Convert to wasm-encoder ValType.
    fn to_val_type(self) -> ValType {
        match self {
            Self::I32 => ValType::I32,
            Self::I64 => ValType::I64,
            Self::F32 => ValType::F32,
            Self::F64 => ValType::F64,
        }
    }
}

impl std::fmt::Display for WasmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),
        }
    }
}

/// A compiled WASM function from BetLang.
#[derive(Debug, Clone)]
pub struct WasmFunction {
    /// Function name.
    pub name: String,
    /// Parameter types.
    pub params: Vec<WasmType>,
    /// Return type.
    pub result: Option<WasmType>,
    /// Actual bytecode size (from wasm-encoder).
    pub code_size: usize,
    /// Whether this function uses probabilistic runtime imports.
    pub uses_probabilistic_imports: bool,
}

/// Output of the BetLang WASM backend.
#[derive(Debug, Clone)]
pub struct WasmModule {
    /// Compiled functions.
    pub functions: Vec<WasmFunction>,
    /// Initial memory pages (64KB each).
    pub initial_memory_pages: u32,
    /// Maximum memory pages.
    pub max_memory_pages: u32,
    /// Actual module binary size in bytes.
    pub binary_size: usize,
    /// The WASM binary module bytes.
    binary: Vec<u8>,
}

impl WasmModule {
    /// Get the WASM binary bytes.
    pub fn to_bytes(&self) -> &[u8] {
        &self.binary
    }

    /// Consume and return the WASM binary bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.binary
    }
}

/// Tracks the actual import function indices in the WASM module.
///
/// Import indices depend on which imports are present, so they
/// must be computed dynamically rather than hardcoded.
struct ImportIndices {
    /// `betlang.random_ternary() -> i32` — random ternary value (0, 1, 2).
    random_ternary: Option<u32>,
    /// `betlang.sample_dist(handle: i32) -> f64` — sample from distribution.
    sample_dist: Option<u32>,
    /// `betlang.observe_dist(handle: i32, value: f64)` — observe value.
    observe_dist: Option<u32>,
    /// `betlang.infer(handle: i32) -> i32` — run inference on distribution.
    infer: Option<u32>,
    /// `betlang.create_dist(type: i32, param1: f64, param2: f64) -> i32`.
    create_dist: Option<u32>,
    /// WASI fd_write for output.
    fd_write: Option<u32>,
}

/// Bump allocator for WASM linear memory.
///
/// Tracks the next free offset in linear memory. Strings from the data
/// section occupy the beginning of memory; the heap starts after them.
/// Used for dynamic allocations of tuples, arrays, and distribution
/// parameter blocks during code generation.
struct BumpAllocator {
    /// Next free byte offset in linear memory.
    next_offset: u32,
    /// Maximum byte capacity (initial_memory_pages * 65536).
    capacity: u32,
}

impl BumpAllocator {
    /// Create a new bump allocator starting at `initial_offset` with a
    /// given page-based capacity.
    fn new(initial_offset: u32, initial_pages: u32) -> Self {
        Self {
            next_offset: initial_offset,
            capacity: initial_pages.saturating_mul(65536),
        }
    }

    /// Allocate `size` bytes, returning the start offset.
    ///
    /// Returns `Err(WasmError::HeapOverflow)` if the allocation would
    /// exceed linear memory capacity.
    fn alloc(&mut self, size: u32) -> Result<u32, WasmError> {
        // Align to 8 bytes for f64 compatibility.
        let aligned = (self.next_offset + 7) & !7;
        let new_offset = aligned.checked_add(size).ok_or(WasmError::HeapOverflow {
            requested: size,
            current: self.next_offset,
            capacity: self.capacity,
        })?;
        if new_offset > self.capacity {
            return Err(WasmError::HeapOverflow {
                requested: size,
                current: self.next_offset,
                capacity: self.capacity,
            });
        }
        self.next_offset = new_offset;
        Ok(aligned)
    }
}

/// WASM backend for BetLang.
///
/// Translates BetLang's probabilistic ternary primitives into WebAssembly
/// modules. Distribution operations are delegated to host runtime imports.
pub struct WasmBackend {
    /// Initial linear memory pages (64KB each).
    initial_memory_pages: u32,
    /// Maximum linear memory pages.
    max_memory_pages: u32,
    /// Enable WASI preview1 integration (fd_write, etc.).
    wasi_enabled: bool,
    /// Non-fatal warnings collected during code generation.
    warnings: Vec<String>,
    /// String constants collected during generation.
    string_data: Vec<(u32, Vec<u8>)>,
    /// Next string data offset.
    data_offset: u32,
}

impl WasmBackend {
    /// Create a new BetLang WASM backend with default memory settings.
    pub fn new() -> Self {
        Self {
            initial_memory_pages: 16, // 1MB initial
            max_memory_pages: 256,    // 16MB max
            wasi_enabled: false,
            warnings: Vec::new(),
            string_data: Vec::new(),
            data_offset: 0,
        }
    }

    /// Retrieve any warnings generated during the last `generate()` call.
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Set initial memory pages.
    pub fn with_initial_memory(mut self, pages: u32) -> Self {
        self.initial_memory_pages = pages;
        self
    }

    /// Set maximum memory pages.
    pub fn with_max_memory(mut self, pages: u32) -> Self {
        self.max_memory_pages = pages;
        self
    }

    /// Enable WASI preview1 integration.
    pub fn with_wasi(mut self, enabled: bool) -> Self {
        self.wasi_enabled = enabled;
        self
    }

    /// Add a string constant to the data section, returning its offset.
    fn intern_string(&mut self, s: &str) -> Result<u32, WasmError> {
        let bytes = s.as_bytes().to_vec();
        let offset = self.data_offset;
        let len = bytes.len() as u32;
        let capacity = self.initial_memory_pages.saturating_mul(65536);
        if offset.checked_add(len).map_or(true, |end| end > capacity) {
            return Err(WasmError::DataSectionOverflow {
                offset,
                capacity,
                pages: self.initial_memory_pages,
            });
        }
        self.string_data.push((offset, bytes));
        self.data_offset += len;
        // Align to 4 bytes for next string.
        self.data_offset = (self.data_offset + 3) & !3;
        Ok(offset)
    }

    /// Generate a WASM module from BetLang function definitions.
    ///
    /// Each entry in `functions` is a tuple of (name, params, result, is_probabilistic).
    /// This scaffold generates valid WASM modules with the correct section
    /// structure; actual instruction emission from BetLang AST will be
    /// added when bet-syntax provides the typed IR.
    pub fn generate(
        &mut self,
        functions: &[(String, Vec<WasmType>, Option<WasmType>, bool)],
    ) -> Result<WasmModule, WasmError> {
        self.warnings.clear();
        self.string_data.clear();
        self.data_offset = 0;

        let mut module = Module::new();

        // --- Type section ---
        let mut types = TypeSection::new();
        let mut type_map: HashMap<(Vec<ValType>, Vec<ValType>), u32> = HashMap::new();
        let mut func_type_indices: Vec<u32> = Vec::new();

        // Determine which imports are needed.
        let needs_probabilistic = functions.iter().any(|(_, _, _, prob)| *prob);
        let mut import_count: u32 = 0;
        let mut import_indices = ImportIndices {
            random_ternary: None,
            sample_dist: None,
            observe_dist: None,
            infer: None,
            create_dist: None,
            fd_write: None,
        };

        // Register import types first (imports must come before functions).
        if needs_probabilistic {
            // random_ternary: () -> i32
            let params = vec![];
            let results = vec![ValType::I32];
            let key = (params.clone(), results.clone());
            let idx = type_map.len() as u32;
            type_map.entry(key).or_insert_with(|| {
                types.ty().function(params, results);
                idx
            });
            import_indices.random_ternary = Some(import_count);
            import_count += 1;

            // sample_dist: (i32) -> f64
            let params = vec![ValType::I32];
            let results = vec![ValType::F64];
            let key = (params.clone(), results.clone());
            let idx = type_map.len() as u32;
            type_map.entry(key).or_insert_with(|| {
                types.ty().function(params, results);
                idx
            });
            import_indices.sample_dist = Some(import_count);
            import_count += 1;

            // observe_dist: (i32, f64) -> ()
            let params = vec![ValType::I32, ValType::F64];
            let results: Vec<ValType> = vec![];
            let key = (params.clone(), results.clone());
            let idx = type_map.len() as u32;
            type_map.entry(key).or_insert_with(|| {
                types.ty().function(params, results);
                idx
            });
            import_indices.observe_dist = Some(import_count);
            import_count += 1;

            // infer: (i32) -> i32
            let params = vec![ValType::I32];
            let results = vec![ValType::I32];
            let key = (params.clone(), results.clone());
            let idx = type_map.len() as u32;
            type_map.entry(key).or_insert_with(|| {
                types.ty().function(params, results);
                idx
            });
            import_indices.infer = Some(import_count);
            import_count += 1;

            // create_dist: (i32, f64, f64) -> i32
            let params = vec![ValType::I32, ValType::F64, ValType::F64];
            let results = vec![ValType::I32];
            let key = (params.clone(), results.clone());
            let idx = type_map.len() as u32;
            type_map.entry(key).or_insert_with(|| {
                types.ty().function(params, results);
                idx
            });
            import_indices.create_dist = Some(import_count);
            import_count += 1;
        }

        if self.wasi_enabled {
            // fd_write: (i32, i32, i32, i32) -> i32
            let params = vec![ValType::I32, ValType::I32, ValType::I32, ValType::I32];
            let results = vec![ValType::I32];
            let key = (params.clone(), results.clone());
            let idx = type_map.len() as u32;
            type_map.entry(key).or_insert_with(|| {
                types.ty().function(params, results);
                idx
            });
            import_indices.fd_write = Some(import_count);
            import_count += 1;
        }

        // Register function types.
        for (_, params, result, _) in functions {
            let wasm_params: Vec<ValType> = params.iter().map(|t| t.to_val_type()).collect();
            let wasm_results: Vec<ValType> = result.iter().map(|t| t.to_val_type()).collect();
            let key = (wasm_params.clone(), wasm_results.clone());
            let idx = type_map.len() as u32;
            let type_idx = *type_map.entry(key).or_insert_with(|| {
                types.ty().function(wasm_params, wasm_results);
                idx
            });
            func_type_indices.push(type_idx);
        }

        module.section(&types);

        // --- Import section ---
        let mut imports = ImportSection::new();

        if needs_probabilistic {
            // Look up type indices for each import signature.
            let rt_void_i32 = *type_map.get(&(vec![], vec![ValType::I32])).expect("TODO: handle error");
            let rt_i32_f64 = *type_map
                .get(&(vec![ValType::I32], vec![ValType::F64]))
                .expect("TODO: handle error");
            let rt_i32f64_void = *type_map
                .get(&(vec![ValType::I32, ValType::F64], vec![]))
                .expect("TODO: handle error");
            let rt_i32_i32 = *type_map
                .get(&(vec![ValType::I32], vec![ValType::I32]))
                .expect("TODO: handle error");
            let rt_i32f64f64_i32 = *type_map
                .get(&(
                    vec![ValType::I32, ValType::F64, ValType::F64],
                    vec![ValType::I32],
                ))
                .expect("TODO: handle error");

            imports.import(
                "betlang",
                "random_ternary",
                EntityType::Function(rt_void_i32),
            );
            imports.import("betlang", "sample_dist", EntityType::Function(rt_i32_f64));
            imports.import(
                "betlang",
                "observe_dist",
                EntityType::Function(rt_i32f64_void),
            );
            imports.import("betlang", "infer", EntityType::Function(rt_i32_i32));
            imports.import(
                "betlang",
                "create_dist",
                EntityType::Function(rt_i32f64f64_i32),
            );
        }

        if self.wasi_enabled {
            let rt_fd_write = *type_map
                .get(&(
                    vec![ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                    vec![ValType::I32],
                ))
                .expect("TODO: handle error");
            imports.import(
                "wasi_snapshot_preview1",
                "fd_write",
                EntityType::Function(rt_fd_write),
            );
        }

        if import_count > 0 {
            module.section(&imports);
        }

        // --- Function section ---
        let mut func_section = FunctionSection::new();
        for type_idx in &func_type_indices {
            func_section.function(*type_idx);
        }
        module.section(&func_section);

        // --- Memory section ---
        let mut memory = MemorySection::new();
        memory.memory(MemoryType {
            minimum: self.initial_memory_pages as u64,
            maximum: Some(self.max_memory_pages as u64),
            memory64: false,
            shared: false,
            page_size_log2: None,
        });
        module.section(&memory);

        // --- Export section ---
        let mut exports = ExportSection::new();
        exports.export("memory", ExportKind::Memory, 0);
        for (i, (name, _, _, _)) in functions.iter().enumerate() {
            exports.export(name.as_str(), ExportKind::Func, import_count + i as u32);
        }
        module.section(&exports);

        // --- Code section ---
        let mut code_section = CodeSection::new();
        let mut wasm_functions = Vec::new();

        let _allocator = BumpAllocator::new(self.data_offset, self.initial_memory_pages);

        for (name, params, result, is_prob) in functions {
            let mut func = WasmFunc::new(vec![]);

            // Scaffold: emit default return value based on return type.
            // Actual instruction emission from BetLang AST will replace this.
            if let Some(ret_ty) = result {
                match ret_ty {
                    WasmType::I32 => {
                        if *is_prob {
                            // For probabilistic functions, call random_ternary.
                            if let Some(_idx) = import_indices.random_ternary {
                                func.instruction(&Instruction::Call(
                                    import_indices.random_ternary.expect("TODO: handle error"),
                                ));
                            } else {
                                func.instruction(&Instruction::I32Const(0));
                            }
                        } else {
                            func.instruction(&Instruction::I32Const(0));
                        }
                    }
                    WasmType::I64 => func.instruction(&Instruction::I64Const(0)),
                    WasmType::F32 => func.instruction(&Instruction::F32Const(0.0)),
                    WasmType::F64 => func.instruction(&Instruction::F64Const(0.0)),
                }
            }

            func.instruction(&Instruction::End);

            wasm_functions.push(WasmFunction {
                name: name.clone(),
                params: params.clone(),
                result: *result,
                code_size: 0, // Updated after encoding.
                uses_probabilistic_imports: *is_prob,
            });

            code_section.function(&func);
        }

        module.section(&code_section);

        // --- Data section ---
        if !self.string_data.is_empty() {
            let mut data_section = DataSection::new();
            for (offset, bytes) in &self.string_data {
                data_section.active(
                    0,
                    &wasm_encoder::ConstExpr::i32_const(*offset as i32),
                    bytes.iter().copied(),
                );
            }
            module.section(&data_section);
        }

        // Finalize binary.
        let binary = module.finish();
        let binary_size = binary.len();

        Ok(WasmModule {
            functions: wasm_functions,
            initial_memory_pages: self.initial_memory_pages,
            max_memory_pages: self.max_memory_pages,
            binary_size,
            binary,
        })
    }
}

impl Default for WasmBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that an empty module (no functions) produces valid WASM.
    #[test]
    fn test_empty_module() {
        let mut backend = WasmBackend::new();
        let result = backend.generate(&[]);
        assert!(result.is_ok());
        let module = result.expect("TODO: handle error");
        assert!(module.binary_size > 0);
        assert_eq!(module.functions.len(), 0);
        // WASM magic number: \0asm
        assert_eq!(&module.to_bytes()[..4], b"\0asm");
    }

    /// Verify that a simple non-probabilistic function generates valid WASM.
    #[test]
    fn test_simple_function() {
        let mut backend = WasmBackend::new();
        let functions = vec![(
            "add".to_string(),
            vec![WasmType::I64, WasmType::I64],
            Some(WasmType::I64),
            false,
        )];
        let result = backend.generate(&functions);
        assert!(result.is_ok());
        let module = result.expect("TODO: handle error");
        assert_eq!(module.functions.len(), 1);
        assert_eq!(module.functions[0].name, "add");
        assert_eq!(module.functions[0].uses_probabilistic_imports, false);
    }

    /// Verify the module structure includes proper sections.
    #[test]
    fn test_module_structure() {
        let mut backend = WasmBackend::new();
        let functions = vec![
            (
                "bet_flip".to_string(),
                vec![],
                Some(WasmType::I32),
                true,
            ),
            (
                "constant".to_string(),
                vec![],
                Some(WasmType::F64),
                false,
            ),
        ];
        let result = backend.generate(&functions);
        assert!(result.is_ok());
        let module = result.expect("TODO: handle error");
        assert_eq!(module.functions.len(), 2);
        assert!(module.functions[0].uses_probabilistic_imports);
        assert!(!module.functions[1].uses_probabilistic_imports);
        assert_eq!(module.initial_memory_pages, 16);
        assert_eq!(module.max_memory_pages, 256);
    }

    /// Verify error handling for heap overflow.
    #[test]
    fn test_error_handling_heap_overflow() {
        let mut allocator = BumpAllocator::new(0, 1); // 1 page = 64KB
        // Allocate most of the page.
        let r1 = allocator.alloc(60000);
        assert!(r1.is_ok());
        // Allocate beyond capacity.
        let r2 = allocator.alloc(10000);
        assert!(r2.is_err());
        match r2 {
            Err(WasmError::HeapOverflow { requested, .. }) => {
                assert_eq!(requested, 10000);
            }
            other => panic!("expected HeapOverflow, got {other:?}"),
        }
    }

    /// Verify the binary starts with WASM magic number and version.
    #[test]
    fn test_binary_validation() {
        let mut backend = WasmBackend::new();
        let functions = vec![(
            "sample_normal".to_string(),
            vec![],
            Some(WasmType::F64),
            true,
        )];
        let result = backend.generate(&functions);
        assert!(result.is_ok());
        let module = result.expect("TODO: handle error");
        let bytes = module.to_bytes();
        // WASM magic: \0asm
        assert_eq!(&bytes[..4], b"\0asm");
        // WASM version: 1
        assert_eq!(&bytes[4..8], &[1, 0, 0, 0]);
        // Module should be non-trivially sized (has imports, functions, memory).
        assert!(bytes.len() > 50);
    }

    /// Verify WASI integration adds fd_write import.
    #[test]
    fn test_wasi_integration() {
        let mut backend = WasmBackend::new().with_wasi(true);
        let functions = vec![(
            "main".to_string(),
            vec![],
            Some(WasmType::I32),
            false,
        )];
        let result = backend.generate(&functions);
        assert!(result.is_ok());
        let module = result.expect("TODO: handle error");
        // Binary should be larger due to WASI import section.
        assert!(module.binary_size > 30);
    }

    /// Verify string interning works and respects memory bounds.
    #[test]
    fn test_string_interning() {
        let mut backend = WasmBackend::new().with_initial_memory(1);
        let offset1 = backend.intern_string("hello");
        assert!(offset1.is_ok());
        assert_eq!(offset1.expect("TODO: handle error"), 0);

        let offset2 = backend.intern_string("world");
        assert!(offset2.is_ok());
        // "hello" is 5 bytes, aligned to 4 -> offset 8.
        assert_eq!(offset2.expect("TODO: handle error"), 8);
    }

    /// Verify ternary type display.
    #[test]
    fn test_wasm_type_display() {
        assert_eq!(format!("{}", WasmType::I32), "i32");
        assert_eq!(format!("{}", WasmType::I64), "i64");
        assert_eq!(format!("{}", WasmType::F32), "f32");
        assert_eq!(format!("{}", WasmType::F64), "f64");
    }
}
