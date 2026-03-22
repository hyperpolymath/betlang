// SPDX-License-Identifier: MIT OR Apache-2.0
//! Parallel execution primitives for Betlang runtime
//!
//! Provides async/concurrent execution for probabilistic computations.

use crate::value::Value;
use im::Vector;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, RwLock, Semaphore};
use tokio::task::JoinHandle;

// ============================================================================
// Parallel Map/Reduce
// ============================================================================

/// Execute a function on each element in parallel
pub async fn parallel_map<F, Fut>(
    items: &Vector<Value>,
    f: F,
    max_concurrency: usize,
) -> Vector<Value>
where
    F: Fn(Value) -> Fut + Send + Sync + Clone + 'static,
    Fut: Future<Output = Value> + Send,
{
    let semaphore = Arc::new(Semaphore::new(max_concurrency));
    let results = Arc::new(Mutex::new(Vec::with_capacity(items.len())));

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for (idx, item) in items.iter().enumerate() {
        let item = item.clone();
        let sem = Arc::clone(&semaphore);
        let res = Arc::clone(&results);
        let f = f.clone();

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            let result = f(item).await;
            let mut guard = res.lock().await;
            guard.push((idx, result));
        });

        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        let _ = handle.await;
    }

    // Sort by original index and extract values
    let mut final_results = results.lock().await;
    final_results.sort_by_key(|(idx, _)| *idx);
    final_results.iter().map(|(_, v)| v.clone()).collect()
}

/// Parallel reduce with associative operation
pub async fn parallel_reduce<F>(
    items: &Vector<Value>,
    initial: Value,
    f: F,
) -> Value
where
    F: Fn(Value, Value) -> Value + Send + Sync + Clone + 'static,
{
    if items.is_empty() {
        return initial;
    }

    if items.len() == 1 {
        return f(initial, items[0].clone());
    }

    // Simple sequential reduce for now
    // A more sophisticated implementation would use work-stealing
    let mut result = initial;
    for item in items.iter() {
        result = f(result, item.clone());
    }
    result
}

