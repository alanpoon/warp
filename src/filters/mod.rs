//! Built-in Filters
//!
//! This module mostly serves as documentation to group together the list of
//! built-in filters. Most of these are available at more convenient paths.

pub mod addr;
pub mod any;
pub mod body;
#[cfg(any(feature = "compression-brotli", feature = "compression-gzip"))]
pub mod compression;
pub mod cookie;
pub mod cors;
pub mod ext;
#[cfg(not(target_os = "wasi"))]
pub mod fs;
#[cfg(target_os = "wasi")]
pub mod fs_wasi;
#[cfg(target_os = "wasi")]
pub use fs_wasi as fs;
pub mod header;
pub mod host;
pub mod log;
pub mod method;
#[cfg(feature = "multipart")]
pub mod multipart;
pub mod path;
pub mod query;
pub mod reply;
pub mod sse;
pub mod trace;
#[cfg(feature = "websocket")]
//#[cfg(not(target_os = "wasi"))]
pub mod ws;
//#[cfg(target_os = "wasi")]
//pub mod ws_wasi;
//#[cfg(target_os = "wasi")]
//pub use ws_wasi as ws;
pub use crate::filter::BoxedFilter;
