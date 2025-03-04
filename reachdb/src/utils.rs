use super::error::ReachdbError;
use memmap2::{MmapMut, MmapOptions};
use std::fs::OpenOptions;

/// Creates and memory-maps a file at `file_path` with the specified `size`.
///
/// **Use-case:** Set up persistent storage for nodes, relationships, or properties using memory mapping.
pub fn create_mmap(file_path: &str, size: usize) -> Result<MmapMut, ReachdbError> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;
    file.set_len(size as u64)?;
    Ok(unsafe { MmapOptions::new().map_mut(&file)? })
}