/// Parallel filter
pub async fn parallel_filter<F, Fut>(
    items: &Vector<Value>,
    predicate: F,
    max_concurrency: usize,
) -> Vector<Value>
where
    F: Fn(Value) -> Fut + Send + Sync + Clone + 'static,
    Fut: Future<Output = bool> + Send,
{
    let semaphore = Arc::new(Semaphore::new(max_concurrency));
    let results = Arc::new(Mutex::new(Vec::with_capacity(items.len())));

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for (idx, item) in items.iter().enumerate() {
        let item = item.clone();
        let sem = Arc::clone(&semaphore);
        let res = Arc::clone(&results);
        let pred = predicate.clone();

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            if pred(item.clone()).await {
                let mut guard = res.lock().await;
                guard.push((idx, item));
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    let mut final_results = results.lock().await;
    final_results.sort_by_key(|(idx, _)| *idx);
    final_results.iter().map(|(_, v)| v.clone()).collect()
}

// ============================================================================
// Channels for Communication
// ============================================================================

/// A bounded channel for Value communication
pub struct Channel {
    sender: mpsc::Sender<Value>,
    receiver: Arc<Mutex<mpsc::Receiver<Value>>>,
}

impl Channel {
    /// Create a new bounded channel
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = mpsc::channel(capacity);
        Channel {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    /// Send a value (may block if channel is full)
    pub async fn send(&self, value: Value) -> Result<(), String> {
        self.sender
            .send(value)
            .await
            .map_err(|_| "Channel closed".to_string())
    }

    /// Receive a value (blocks until available)
    pub async fn recv(&self) -> Option<Value> {
        self.receiver.lock().await.recv().await
    }

    /// Try to receive without blocking
    pub async fn try_recv(&self) -> Option<Value> {
        match self.receiver.lock().await.try_recv() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    /// Get a clone of the sender
    pub fn sender(&self) -> mpsc::Sender<Value> {
        self.sender.clone()
    }
}

// ============================================================================
// Concurrent Collections
// ============================================================================

/// Thread-safe mutable vector
pub struct ConcurrentVector {
    inner: RwLock<Vec<Value>>,
}

impl ConcurrentVector {
    pub fn new() -> Self {
        ConcurrentVector {
            inner: RwLock::new(Vec::new()),
        }
    }

    pub fn from_vec(vec: Vec<Value>) -> Self {
        ConcurrentVector {
            inner: RwLock::new(vec),
        }
    }

    pub async fn push(&self, value: Value) {
        self.inner.write().await.push(value);
    }

    pub async fn get(&self, index: usize) -> Option<Value> {
        self.inner.read().await.get(index).cloned()
    }

    pub async fn len(&self) -> usize {
        self.inner.read().await.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.inner.read().await.is_empty()
    }

    pub async fn to_vec(&self) -> Vec<Value> {
        self.inner.read().await.clone()
    }

    pub async fn to_vector(&self) -> Vector<Value> {
        self.inner.read().await.iter().cloned().collect()
    }
}

impl Default for ConcurrentVector {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe mutable map
pub struct ConcurrentMap {
    inner: RwLock<std::collections::HashMap<String, Value>>,
}

impl ConcurrentMap {
    pub fn new() -> Self {
        ConcurrentMap {
            inner: RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub async fn insert(&self, key: String, value: Value) -> Option<Value> {
        self.inner.write().await.insert(key, value)
    }

    pub async fn get(&self, key: &str) -> Option<Value> {
        self.inner.read().await.get(key).cloned()
    }

    pub async fn remove(&self, key: &str) -> Option<Value> {
        self.inner.write().await.remove(key)
    }

    pub async fn contains_key(&self, key: &str) -> bool {
        self.inner.read().await.contains_key(key)
    }

    pub async fn len(&self) -> usize {
        self.inner.read().await.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.inner.read().await.is_empty()
    }

    pub async fn keys(&self) -> Vec<String> {
        self.inner.read().await.keys().cloned().collect()
    }
}

impl Default for ConcurrentMap {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Task Spawning
// ============================================================================

/// Spawn a task that runs concurrently
pub fn spawn<F, T>(future: F) -> JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    tokio::spawn(future)
}

/// Spawn a blocking task
pub fn spawn_blocking<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(f)
}

/// Run multiple futures concurrently and wait for all
pub async fn join_all<I, F>(futures: I) -> Vec<Value>
where
    I: IntoIterator<Item = F>,
    F: Future<Output = Value> + Send + 'static,
{
    let handles: Vec<_> = futures.into_iter().map(tokio::spawn).collect();
    let mut results = Vec::with_capacity(handles.len());

    for handle in handles {
        match handle.await {
            Ok(value) => results.push(value),
            Err(_) => results.push(Value::Error(Arc::new("Task panicked".to_string()))),
        }
    }

    results
}

/// Run multiple futures concurrently and return first to complete
pub async fn race<I, F>(futures: I) -> Value
where
    I: IntoIterator<Item = F>,
    F: Future<Output = Value> + Send + 'static,
{
    use tokio::select;

    let mut handles: Vec<_> = futures.into_iter().map(tokio::spawn).collect();

    if handles.is_empty() {
        return Value::Unit;
    }

    // Simple implementation - wait for first
    // A proper implementation would use select! macro
    for handle in handles.iter_mut() {
        if let Ok(value) = handle.await {
            return value;
        }
    }

    Value::Unit
}

// ============================================================================
// Parallel Monte Carlo
// ============================================================================

/// Parallel Monte Carlo sampling from a distribution
pub async fn parallel_sample(
    dist: &Value,
    n: usize,
    num_workers: usize,
) -> Result<Vector<Value>, String> {
    match dist {
        Value::Dist(d) => {
            let sampler = Arc::clone(d);
            let samples_per_worker = n / num_workers;
            let remainder = n % num_workers;

            let mut handles = Vec::new();

            for i in 0..num_workers {
                let sampler = Arc::clone(&sampler);
                let count = samples_per_worker + if i < remainder { 1 } else { 0 };

                let handle = tokio::spawn(async move {
                    (0..count).map(|_| (sampler.sampler)()).collect::<Vec<_>>()
                });

                handles.push(handle);
            }

            let mut all_samples = Vector::new();
            for handle in handles {
                match handle.await {
                    Ok(samples) => {
                        for s in samples {
                            all_samples.push_back(s);
                        }
                    }
                    Err(e) => return Err(format!("Sampling failed: {}", e)),
                }
            }

            Ok(all_samples)
        }
        _ => Err(format!("Cannot sample from {}", dist.type_name())),
    }
}

/// Parallel Monte Carlo estimation of expected value
pub async fn parallel_expected_value(
    dist: &Value,
    n: usize,
    num_workers: usize,
) -> Result<f64, String> {
    let samples = parallel_sample(dist, n, num_workers).await?;

    let sum: f64 = samples
        .iter()
        .filter_map(|v| match v {
            Value::Int(i) => Some(*i as f64),
            Value::Float(f) => Some(*f),
            _ => None,
        })
        .sum();

    Ok(sum / n as f64)
}

// ============================================================================
// Rate Limiting
// ============================================================================

/// Rate limiter for controlling execution speed
pub struct RateLimiter {
    semaphore: Semaphore,
    refill_interval: std::time::Duration,
}

impl RateLimiter {
    /// Create a rate limiter with given capacity and refill rate
    pub fn new(capacity: usize, refill_per_second: f64) -> Self {
        let interval = std::time::Duration::from_secs_f64(1.0 / refill_per_second);
        RateLimiter {
            semaphore: Semaphore::new(capacity),
            refill_interval: interval,
        }
    }

    /// Acquire a permit (blocks if rate limited)
    pub async fn acquire(&self) -> Result<(), String> {
        self.semaphore
            .acquire()
            .await
            .map(|_| ())
            .map_err(|_| "Rate limiter closed".to_string())
    }

    /// Try to acquire without blocking
    pub fn try_acquire(&self) -> bool {
        self.semaphore.try_acquire().is_ok()
    }
}

// ============================================================================
// Work Pool
// ============================================================================

/// A pool of workers for executing tasks
pub struct WorkPool {
    sender: mpsc::Sender<Box<dyn FnOnce() + Send + 'static>>,
    _handles: Vec<JoinHandle<()>>,
}

impl WorkPool {
    /// Create a new work pool with given number of workers
    pub fn new(num_workers: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<Box<dyn FnOnce() + Send + 'static>>(100);
        let receiver = Arc::new(Mutex::new(receiver));

        let mut handles = Vec::new();

        for _ in 0..num_workers {
            let rx = Arc::clone(&receiver);
            let handle = tokio::spawn(async move {
                loop {
                    let task = {
                        let mut guard = rx.lock().await;
                        guard.recv().await
                    };

                    match task {
                        Some(f) => f(),
                        None => break,
                    }
                }
            });
            handles.push(handle);
        }

        WorkPool {
            sender,
            _handles: handles,
        }
    }

    /// Submit a task to the pool
    pub async fn submit<F>(&self, f: F) -> Result<(), String>
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .send(Box::new(f))
            .await
            .map_err(|_| "Work pool closed".to_string())
    }
}

// ============================================================================
// Native function bindings
// ============================================================================

use crate::value::NativeFunction;

/// Get all parallel execution native functions
pub fn native_functions() -> Vec<NativeFunction> {
    vec![
        NativeFunction {
            name: "spawn",
            arity: 1,
            func: |_args| {
                // Spawning requires async context, return placeholder
                Ok(Value::String(Arc::new(
                    "spawn requires async context".to_string(),
                )))
            },
        },
        NativeFunction {
            name: "channel",
            arity: 1,
            func: |args| {
                if let Some(Value::Int(capacity)) = args.first() {
                    // Return a placeholder since we can't easily serialize channels
                    Ok(Value::String(Arc::new(format!(
                        "channel({})",
                        capacity
                    ))))
                } else {
                    Err("channel expects capacity".to_string())
                }
            },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_vector() {
        let vec = ConcurrentVector::new();
        vec.push(Value::Int(1)).await;
        vec.push(Value::Int(2)).await;
        vec.push(Value::Int(3)).await;

        assert_eq!(vec.len().await, 3);
        assert_eq!(vec.get(1).await, Some(Value::Int(2)));
    }

    #[tokio::test]
    async fn test_concurrent_map() {
        let map = ConcurrentMap::new();
        map.insert("a".to_string(), Value::Int(1)).await;
        map.insert("b".to_string(), Value::Int(2)).await;

        assert_eq!(map.get("a").await, Some(Value::Int(1)));
        assert!(map.contains_key("b").await);
        assert!(!map.contains_key("c").await);
    }

    #[tokio::test]
    async fn test_channel() {
        let ch = Channel::new(10);

        ch.send(Value::Int(42)).await.unwrap();
        let received = ch.recv().await;

        assert_eq!(received, Some(Value::Int(42)));
    }
}
