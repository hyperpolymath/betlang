// SPDX-License-Identifier: MIT OR Apache-2.0
//! Data structures for Betlang runtime
//!
//! Provides persistent (immutable) data structures optimized for functional programming.

use crate::value::Value;
use im::{HashMap, HashSet, Vector};
use std::cmp::Ordering;
use std::sync::Arc;

// ============================================================================
// Persistent Vector (List)
// ============================================================================

/// Persistent vector operations
pub mod list {
    use super::*;

    /// Create empty list
    pub fn empty() -> Vector<Value> {
        Vector::new()
    }

    /// Create list from values
    pub fn of(values: Vec<Value>) -> Vector<Value> {
        values.into_iter().collect()
    }

    /// Create list with single element
    pub fn singleton(value: Value) -> Vector<Value> {
        Vector::unit(value)
    }

    /// Create list by repeating value n times
    pub fn repeat(value: Value, n: usize) -> Vector<Value> {
        std::iter::repeat(value).take(n).collect()
    }

    /// Create range [start, end)
    pub fn range(start: i64, end: i64) -> Vector<Value> {
        (start..end).map(Value::Int).collect()
    }

    /// Create range [start, end] with step
    pub fn range_step(start: i64, end: i64, step: i64) -> Vector<Value> {
        let mut result = Vector::new();
        let mut current = start;
        if step > 0 {
            while current <= end {
                result.push_back(Value::Int(current));
                current += step;
            }
        } else if step < 0 {
            while current >= end {
                result.push_back(Value::Int(current));
                current += step;
            }
        }
        result
    }

    /// Get length
    pub fn len(list: &Vector<Value>) -> usize {
        list.len()
    }

    /// Check if empty
    pub fn is_empty(list: &Vector<Value>) -> bool {
        list.is_empty()
    }

    /// Get element at index
    pub fn get(list: &Vector<Value>, index: usize) -> Option<&Value> {
        list.get(index)
    }

    /// Get first element
    pub fn head(list: &Vector<Value>) -> Option<&Value> {
        list.head()
    }

    /// Get last element
    pub fn last(list: &Vector<Value>) -> Option<&Value> {
        list.last()
    }

    /// Get all elements except first
    pub fn tail(list: &Vector<Value>) -> Vector<Value> {
        if list.is_empty() {
            Vector::new()
        } else {
            list.skip(1)
        }
    }

    /// Get all elements except last
    pub fn init(list: &Vector<Value>) -> Vector<Value> {
        if list.is_empty() {
            Vector::new()
        } else {
            list.take(list.len() - 1)
        }
    }

    /// Append element to end
    pub fn push(list: &Vector<Value>, value: Value) -> Vector<Value> {
        let mut new_list = list.clone();
        new_list.push_back(value);
        new_list
    }

    /// Prepend element to front
    pub fn cons(value: Value, list: &Vector<Value>) -> Vector<Value> {
        let mut new_list = list.clone();
        new_list.push_front(value);
        new_list
    }

    /// Concatenate two lists
    pub fn concat(a: &Vector<Value>, b: &Vector<Value>) -> Vector<Value> {
        let mut result = a.clone();
        result.append(b.clone());
        result
    }

    /// Take first n elements
    pub fn take(list: &Vector<Value>, n: usize) -> Vector<Value> {
        list.take(n)
    }

    /// Drop first n elements
    pub fn drop(list: &Vector<Value>, n: usize) -> Vector<Value> {
        list.skip(n)
    }

    /// Reverse list
    pub fn reverse(list: &Vector<Value>) -> Vector<Value> {
        list.iter().cloned().rev().collect()
    }

    /// Flatten nested lists
    pub fn flatten(list: &Vector<Value>) -> Vector<Value> {
        let mut result = Vector::new();
        for item in list.iter() {
            if let Value::List(inner) = item {
                result.append(inner.clone());
            } else {
                result.push_back(item.clone());
            }
        }
        result
    }

