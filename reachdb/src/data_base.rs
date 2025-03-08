// All the metadata for a particular session is stored here

use std::result::Result;
use crate::{errors::ReachdbError, records::{node::NodeRecord, relationship::RelationshipRecord, Record}, utils::create_mmap};
use log::info;
use memmap2::{Mmap, MmapMut};

/// Stores all the metadata for a particular session
pub struct Reachdb {
    node_mmap_size: usize,
    relationship_mmap_size: usize,
    node_mmap: Option<MmapMut>,
    relation_mmap: Option<MmapMut>,
    // Metadata
    node_count: u64,
    relationship_count: u64,
}

impl Reachdb {
    pub fn new() -> Result<Self, ReachdbError> {        
        Ok(Self {
            node_mmap_size: 4096,
            relationship_mmap_size: 4096,
            node_mmap: None,
            relation_mmap: None,
            // Metadata
            node_count: 0,
            relationship_count: 0,
        })
    }

    fn update(
        &mut self,
        node_count: Option<u64>, 
        relationship_count: Option<u64>, 
        node_mmap_size: Option<usize>, 
        relationship_mmap_size: Option<usize>, 
        node_mmap: Option<MmapMut>, 
        relation_mmap: Option<MmapMut>) 
    {
        // Keeps the previous value if None
        self.node_count = node_count.unwrap_or(self.node_count); 
        self.relationship_count = relationship_count.unwrap_or(self.relationship_count);
        self.node_mmap_size = node_mmap_size.unwrap_or(self.node_mmap_size);
        self.relationship_mmap_size = relationship_mmap_size.unwrap_or(self.relationship_mmap_size);
        self.node_mmap = node_mmap.or(self.node_mmap.take());
        self.relation_mmap = relation_mmap.or(self.relation_mmap.take());

    }

    pub fn prepare(&mut self, node_mmap_size: usize, relation_mmap_size: usize) -> Result<(), ReachdbError> {
        // Create directories if they don't exist
        std::fs::create_dir_all("data")?;

        // Open sled databases (this will create them if they don't exist)
        let node_db = sled::open(Self::get_db_path().0)?;
        let property_db = sled::open(Self::get_property_db_path())?;        
        // Make sure the databases are properly initialized
        node_db.flush()?;
        property_db.flush()?;

        // Creating the Mmap files
        let node_mmap = create_mmap(
            &Self::get_db_path().1,
            node_mmap_size
        )?;
        let relation_mmap = create_mmap(
            &Self::get_db_path().2,
            relation_mmap_size
        )?;

        // Update the metadata
        self.update(
            None,
            None,
            Some(node_mmap_size),
            Some(relation_mmap_size),
            Some(node_mmap),
            Some(relation_mmap)
        );

        info!("Databases created successfully");

        Ok(())
    }

    fn get_property_db_path() -> String {
        "data/property".to_string()
    }

    fn get_db_path() -> (String,String, String) {
        (
            "data/reachdb.nodeid".to_string(),
            "data/reachdb.node.db".to_string(),
            "data/reachdb.relationship.db".to_string(),
        )
    }

    fn update_links(node: &mut NodeRecord, mmap: &mut MmapMut, new_relation_offset: u64) -> Result<u64, ReachdbError> {

        // Process the src relation
        let relation_offset = node.first_relationship_offset;
        let mut relation = RelationshipRecord::read(mmap, relation_offset)?;

        // Update this old src relationship record
        relation.update(
            None,
            Some(new_relation_offset),
            None,
            None,
            None
        );
        // Write the relationship back to the mmap
        relation.write(mmap, relation_offset)?;

        // Update the current node
        node.update(
            Some(new_relation_offset),
            None
        );
        // Write the node back to the mmap
        node.write(mmap, node.id)?;

        Ok(relation_offset)
    }

    fn add_relation(&mut self, src_id: &u64, tgt_id: &u64, relation_str: &str) -> Result<(), ReachdbError> {

        let node_mmap = self.node_mmap.as_mut().expect("Mmap not initialized");
        let mut src_node = NodeRecord::read(node_mmap, *src_id)?;
        let mut tgt_node = NodeRecord::read(node_mmap, *tgt_id)?;

        // Process the nodes's relation
        let relation_offset = src_node.first_relationship_offset + 1;
        let prev_src_relation_offset = Self::update_links(&mut src_node, node_mmap, relation_offset)?;
        let prev_tgt_relation_offset = Self::update_links(&mut tgt_node, node_mmap, relation_offset)?;

        // Write the new relationship record
        let _relation = RelationshipRecord::new(
            *src_id,
            *tgt_id,
            None,
            None,
            Some(prev_src_relation_offset),
            None,
            Some(prev_tgt_relation_offset),
        ).write(node_mmap, relation_offset)?;

        // Metadata update
        self.relationship_count += 1;

        Ok(())

    }

    fn if_edge_exists(&self) -> Result<bool, ReachdbError> {
        todo!("Check if the edge already exists");
    }

    pub fn get_node_id(&mut self, node: &str) -> Result<u64, ReachdbError> {

        let db = sled::open(Self::get_db_path().0)?;

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

        if !self.if_edge_exists()? {
            // Add the relationship
            self.add_relation(&src_id, &tgt_id, relationship)?;
        }
        info!("Added Edge: \"{}\"(id:{}) - [{}] -> \"{}\"(id:{})", source, src_id, relationship, target, tgt_id);

        Ok(())
    }
}