// SPDX-License-Identifier: MIT OR Apache-2.0
//! I/O operations for Betlang runtime
//!
//! Provides file, network, and stdio operations with async support.

use crate::value::{FileHandle, FileMode, Value};
use std::collections::HashMap;
use std::io::{self, BufRead, Write as IoWrite};
use std::path::Path;
use std::sync::Arc;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::Mutex;

/// Result type for I/O operations
pub type IoResult<T> = Result<T, IoError>;

/// I/O error types
#[derive(Debug, Clone)]
pub enum IoError {
    NotFound(String),
    PermissionDenied(String),
    AlreadyExists(String),
    ConnectionRefused(String),
    ConnectionReset(String),
    Timeout(String),
    InvalidData(String),
    Other(String),
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoError::NotFound(s) => write!(f, "Not found: {}", s),
            IoError::PermissionDenied(s) => write!(f, "Permission denied: {}", s),
            IoError::AlreadyExists(s) => write!(f, "Already exists: {}", s),
            IoError::ConnectionRefused(s) => write!(f, "Connection refused: {}", s),
            IoError::ConnectionReset(s) => write!(f, "Connection reset: {}", s),
            IoError::Timeout(s) => write!(f, "Timeout: {}", s),
            IoError::InvalidData(s) => write!(f, "Invalid data: {}", s),
            IoError::Other(s) => write!(f, "I/O error: {}", s),
        }
    }
}

impl std::error::Error for IoError {}

impl From<std::io::Error> for IoError {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind;
        match err.kind() {
            ErrorKind::NotFound => IoError::NotFound(err.to_string()),
            ErrorKind::PermissionDenied => IoError::PermissionDenied(err.to_string()),
            ErrorKind::AlreadyExists => IoError::AlreadyExists(err.to_string()),
            ErrorKind::ConnectionRefused => IoError::ConnectionRefused(err.to_string()),
            ErrorKind::ConnectionReset => IoError::ConnectionReset(err.to_string()),
            ErrorKind::TimedOut => IoError::Timeout(err.to_string()),
            ErrorKind::InvalidData => IoError::InvalidData(err.to_string()),
            _ => IoError::Other(err.to_string()),
        }
    }
}

// ============================================================================
// File Operations
// ============================================================================

/// File system operations
pub mod file {
    use super::*;

    /// Read entire file as string
    pub async fn read_string<P: AsRef<Path>>(path: P) -> IoResult<String> {
        fs::read_to_string(path).await.map_err(Into::into)
    }

    /// Read entire file as bytes
    pub async fn read_bytes<P: AsRef<Path>>(path: P) -> IoResult<Vec<u8>> {
        fs::read(path).await.map_err(Into::into)
    }

    /// Write string to file
    pub async fn write_string<P: AsRef<Path>>(path: P, contents: &str) -> IoResult<()> {
        fs::write(path, contents).await.map_err(Into::into)
    }

    /// Write bytes to file
    pub async fn write_bytes<P: AsRef<Path>>(path: P, contents: &[u8]) -> IoResult<()> {
        fs::write(path, contents).await.map_err(Into::into)
    }

    /// Append string to file
    pub async fn append_string<P: AsRef<Path>>(path: P, contents: &str) -> IoResult<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .await?;
        file.write_all(contents.as_bytes()).await.map_err(Into::into)
    }

    /// Check if file exists
    pub async fn exists<P: AsRef<Path>>(path: P) -> bool {
        fs::metadata(path).await.is_ok()
    }

    /// Get file metadata
    pub async fn metadata<P: AsRef<Path>>(path: P) -> IoResult<FileMetadata> {
        let meta = fs::metadata(path).await?;
        Ok(FileMetadata {
            size: meta.len(),
            is_file: meta.is_file(),
            is_dir: meta.is_dir(),
            is_symlink: meta.is_symlink(),
            readonly: meta.permissions().readonly(),
        })
    }

    /// Remove file
    pub async fn remove<P: AsRef<Path>>(path: P) -> IoResult<()> {
        fs::remove_file(path).await.map_err(Into::into)
    }

    /// Copy file
    pub async fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> IoResult<u64> {
        fs::copy(from, to).await.map_err(Into::into)
    }

    /// Rename/move file
    pub async fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> IoResult<()> {
        fs::rename(from, to).await.map_err(Into::into)
    }

    /// Read file line by line (returns iterator-like structure)
    pub async fn read_lines<P: AsRef<Path>>(path: P) -> IoResult<Vec<String>> {
        let contents = fs::read_to_string(path).await?;
        Ok(contents.lines().map(String::from).collect())
    }

    /// File metadata
    #[derive(Debug, Clone)]
    pub struct FileMetadata {
        pub size: u64,
        pub is_file: bool,
        pub is_dir: bool,
        pub is_symlink: bool,
        pub readonly: bool,
    }
}