    /// Zip two lists into list of tuples
    pub fn zip(a: &Vector<Value>, b: &Vector<Value>) -> Vector<Value> {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| Value::Tuple(Arc::new(vec![x.clone(), y.clone()])))
            .collect()
    }

    /// Unzip list of tuples into two lists
    pub fn unzip(list: &Vector<Value>) -> (Vector<Value>, Vector<Value>) {
        let mut a = Vector::new();
        let mut b = Vector::new();
        for item in list.iter() {
            if let Value::Tuple(t) = item {
                if t.len() >= 2 {
                    a.push_back(t[0].clone());
                    b.push_back(t[1].clone());
                }
            }
        }
        (a, b)
    }

    /// Check if list contains value
    pub fn contains(list: &Vector<Value>, value: &Value) -> bool {
        list.iter().any(|v| v == value)
    }

    /// Find index of value
    pub fn index_of(list: &Vector<Value>, value: &Value) -> Option<usize> {
        list.iter().position(|v| v == value)
    }

    /// Count occurrences of value
    pub fn count(list: &Vector<Value>, value: &Value) -> usize {
        list.iter().filter(|v| *v == value).count()
    }

    /// Remove duplicates (preserving order)
    pub fn distinct(list: &Vector<Value>) -> Vector<Value> {
        let mut seen = HashSet::new();
        let mut result = Vector::new();
        for item in list.iter() {
            if !seen.contains(item) {
                seen.insert(item.clone());
                result.push_back(item.clone());
            }
        }
        result
    }

    /// Slice [start, end)
    pub fn slice(list: &Vector<Value>, start: usize, end: usize) -> Vector<Value> {
        list.skip(start).take(end.saturating_sub(start))
    }

    /// Intersperse separator between elements
    pub fn intersperse(list: &Vector<Value>, sep: Value) -> Vector<Value> {
        let mut result = Vector::new();
        for (i, item) in list.iter().enumerate() {
            if i > 0 {
                result.push_back(sep.clone());
            }
            result.push_back(item.clone());
        }
        result
    }

    /// Split list at index
    pub fn split_at(list: &Vector<Value>, index: usize) -> (Vector<Value>, Vector<Value>) {
        (list.take(index), list.skip(index))
    }

    /// Group consecutive elements by predicate
    pub fn group_by<F>(list: &Vector<Value>, eq: F) -> Vector<Value>
    where
        F: Fn(&Value, &Value) -> bool,
    {
        let mut result = Vector::new();
        let mut current_group = Vector::new();

        for item in list.iter() {
            if current_group.is_empty() {
                current_group.push_back(item.clone());
            } else if eq(current_group.last().unwrap(), item) {
                current_group.push_back(item.clone());
            } else {
                result.push_back(Value::List(current_group));
                current_group = Vector::unit(item.clone());
            }
        }

        if !current_group.is_empty() {
            result.push_back(Value::List(current_group));
        }

        result
    }
}

// ============================================================================
// Persistent HashMap (Map)
// ============================================================================

/// Persistent map operations
pub mod map {
    use super::*;

    /// Create empty map
    pub fn empty() -> HashMap<String, Value> {
        HashMap::new()
    }

    /// Create map from key-value pairs
    pub fn of(pairs: Vec<(String, Value)>) -> HashMap<String, Value> {
        pairs.into_iter().collect()
    }

    /// Create map with single entry
    pub fn singleton(key: String, value: Value) -> HashMap<String, Value> {
        HashMap::unit(key, value)
    }

