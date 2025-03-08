use super::errors::ReachdbError;
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

/// Insert/Update a key-value pair into the database, for a given database path.
pub fn db_insert<T: serde::Serialize>(db_path: &str, key: &str, val: &T) -> Result<(), ReachdbError> {
    let db = sled::open(db_path)?;
    let val = bincode::serialize(val)?;

    // Insert a key-value pair into the database.
    db.insert(key, val)?;

    // Flush the database to disk.
    db.flush()?;

    Ok(())
}

/// Retrieve a value from the database by its key, for a given database path.
pub fn db_get<T>(db_path: &str, key: &str) -> Result<(), ReachdbError> {
    let db = sled::open(db_path)?;
    // Retrieve a value by its key.
    if let Some(value) = db.get(key)? {
        // Convert the retrieved value (a byte array) into a String.
        println!("{} => {}", key, String::from_utf8(value.to_vec())?);
    } else {
        println!("No value found for {}", key);
    }

    Ok(())
}