// ============================================================================
// Directory Operations
// ============================================================================

/// Directory operations
pub mod dir {
    use super::*;

    /// Create directory
    pub async fn create<P: AsRef<Path>>(path: P) -> IoResult<()> {
        fs::create_dir(path).await.map_err(Into::into)
    }

    /// Create directory and all parent directories
    pub async fn create_all<P: AsRef<Path>>(path: P) -> IoResult<()> {
        fs::create_dir_all(path).await.map_err(Into::into)
    }

    /// Remove empty directory
    pub async fn remove<P: AsRef<Path>>(path: P) -> IoResult<()> {
        fs::remove_dir(path).await.map_err(Into::into)
    }

    /// Remove directory and all contents
    pub async fn remove_all<P: AsRef<Path>>(path: P) -> IoResult<()> {
        fs::remove_dir_all(path).await.map_err(Into::into)
    }

    /// List directory entries
    pub async fn list<P: AsRef<Path>>(path: P) -> IoResult<Vec<DirEntry>> {
        let mut entries = Vec::new();
        let mut read_dir = fs::read_dir(path).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            let file_type = entry.file_type().await?;
            entries.push(DirEntry {
                name: entry.file_name().to_string_lossy().into_owned(),
                path: entry.path().to_string_lossy().into_owned(),
                is_file: file_type.is_file(),
                is_dir: file_type.is_dir(),
                is_symlink: file_type.is_symlink(),
            });
        }
        Ok(entries)
    }

    /// Directory entry
    #[derive(Debug, Clone)]
    pub struct DirEntry {
        pub name: String,
        pub path: String,
        pub is_file: bool,
        pub is_dir: bool,
        pub is_symlink: bool,
    }
}

// ============================================================================
// Network Operations - TCP
// ============================================================================

/// TCP networking operations
pub mod tcp {
    use super::*;

    /// TCP connection wrapper
    pub struct TcpConnection {
        stream: TcpStream,
        peer_addr: String,
    }

    impl TcpConnection {
        /// Connect to a TCP server
        pub async fn connect(addr: &str) -> IoResult<Self> {
            let stream = TcpStream::connect(addr).await?;
            let peer_addr = stream
                .peer_addr()
                .map(|a| a.to_string())
                .unwrap_or_else(|_| "unknown".to_string());
            Ok(Self { stream, peer_addr })
        }

        /// Get peer address
        pub fn peer_addr(&self) -> &str {
            &self.peer_addr
        }

        /// Read data
        pub async fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
            self.stream.read(buf).await.map_err(Into::into)
        }

        /// Read exact number of bytes
        pub async fn read_exact(&mut self, buf: &mut [u8]) -> IoResult<()> {
            self.stream.read_exact(buf).await.map_err(Into::into).map(|_| ())
        }

        /// Read all available data
        pub async fn read_to_end(&mut self) -> IoResult<Vec<u8>> {
            let mut buf = Vec::new();
            self.stream.read_to_end(&mut buf).await?;
            Ok(buf)
        }

        /// Read line (until newline)
        pub async fn read_line(&mut self) -> IoResult<String> {
            let mut reader = BufReader::new(&mut self.stream);
            let mut line = String::new();
            reader.read_line(&mut line).await?;
            Ok(line)
        }