    /// Get value by key
    pub fn get<'a>(map: &'a HashMap<String, Value>, key: &str) -> Option<&'a Value> {
        map.get(key)
    }

    /// Get value with default
    pub fn get_or(map: &HashMap<String, Value>, key: &str, default: Value) -> Value {
        map.get(key).cloned().unwrap_or(default)
    }

    /// Check if key exists
    pub fn contains_key(map: &HashMap<String, Value>, key: &str) -> bool {
        map.contains_key(key)
    }

    /// Insert key-value pair
    pub fn insert(map: &HashMap<String, Value>, key: String, value: Value) -> HashMap<String, Value> {
        map.update(key, value)
    }

    /// Remove key
    pub fn remove(map: &HashMap<String, Value>, key: &str) -> HashMap<String, Value> {
        map.without(key)
    }

    /// Get number of entries
    pub fn len(map: &HashMap<String, Value>) -> usize {
        map.len()
    }

    /// Check if empty
    pub fn is_empty(map: &HashMap<String, Value>) -> bool {
        map.is_empty()
    }

    /// Get all keys
    pub fn keys(map: &HashMap<String, Value>) -> Vector<Value> {
        map.keys()
            .map(|k| Value::String(Arc::new(k.clone())))
            .collect()
    }

    /// Get all values
    pub fn values(map: &HashMap<String, Value>) -> Vector<Value> {
        map.values().cloned().collect()
    }

    /// Get all key-value pairs as tuples
    pub fn entries(map: &HashMap<String, Value>) -> Vector<Value> {
        map.iter()
            .map(|(k, v)| {
                Value::Tuple(Arc::new(vec![
                    Value::String(Arc::new(k.clone())),
                    v.clone(),
                ]))
            })
            .collect()
    }

    /// Merge two maps (second overwrites first on conflicts)
    pub fn merge(a: &HashMap<String, Value>, b: &HashMap<String, Value>) -> HashMap<String, Value> {
        a.clone().union(b.clone())
    }

    /// Filter map by predicate on keys
    pub fn filter_keys<F>(map: &HashMap<String, Value>, pred: F) -> HashMap<String, Value>
    where
        F: Fn(&str) -> bool,
    {
        map.iter()
            .filter(|(k, _)| pred(k))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Select subset of keys
    pub fn select(map: &HashMap<String, Value>, keys: &[&str]) -> HashMap<String, Value> {
        keys.iter()
            .filter_map(|k| map.get(*k).map(|v| ((*k).to_string(), v.clone())))
            .collect()
    }

    /// Omit subset of keys
    pub fn omit(map: &HashMap<String, Value>, keys: &[&str]) -> HashMap<String, Value> {
        let key_set: std::collections::HashSet<_> = keys.iter().collect();
        map.iter()
            .filter(|(k, _)| !key_set.contains(&k.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Convert map to Value::Map
    pub fn to_value(map: HashMap<String, Value>) -> Value {
        Value::Map(map)
    }
}

// ============================================================================
// Persistent HashSet (Set)
// ============================================================================

/// Persistent set operations
pub mod set {
    use super::*;

    /// Create empty set
    pub fn empty() -> HashSet<Value> {
        HashSet::new()
    }

    /// Create set from values
    pub fn of(values: Vec<Value>) -> HashSet<Value> {
        values.into_iter().collect()
    }

    /// Create set with single element
    pub fn singleton(value: Value) -> HashSet<Value> {
        HashSet::unit(value)
    }

    /// Check if value is in set
    pub fn contains(set: &HashSet<Value>, value: &Value) -> bool {
        set.contains(value)
    }

    /// Insert value
    pub fn insert(set: &HashSet<Value>, value: Value) -> HashSet<Value> {
        set.update(value)
    }

    /// Remove value
    pub fn remove(set: &HashSet<Value>, value: &Value) -> HashSet<Value> {
        set.without(value)
    }

    /// Get size
    pub fn len(set: &HashSet<Value>) -> usize {
        set.len()
    }

    /// Check if empty
    pub fn is_empty(set: &HashSet<Value>) -> bool {
        set.is_empty()
    }

    /// Union of two sets
    pub fn union(a: &HashSet<Value>, b: &HashSet<Value>) -> HashSet<Value> {
        a.clone().union(b.clone())
    }

    /// Intersection of two sets
    pub fn intersection(a: &HashSet<Value>, b: &HashSet<Value>) -> HashSet<Value> {
        a.clone().intersection(b.clone())
    }

    /// Difference (a - b)
    pub fn difference(a: &HashSet<Value>, b: &HashSet<Value>) -> HashSet<Value> {
        a.clone().difference(b.clone())
    }

    /// Symmetric difference (elements in a or b but not both)
    pub fn symmetric_difference(a: &HashSet<Value>, b: &HashSet<Value>) -> HashSet<Value> {
        a.clone().symmetric_difference(b.clone())
    }

    /// Check if a is subset of b
    pub fn is_subset(a: &HashSet<Value>, b: &HashSet<Value>) -> bool {
        a.is_subset(b)
    }

    /// Check if a is superset of b
    pub fn is_superset(a: &HashSet<Value>, b: &HashSet<Value>) -> bool {
        b.is_subset(a)
    }

    /// Check if sets are disjoint (no common elements)
    pub fn is_disjoint(a: &HashSet<Value>, b: &HashSet<Value>) -> bool {
        a.clone().intersection(b.clone()).is_empty()
    }

    /// Convert set to list
    pub fn to_list(set: &HashSet<Value>) -> Vector<Value> {
        set.iter().cloned().collect()
    }

    /// Create set from list
    pub fn from_list(list: &Vector<Value>) -> HashSet<Value> {
        list.iter().cloned().collect()
    }
}

// ============================================================================
// Tuple Operations
// ============================================================================

/// Tuple operations
pub mod tuple {
    use super::*;

    /// Create tuple from values
    pub fn of(values: Vec<Value>) -> Value {
        Value::Tuple(Arc::new(values))
    }

    /// Create pair (2-tuple)
    pub fn pair(a: Value, b: Value) -> Value {
        Value::Tuple(Arc::new(vec![a, b]))
    }

    /// Create triple (3-tuple)
    pub fn triple(a: Value, b: Value, c: Value) -> Value {
        Value::Tuple(Arc::new(vec![a, b, c]))
    }

    /// Get first element of pair
    pub fn fst(tuple: &Value) -> Option<Value> {
        if let Value::Tuple(t) = tuple {
            t.first().cloned()
        } else {
            None
        }
    }

    /// Get second element of pair
    pub fn snd(tuple: &Value) -> Option<Value> {
        if let Value::Tuple(t) = tuple {
            t.get(1).cloned()
        } else {
            None
        }
    }

    /// Get third element of triple
    pub fn thd(tuple: &Value) -> Option<Value> {
        if let Value::Tuple(t) = tuple {
            t.get(2).cloned()
        } else {
            None
        }
    }

    /// Get element at index
    pub fn get(tuple: &Value, index: usize) -> Option<Value> {
        if let Value::Tuple(t) = tuple {
            t.get(index).cloned()
        } else {
            None
        }
    }

    /// Get tuple length
    pub fn len(tuple: &Value) -> Option<usize> {
        if let Value::Tuple(t) = tuple {
            Some(t.len())
        } else {
            None
        }
    }

    /// Convert tuple to list
    pub fn to_list(tuple: &Value) -> Option<Vector<Value>> {
        if let Value::Tuple(t) = tuple {
            Some(t.iter().cloned().collect())
        } else {
            None
        }
    }

    /// Swap pair elements
    pub fn swap(tuple: &Value) -> Option<Value> {
        if let Value::Tuple(t) = tuple {
            if t.len() == 2 {
                Some(Value::Tuple(Arc::new(vec![t[1].clone(), t[0].clone()])))
            } else {
                None
            }
        } else {
            None
        }
    }
}

// ============================================================================
// Queue (Persistent FIFO)
// ============================================================================

/// Persistent queue (FIFO) operations
pub mod queue {
    use super::*;

    /// Queue is implemented as a pair of lists for amortized O(1) operations
    #[derive(Clone, Debug)]
    pub struct Queue {
        front: Vector<Value>,
        back: Vector<Value>,
    }

    impl Queue {
        /// Create empty queue
        pub fn new() -> Self {
            Queue {
                front: Vector::new(),
                back: Vector::new(),
            }
        }

        /// Create queue from list
        pub fn from_list(list: Vector<Value>) -> Self {
            Queue {
                front: list,
                back: Vector::new(),
            }
        }

        /// Check if empty
        pub fn is_empty(&self) -> bool {
            self.front.is_empty() && self.back.is_empty()
        }

        /// Get length
        pub fn len(&self) -> usize {
            self.front.len() + self.back.len()
        }

        /// Enqueue (add to back)
        pub fn enqueue(&self, value: Value) -> Self {
            let mut new_back = self.back.clone();
            new_back.push_back(value);
            Queue {
                front: self.front.clone(),
                back: new_back,
            }
        }

        /// Dequeue (remove from front)
        pub fn dequeue(&self) -> Option<(Value, Self)> {
            if self.front.is_empty() {
                if self.back.is_empty() {
                    return None;
                }
                // Reverse back into front
                let new_front: Vector<Value> = self.back.iter().cloned().rev().collect();
                let value = new_front.head()?.clone();
                Some((
                    value,
                    Queue {
                        front: new_front.skip(1),
                        back: Vector::new(),
                    },
                ))
            } else {
                let value = self.front.head()?.clone();
                Some((
                    value,
                    Queue {
                        front: self.front.skip(1),
                        back: self.back.clone(),
                    },
                ))
            }
        }

        /// Peek at front without removing
        pub fn peek(&self) -> Option<Value> {
            if self.front.is_empty() {
                self.back.last().cloned()
            } else {
                self.front.head().cloned()
            }
        }

        /// Convert to list
        pub fn to_list(&self) -> Vector<Value> {
            let mut result = self.front.clone();
            for item in self.back.iter().rev() {
                result.push_back(item.clone());
            }
            result
        }
    }

    impl Default for Queue {
        fn default() -> Self {
            Self::new()
        }
    }
}

// ============================================================================
// Stack (Persistent LIFO)
// ============================================================================

/// Persistent stack (LIFO) operations
pub mod stack {
    use super::*;

    /// Stack is just a wrapper around Vector for clarity
    #[derive(Clone, Debug)]
    pub struct Stack(Vector<Value>);

    impl Stack {
        /// Create empty stack
        pub fn new() -> Self {
            Stack(Vector::new())
        }

        /// Create stack from list (top is first element)
        pub fn from_list(list: Vector<Value>) -> Self {
            Stack(list)
        }

        /// Check if empty
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        /// Get length
        pub fn len(&self) -> usize {
            self.0.len()
        }

        /// Push onto stack
        pub fn push(&self, value: Value) -> Self {
            let mut new_stack = self.0.clone();
            new_stack.push_front(value);
            Stack(new_stack)
        }

        /// Pop from stack
        pub fn pop(&self) -> Option<(Value, Self)> {
            if self.0.is_empty() {
                None
            } else {
                let value = self.0.head()?.clone();
                Some((value, Stack(self.0.skip(1))))
            }
        }

        /// Peek at top without removing
        pub fn peek(&self) -> Option<Value> {
            self.0.head().cloned()
        }

        /// Convert to list
        pub fn to_list(&self) -> Vector<Value> {
            self.0.clone()
        }
    }

    impl Default for Stack {
        fn default() -> Self {
            Self::new()
        }
    }
}

// ============================================================================
// Sorting utilities
// ============================================================================

/// Sorting utilities for lists
pub mod sort {
    use super::*;

    /// Sort list of comparable values
    pub fn sort(list: &Vector<Value>) -> Vector<Value> {
        let mut vec: Vec<Value> = list.iter().cloned().collect();
        vec.sort_by(compare_values);
        vec.into_iter().collect()
    }

    /// Sort list in descending order
    pub fn sort_desc(list: &Vector<Value>) -> Vector<Value> {
        let mut vec: Vec<Value> = list.iter().cloned().collect();
        vec.sort_by(|a, b| compare_values(b, a));
        vec.into_iter().collect()
    }

    /// Compare two values for ordering
    pub fn compare_values(a: &Value, b: &Value) -> Ordering {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => x.cmp(y),
            (Value::Float(x), Value::Float(y)) => x.partial_cmp(y).unwrap_or(Ordering::Equal),
            (Value::String(x), Value::String(y)) => x.cmp(y),
            (Value::Bool(x), Value::Bool(y)) => x.cmp(y),
            // Different types: compare by type name
            _ => a.type_name().cmp(b.type_name()),
        }
    }

    /// Check if list is sorted
    pub fn is_sorted(list: &Vector<Value>) -> bool {
        list.iter()
            .zip(list.iter().skip(1))
            .all(|(a, b)| compare_values(a, b) != Ordering::Greater)
    }

    /// Get minimum value
    pub fn min(list: &Vector<Value>) -> Option<Value> {
        list.iter().min_by(|a, b| compare_values(a, b)).cloned()
    }

    /// Get maximum value
    pub fn max(list: &Vector<Value>) -> Option<Value> {
        list.iter().max_by(|a, b| compare_values(a, b)).cloned()
    }
}

// ============================================================================
// Native function bindings
// ============================================================================

use crate::value::NativeFunction;

/// Get all data structure native functions
pub fn native_functions() -> Vec<NativeFunction> {
    vec![
        NativeFunction {
            name: "list_len",
            arity: 1,
            func: |args| {
                if let Some(Value::List(l)) = args.first() {
                    Ok(Value::Int(l.len() as i64))
                } else {
                    Err("list_len expects a list".to_string())
                }
            },
        },
        NativeFunction {
            name: "list_head",
            arity: 1,
            func: |args| {
                if let Some(Value::List(l)) = args.first() {
                    Ok(l.head().cloned().unwrap_or(Value::Unit))
                } else {
                    Err("list_head expects a list".to_string())
                }
            },
        },
        NativeFunction {
            name: "list_tail",
            arity: 1,
            func: |args| {
                if let Some(Value::List(l)) = args.first() {
                    Ok(Value::List(list::tail(l)))
                } else {
                    Err("list_tail expects a list".to_string())
                }
            },
        },
        NativeFunction {
            name: "list_reverse",
            arity: 1,
            func: |args| {
                if let Some(Value::List(l)) = args.first() {
                    Ok(Value::List(list::reverse(l)))
                } else {
                    Err("list_reverse expects a list".to_string())
                }
            },
        },
        NativeFunction {
            name: "list_sort",
            arity: 1,
            func: |args| {
                if let Some(Value::List(l)) = args.first() {
                    Ok(Value::List(sort::sort(l)))
                } else {
                    Err("list_sort expects a list".to_string())
                }
            },
        },
        NativeFunction {
            name: "map_get",
            arity: 2,
            func: |args| {
                if args.len() >= 2 {
                    if let (Value::Map(m), Value::String(k)) = (&args[0], &args[1]) {
                        Ok(m.get(k.as_str()).cloned().unwrap_or(Value::Unit))
                    } else {
                        Err("map_get expects (map, string)".to_string())
                    }
                } else {
                    Err("map_get expects 2 arguments".to_string())
                }
            },
        },
        NativeFunction {
            name: "map_keys",
            arity: 1,
            func: |args| {
                if let Some(Value::Map(m)) = args.first() {
                    Ok(Value::List(map::keys(m)))
                } else {
                    Err("map_keys expects a map".to_string())
                }
            },
        },
        NativeFunction {
            name: "map_values",
            arity: 1,
            func: |args| {
                if let Some(Value::Map(m)) = args.first() {
                    Ok(Value::List(map::values(m)))
                } else {
                    Err("map_values expects a map".to_string())
                }
            },
        },
        NativeFunction {
            name: "set_contains",
            arity: 2,
            func: |args| {
                if args.len() >= 2 {
                    if let Value::Set(s) = &args[0] {
                        let key_map: HashMap<Value, ()> = s.clone();
                        Ok(Value::Bool(key_map.contains_key(&args[1])))
                    } else {
                        Err("set_contains expects (set, value)".to_string())
                    }
                } else {
                    Err("set_contains expects 2 arguments".to_string())
                }
            },
        },
        NativeFunction {
            name: "fst",
            arity: 1,
            func: |args| {
                if let Some(t) = args.first() {
                    Ok(tuple::fst(t).unwrap_or(Value::Unit))
                } else {
                    Err("fst expects a tuple".to_string())
                }
            },
        },
        NativeFunction {
            name: "snd",
            arity: 1,
            func: |args| {
                if let Some(t) = args.first() {
                    Ok(tuple::snd(t).unwrap_or(Value::Unit))
                } else {
                    Err("snd expects a tuple".to_string())
                }
            },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_operations() {
        let list = list::of(vec![Value::Int(3), Value::Int(1), Value::Int(2)]);
        assert_eq!(list::len(&list), 3);
        assert_eq!(list::head(&list), Some(&Value::Int(3)));

        let sorted = sort::sort(&list);
        assert_eq!(sorted.head(), Some(&Value::Int(1)));
    }

    #[test]
    fn test_map_operations() {
        let m = map::of(vec![
            ("a".to_string(), Value::Int(1)),
            ("b".to_string(), Value::Int(2)),
        ]);
        assert_eq!(map::get(&m, "a"), Some(&Value::Int(1)));
        assert!(map::contains_key(&m, "b"));
        assert!(!map::contains_key(&m, "c"));
    }

    #[test]
    fn test_set_operations() {
        let s1 = set::of(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        let s2 = set::of(vec![Value::Int(2), Value::Int(3), Value::Int(4)]);

        let union = set::union(&s1, &s2);
        assert_eq!(set::len(&union), 4);

        let intersection = set::intersection(&s1, &s2);
        assert_eq!(set::len(&intersection), 2);
    }

    #[test]
    fn test_queue() {
        let q = queue::Queue::new()
            .enqueue(Value::Int(1))
            .enqueue(Value::Int(2))
            .enqueue(Value::Int(3));

        let (v1, q) = q.dequeue().unwrap();
        assert_eq!(v1, Value::Int(1));

        let (v2, _) = q.dequeue().unwrap();
        assert_eq!(v2, Value::Int(2));
    }

    #[test]
    fn test_stack() {
        let s = stack::Stack::new()
            .push(Value::Int(1))
            .push(Value::Int(2))
            .push(Value::Int(3));

        let (v1, s) = s.pop().unwrap();
        assert_eq!(v1, Value::Int(3));

        let (v2, _) = s.pop().unwrap();
        assert_eq!(v2, Value::Int(2));
    }
}
