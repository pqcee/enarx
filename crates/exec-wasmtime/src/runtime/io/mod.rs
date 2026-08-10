// SPDX-License-Identifier: Apache-2.0

//! I/O functionality for keeps

pub mod null;

use wasi_common::{file::FileAccessMode, WasiFile};

pub fn stdio_file(file: impl WasiFile + 'static) -> (Box<dyn WasiFile>, FileAccessMode) {
    // Ensure wasmtime can detect the TTY.
    let caps = FileAccessMode::all();
    (Box::new(file), caps)
}