        /// Write data
        pub async fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
            self.stream.write(buf).await.map_err(Into::into)
        }

        /// Write all data
        pub async fn write_all(&mut self, buf: &[u8]) -> IoResult<()> {
            self.stream.write_all(buf).await.map_err(Into::into)
        }

        /// Flush write buffer
        pub async fn flush(&mut self) -> IoResult<()> {
            self.stream.flush().await.map_err(Into::into)
        }

        /// Shutdown connection
        pub async fn shutdown(&mut self) -> IoResult<()> {
            self.stream.shutdown().await.map_err(Into::into)
        }
    }

    /// TCP server wrapper
    pub struct TcpServer {
        listener: TcpListener,
        local_addr: String,
    }

    impl TcpServer {
        /// Bind to address and start listening
        pub async fn bind(addr: &str) -> IoResult<Self> {
            let listener = TcpListener::bind(addr).await?;
            let local_addr = listener
                .local_addr()
                .map(|a| a.to_string())
                .unwrap_or_else(|_| "unknown".to_string());
            Ok(Self {
                listener,
                local_addr,
            })
        }

        /// Get local address
        pub fn local_addr(&self) -> &str {
            &self.local_addr
        }

        /// Accept next connection
        pub async fn accept(&self) -> IoResult<TcpConnection> {
            let (stream, addr) = self.listener.accept().await?;
            Ok(TcpConnection {
                stream,
                peer_addr: addr.to_string(),
            })
        }
    }
}

// ============================================================================
// Network Operations - UDP
// ============================================================================

/// UDP networking operations
pub mod udp {
    use super::*;

    /// UDP socket wrapper
    pub struct UdpConnection {
        socket: UdpSocket,
        local_addr: String,
    }

    impl UdpConnection {
        /// Bind to local address
        pub async fn bind(addr: &str) -> IoResult<Self> {
            let socket = UdpSocket::bind(addr).await?;
            let local_addr = socket
                .local_addr()
                .map(|a| a.to_string())
                .unwrap_or_else(|_| "unknown".to_string());
            Ok(Self { socket, local_addr })
        }

        /// Get local address
        pub fn local_addr(&self) -> &str {
            &self.local_addr
        }

        /// Connect to remote address (for send/recv without specifying addr)
        pub async fn connect(&self, addr: &str) -> IoResult<()> {
            self.socket.connect(addr).await.map_err(Into::into)
        }

        /// Send data to connected address
        pub async fn send(&self, buf: &[u8]) -> IoResult<usize> {
            self.socket.send(buf).await.map_err(Into::into)
        }

        /// Receive data from connected address
        pub async fn recv(&self, buf: &mut [u8]) -> IoResult<usize> {
            self.socket.recv(buf).await.map_err(Into::into)
        }

        /// Send data to specific address
        pub async fn send_to(&self, buf: &[u8], addr: &str) -> IoResult<usize> {
            self.socket.send_to(buf, addr).await.map_err(Into::into)
        }

        /// Receive data with sender address
        pub async fn recv_from(&self, buf: &mut [u8]) -> IoResult<(usize, String)> {
            let (size, addr) = self.socket.recv_from(buf).await?;
            Ok((size, addr.to_string()))
        }
    }
}

// ============================================================================
// Standard I/O Operations
// ============================================================================

/// Standard I/O operations
pub mod stdio {
    use super::*;

    /// Print to stdout (no newline)
    pub fn print(s: &str) {
        print!("{}", s);
        let _ = io::stdout().flush();
    }

    /// Print to stdout with newline
    pub fn println(s: &str) {
        println!("{}", s);
    }

    /// Print to stderr (no newline)
    pub fn eprint(s: &str) {
        eprint!("{}", s);
        let _ = io::stderr().flush();
    }

    /// Print to stderr with newline
    pub fn eprintln(s: &str) {
        eprintln!("{}", s);
    }

    /// Read line from stdin
    pub fn read_line() -> IoResult<String> {
        let mut line = String::new();
        io::stdin()
            .lock()
            .read_line(&mut line)
            .map_err(IoError::from)?;
        // Remove trailing newline
        if line.ends_with('\n') {
            line.pop();
            if line.ends_with('\r') {
                line.pop();
            }
        }
        Ok(line)
    }

