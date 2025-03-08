// All the metadata for a particular session is stored here

// use std::result::Result;

use crate::errors::ReachdbError;
use log::info;

pub struct Reachdb {
    node_count: u64,
}

impl Reachdb {
    pub fn new() -> Result<Self, ReachdbError> {
        // Create directories if they don't exist
        std::fs::create_dir_all("data")?;

        // Open the databases (this will create them if they don't exist)
        let node_db = sled::open(Self::get_nodedb())?;
        let property_db = sled::open(Self::get_propertydb())?;
        
        // Make sure the databases are properly initialized
        node_db.flush()?;
        property_db.flush()?;
        
        info!("Databases created successfully");
        
        Ok(Self {
            node_count: 0,
        })
    }
    pub fn prepare(&self) -> Result<(), ReachdbError> {
       todo!("Implement this function")
    }
    pub fn get_propertydb() -> String {
        "data/property".to_string()
    }
    pub fn get_nodedb() -> String {
        "data/node".to_string()
    }
    pub fn get_node_id(&mut self, node: &str) -> Result<u64, ReachdbError> {

        let db = sled::open(Self::get_nodedb())?;

        // Check if the String is already mapped
        if let Some(id_bytes) = db.get(node)? {
            let id = bincode::deserialize::<u64>(&id_bytes)?;
            info!("Found: \"{}\"(id:{})", node, id);
            return Ok(id);
        }

        // Reterieve and update the counter
        let new_id = self.node_count + 1;

        // Insert the mapping: string -> new_id, and update the counter.
        db.insert(node, bincode::serialize(&new_id)?)?;
        self.node_count += 1; // Increment the counter
        db.flush()?; // Ensure data is persisted
        info!("Added: \"{}\"(id:{})", node, new_id);
        Ok(new_id)
    }
    pub fn add_edge(&mut self, source: &str, target: &str, relationship: &str) -> Result<(), ReachdbError> {
        let src_id = self.get_node_id(source)?;
        let tgt_id = self.get_node_id(target)?;
        info!("Added Edge: \"{}\"(id:{}) - [{}] -> \"{}\"(id:{})", source, src_id, relationship, target, tgt_id);

        Ok(())
    }
}