    /// Read all stdin until EOF
    pub fn read_all() -> IoResult<String> {
        let mut contents = String::new();
        for line in io::stdin().lock().lines() {
            contents.push_str(&line.map_err(IoError::from)?);
            contents.push('\n');
        }
        Ok(contents)
    }

    /// Formatted print (like printf)
    pub fn printf(format: &str, args: &[Value]) -> String {
        let mut result = String::new();
        let mut arg_idx = 0;
        let mut chars = format.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '%' {
                match chars.peek() {
                    Some('%') => {
                        chars.next();
                        result.push('%');
                    }
                    Some('s') => {
                        chars.next();
                        if arg_idx < args.len() {
                            result.push_str(&format!("{}", args[arg_idx]));
                            arg_idx += 1;
                        }
                    }
                    Some('d') | Some('i') => {
                        chars.next();
                        if arg_idx < args.len() {
                            if let Value::Int(n) = &args[arg_idx] {
                                result.push_str(&format!("{}", n));
                            }
                            arg_idx += 1;
                        }
                    }
                    Some('f') => {
                        chars.next();
                        if arg_idx < args.len() {
                            if let Value::Float(n) = &args[arg_idx] {
                                result.push_str(&format!("{}", n));
                            }
                            arg_idx += 1;
                        }
                    }
                    Some('b') => {
                        chars.next();
                        if arg_idx < args.len() {
                            if let Value::Bool(b) = &args[arg_idx] {
                                result.push_str(&format!("{}", b));
                            }
                            arg_idx += 1;
                        }
                    }
                    Some('t') => {
                        chars.next();
                        if arg_idx < args.len() {
                            if let Value::Ternary(t) = &args[arg_idx] {
                                result.push_str(&format!("{}", t));
                            }
                            arg_idx += 1;
                        }
                    }
                    _ => result.push(c),
                }
            } else {
                result.push(c);
            }
        }
        result
    }
}

// ============================================================================
// Buffered I/O
// ============================================================================

/// Buffered file operations for large files
pub mod buffered {
    use super::*;

    /// Buffered file reader
    pub struct BufferedReader {
        reader: BufReader<File>,
    }

    impl BufferedReader {
        /// Open file for buffered reading
        pub async fn open<P: AsRef<Path>>(path: P) -> IoResult<Self> {
            let file = File::open(path).await?;
            Ok(Self {
                reader: BufReader::new(file),
            })
        }

        /// Read line
        pub async fn read_line(&mut self) -> IoResult<Option<String>> {
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line).await?;
            if bytes_read == 0 {
                Ok(None)
            } else {
                // Remove trailing newline
                if line.ends_with('\n') {
                    line.pop();
                    if line.ends_with('\r') {
                        line.pop();
                    }
                }
                Ok(Some(line))
            }
        }

        /// Read chunk of bytes
        pub async fn read_chunk(&mut self, size: usize) -> IoResult<Vec<u8>> {
            let mut buf = vec![0u8; size];
            let bytes_read = self.reader.read(&mut buf).await?;
            buf.truncate(bytes_read);
            Ok(buf)
        }
    }

    /// Buffered file writer
    pub struct BufferedWriter {
        writer: BufWriter<File>,
    }

    impl BufferedWriter {
        /// Open file for buffered writing
        pub async fn create<P: AsRef<Path>>(path: P) -> IoResult<Self> {
            let file = File::create(path).await?;
            Ok(Self {
                writer: BufWriter::new(file),
            })
        }

        /// Open file for buffered appending
        pub async fn append<P: AsRef<Path>>(path: P) -> IoResult<Self> {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .await?;
            Ok(Self {
                writer: BufWriter::new(file),
            })
        }

        /// Write line
        pub async fn write_line(&mut self, line: &str) -> IoResult<()> {
            self.writer.write_all(line.as_bytes()).await?;
            self.writer.write_all(b"\n").await?;
            Ok(())
        }

        /// Write bytes
        pub async fn write(&mut self, data: &[u8]) -> IoResult<()> {
            self.writer.write_all(data).await.map_err(Into::into)
        }

        /// Flush buffer to disk
        pub async fn flush(&mut self) -> IoResult<()> {
            self.writer.flush().await.map_err(Into::into)
        }
    }
}

// ============================================================================
// Path utilities
// ============================================================================

/// Path manipulation utilities
pub mod path {
    use std::path::{Path, PathBuf};

    /// Join path components
    pub fn join(base: &str, parts: &[&str]) -> String {
        let mut path = PathBuf::from(base);
        for part in parts {
            path.push(part);
        }
        path.to_string_lossy().into_owned()
    }

    /// Get parent directory
    pub fn parent(path: &str) -> Option<String> {
        Path::new(path)
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
    }

    /// Get file name
    pub fn file_name(path: &str) -> Option<String> {
        Path::new(path)
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
    }

    /// Get file stem (name without extension)
    pub fn file_stem(path: &str) -> Option<String> {
        Path::new(path)
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
    }

    /// Get file extension
    pub fn extension(path: &str) -> Option<String> {
        Path::new(path)
            .extension()
            .map(|e| e.to_string_lossy().into_owned())
    }

    /// Check if path is absolute
    pub fn is_absolute(path: &str) -> bool {
        Path::new(path).is_absolute()
    }

    /// Normalize path (resolve . and ..)
    pub fn normalize(path: &str) -> String {
        let mut components = Vec::new();
        for component in Path::new(path).components() {
            use std::path::Component;
            match component {
                Component::ParentDir => {
                    components.pop();
                }
                Component::CurDir => {}
                c => components.push(c),
            }
        }
        let result: PathBuf = components.iter().collect();
        result.to_string_lossy().into_owned()
    }
}

// ============================================================================
// Native function bindings for betlang
// ============================================================================

use crate::value::NativeFunction;

/// Get all I/O native functions
pub fn native_functions() -> Vec<NativeFunction> {
    vec![
        NativeFunction {
            name: "print",
            arity: 1,
            func: |args| {
                if let Some(v) = args.first() {
                    stdio::print(&format!("{}", v));
                }
                Ok(Value::Unit)
            },
        },
        NativeFunction {
            name: "println",
            arity: 1,
            func: |args| {
                if let Some(v) = args.first() {
                    stdio::println(&format!("{}", v));
                }
                Ok(Value::Unit)
            },
        },
        NativeFunction {
            name: "eprint",
            arity: 1,
            func: |args| {
                if let Some(v) = args.first() {
                    stdio::eprint(&format!("{}", v));
                }
                Ok(Value::Unit)
            },
        },
        NativeFunction {
            name: "eprintln",
            arity: 1,
            func: |args| {
                if let Some(v) = args.first() {
                    stdio::eprintln(&format!("{}", v));
                }
                Ok(Value::Unit)
            },
        },
        NativeFunction {
            name: "read_line",
            arity: 0,
            func: |_| match stdio::read_line() {
                Ok(s) => Ok(Value::String(Arc::new(s))),
                Err(e) => Ok(Value::Error(Arc::new(e.to_string()))),
            },
        },
        NativeFunction {
            name: "file_exists",
            arity: 1,
            func: |args| {
                if let Some(Value::String(path)) = args.first() {
                    Ok(Value::Bool(std::path::Path::new(path.as_str()).exists()))
                } else {
                    Err("file_exists expects a string path".to_string())
                }
            },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_join() {
        assert_eq!(path::join("/home", &["user", "file.txt"]), "/home/user/file.txt");
    }

    #[test]
    fn test_path_normalize() {
        assert_eq!(path::normalize("/foo/bar/../baz"), "/foo/baz");
    }

    #[test]
    fn test_printf() {
        let result = stdio::printf("Hello %s, you have %d items", &[
            Value::String(Arc::new("World".to_string())),
            Value::Int(42),
        ]);
        assert_eq!(result, "Hello World, you have 42 items");
    }
}
