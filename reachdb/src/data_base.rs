// All the metadata for a particular session is stored here

use std::result::Result;
use crate::{errors::ReachdbError, records::{node::NodeRecord, relationship::RelationshipRecord, Record, NULL_OFFSET}, utils::create_mmap};
use log::{info, debug};
use memmap2::MmapMut;
use serde::{Deserialize, Serialize};

pub trait UserDefinedRelationType {
    fn get_type_id(relation: &str) -> Option<Self> where Self: Sized;
    fn type_id(&self) -> u8;
}

/// Wrapper for the memory maps
#[derive(Debug)]
pub struct MmapWrapper {
    pub node_mmap: MmapMut,
    pub relation_mmap: MmapMut,
}
impl MmapWrapper {
    pub fn flush(&self) -> Result<(), ReachdbError> {
        self.node_mmap.flush()?;
        self.relation_mmap.flush()?;
        Ok(())
    }
    pub fn get_node_as_mut(&mut self) -> &mut MmapMut {
        &mut self.node_mmap
    }
    pub fn get_relation_as_mut(&mut self) -> &mut MmapMut {
        &mut self.relation_mmap
    }
    pub fn get_node_as_ref(&self) -> &MmapMut {
        &self.node_mmap
    }
    pub fn get_relation_as_ref(&self) -> &MmapMut {
        &self.relation_mmap
    }
    pub fn take_as_mut(&mut self) -> (&mut MmapMut, &mut MmapMut) {
        (&mut self.node_mmap, &mut self.relation_mmap)
    }
    pub fn take_as_ref(&self) -> (&MmapMut, &MmapMut) {
        (&self.node_mmap, &self.relation_mmap)
    }
    
}

/// Stores all the metadata for a particular session
#[derive(Debug, Serialize, Deserialize)]
pub struct Reachdb<E: UserDefinedRelationType> {
    
    #[serde(skip_serializing, skip_deserializing)]
    _marker: std::marker::PhantomData<E>, // We don't store `E`, but we want to enforce the trait

    #[serde(skip_serializing, skip_deserializing)]
    mmap: Option<MmapWrapper>,
    
    // Metadata
    node_mmap_size: usize,
    relation_mmap_size: usize,
    node_count: u64,
    relationship_count: u64,
    property_count: u64,
    path: String,

}

impl<E: UserDefinedRelationType + std::fmt::Debug> Reachdb<E> {
    fn new(path: &str) -> Result<Self, ReachdbError> {        
        Ok(Self {
            _marker: std::marker::PhantomData,
            mmap: None,
            
            // Metadata
            node_mmap_size: 4096,
            relation_mmap_size: 4096,
            node_count: 0,
            relationship_count: 0,
            property_count: 0,
            path: path.to_string(),
        })
    }

    fn update(
        &mut self,
        node_count: Option<u64>, 
        relationship_count: Option<u64>, 
        node_mmap_size: Option<usize>, 
        relation_mmap_size: Option<usize>, 
        mmap: Option<MmapWrapper>) 
    {
        // Keeps the previous value if None
        self.node_count = node_count.unwrap_or(self.node_count); 
        self.relationship_count = relationship_count.unwrap_or(self.relationship_count);
        self.node_mmap_size = node_mmap_size.unwrap_or(self.node_mmap_size);
        self.relation_mmap_size = relation_mmap_size.unwrap_or(self.relation_mmap_size);
        self.mmap = mmap.or(self.mmap.take());
    }

    pub fn prepare(
        &mut self,
        node_mmap_size: Option<usize>,
        relation_mmap_size: Option<usize>) -> Result<(), ReachdbError> {

        info!("Preparing databases...");
        // If None then keep 4096(default value)
        let node_mmap_size  = node_mmap_size.unwrap_or(self.node_mmap_size);
        let relation_mmap_size  = relation_mmap_size.unwrap_or(self.relation_mmap_size);

        // Open sled databases (this will create them if they don't exist)
        let node_db = sled::open(&Self::get_db_path(&self.path)[0])?;
        let property_db = sled::open(&Self::get_db_path(&self.path)[1])?;  

        // Make sure the databases are properly initialized
        node_db.flush()?;
        property_db.flush()?;

        // Creating the Mmap files
        let node_mmap = create_mmap(
            &Self::get_db_path(&self.path)[2],
            node_mmap_size
        )?;
        let relation_mmap = create_mmap(
            &Self::get_db_path(&self.path)[3],
            relation_mmap_size
        )?;

        // Update the metadata
        self.update(
            None,
            None,
            Some(node_mmap_size),
            Some(relation_mmap_size),
            Some(MmapWrapper {
                node_mmap,
                relation_mmap
            })
        );

        info!("Databases created successfully");

        Ok(())
    }

    pub fn open(path: &str, node_mmap_size: Option<usize>, relation_mmap_size: Option<usize>) -> Result<Self, ReachdbError> {
        
        info!("Opening databases...");
        let mut reachdb: Reachdb<E>;
        // Check if path exists
        if !std::path::Path::new(path).exists() {
            // Create the directory since it doesn't exist
            std::fs::create_dir_all(path)?;
            reachdb = Self::new(path)?;
        } else {
            let metadata_path = &Self::get_db_path(path)[4];
            if !std::path::Path::new(metadata_path).exists() {
                reachdb = Self::new(path)?;
            } else {
                let metadata = std::fs::read_to_string(metadata_path)?;
                reachdb = serde_json::from_str(&metadata)?;
            }
        }
        
        // Prepare the databases
        reachdb.prepare(node_mmap_size, relation_mmap_size)?;

        info!("Opening databases... {:#?}", reachdb);
        info!("Databases opened successfully");

        Ok(reachdb)
    }

    // Close databases and save metadata
    pub fn close(&mut self) -> Result<(), ReachdbError> {

        // Serialize and save metadata to a file
        let metadata_path = &Self::get_db_path(&self.path)[4];
        let metadata = serde_json::to_string(&self)?;
        std::fs::write(metadata_path, metadata)?;
        
        // Flush and drop memory maps
        if let Some(mmap) = self.mmap.take() {
            mmap.flush()?;
        }
        
        info!("Reachdb closed successfully");
        Ok(())
    }
        
    fn get_db_path(path: &str) -> Vec<String> {
        vec![
            format!("{}/reachdb.nodeid", path),
            format!("{}/reachdb.property", path),
            format!("{}/reachdb.node.db", path),
            format!("{}/reachdb.relationship.db", path),
            format!("{}/reachdb.metadata.json", path),
        ]
    }

    fn update_node_links(&mut self, node: &mut NodeRecord, new_relation_id: u64, is_target_node: bool) -> Result<u64, ReachdbError> {
        let (node_mmap, relation_mmap) = self.mmap
            .as_mut()
            .expect("Mmap not initialized")
            .take_as_mut();

        let relation_id = node.first_relationship_id;

        // Check if this is Not a new node
        if relation_id != NULL_OFFSET {
            // Process the src relation
            let mut relation = RelationshipRecord::read(relation_mmap, relation_id)?;

            // Update this old src relationship record
            match is_target_node {
                true => {
                    relation.update(
                        None,
                        None,
                        None,
                        Some(new_relation_id),
                        None
                    );
                },
                false => {
                    relation.update(
                        None,
                        Some(new_relation_id),
                        None,
                        None,
                        None
                    );
                }
            }
            // Write the relationship back to the mmap
            relation.write(relation_mmap, relation_id)?;
        }

        // Update the current node
        node.update(
            Some(new_relation_id),
            None
        );
        // Write the node back to the mmap
        node.write(node_mmap, node.id)?;

        Ok(relation_id)
    }

    fn add_relation(&mut self, src_id: &u64, tgt_id: &u64, type_id: &u8) -> Result<(), ReachdbError> {

        let node_mmap = self.mmap
            .as_ref()
            .expect("Mmap not initialized")
            .get_node_as_ref();

        let mut src_node = NodeRecord::read(node_mmap, *src_id)?;
        let mut tgt_node = NodeRecord::read(node_mmap, *tgt_id)?;

        // Process the nodes's relation
        let new_relation_id = self.relationship_count;
        let prev_src_relation_id= self.update_node_links(&mut src_node, new_relation_id, false)?;
        let prev_tgt_relation_id = self.update_node_links(&mut tgt_node, new_relation_id, true)?;
        
        // Write the new relationship record
        let relation_mmap = self.mmap
            .as_mut()
            .expect("Mmap not initialized")
            .get_relation_as_mut();

        let _relation = RelationshipRecord::new(
            *src_id,
            *tgt_id,
            *type_id,
            None,
            None,
            Some(prev_src_relation_id),
            None,
            Some(prev_tgt_relation_id),
        ).write(relation_mmap, new_relation_id)?;
        info!("Added new RelationRecord: [type: {}](id:{})", type_id, new_relation_id);

        // Metadata update
        self.relationship_count += 1;

        Ok(())

    }
    // fn get_mmap(&self) -> Result<(&MmapMut, &MmapMut), ReachdbError> {
    //     let node_mmap = self.node_mmap.as_ref().expect("Node Mmap not initialized");
    //     let relation_mmap = self.relation_mmap.as_ref().expect("RelationMmap not initialized");
    //     Ok((node_mmap, relation_mmap))
    // }
    pub fn get_node(&self, node_id: u64) -> Result<NodeRecord, ReachdbError> {
        let (node_mmap, _) = match self.mmap.as_ref() {
            Some(mmap) => mmap.take_as_ref(),
            None => return Err(ReachdbError::OtherError("Mmap not initialized".to_string())),
        };
        NodeRecord::read(node_mmap, node_id)
    }
    pub fn get_relation(&self, relation_id: u64) -> Result<RelationshipRecord, ReachdbError> {
        let (_, relation_mmap) = match self.mmap.as_ref() {
            Some(mmap) => mmap.take_as_ref(),
            None => return Err(ReachdbError::OtherError("Mmap not initialized".to_string())),
        };
        RelationshipRecord::read(relation_mmap, relation_id)
    }
    pub fn get_connected_node(&self, node_id: u64, relation_id: u64) -> Result<u64, ReachdbError> {
        let (_, relation_mmap) = match self.mmap.as_ref() {
            Some(mmap) => mmap.take_as_ref(),
            None => return Err(ReachdbError::OtherError("Mmap not initialized".to_string())),
        };
        let relation = RelationshipRecord::read(relation_mmap, relation_id)?;
        let next_id = if node_id == relation.source_id {
            relation.target_id
        } else {
            relation.source_id
        };
        debug!("Relation: {relation:#?} Next Node: {next_id}", relation=relation, next_id=next_id);
        Ok(next_id)
    }
    pub fn get_all_node_relations(&self, node_id: u64) -> Result<Vec<u64>, ReachdbError> {
        let (node_mmap, relation_mmap) = match self.mmap.as_ref() {
            Some(mmap) => mmap.take_as_ref(),
            None => return Err(ReachdbError::OtherError("Mmap not initialized".to_string())),
        };
        let node = NodeRecord::read(node_mmap, node_id)?;
        let relations = RelationshipRecord::into_iter(
            relation_mmap,
            &node_id,
            node.first_relationship_id
        ).filter_map(|rel| {
            rel.ok().map(|(id, _)| id)
        }).collect();
        debug!("Node: {node:#?} Relations: {relations:?}", node=node, relations=relations);
        Ok(relations)
    }
    pub fn get_outgoing_node_relations(&self, node_id: u64) -> Result<Vec<u64>, ReachdbError> {
        let (node_mmap, relation_mmap) = match self.mmap.as_ref() {
            Some(mmap) => mmap.take_as_ref(),
            None => return Err(ReachdbError::OtherError("Mmap not initialized".to_string())),
        };
        let node = NodeRecord::read(node_mmap, node_id)?;
        let relations = RelationshipRecord::into_iter(
            relation_mmap,
            &node_id,
            node.first_relationship_id
        ).filter_map(|rel| {
            rel.ok().map(|(id, rel)| {
                if rel.source_id == node_id {
                    Some(id)
                } else {
                    None
                }
            }).flatten()
        }).collect();
        debug!("Node: {node:#?} Relations: {relations:?}", node=node, relations=relations);
        Ok(relations)
    }
    pub fn get_incoming_node_relations(&self, node_id: u64) -> Result<Vec<u64>, ReachdbError> {
        let (node_mmap, relation_mmap) = match self.mmap.as_ref() {
            Some(mmap) => mmap.take_as_ref(),
            None => return Err(ReachdbError::OtherError("Mmap not initialized".to_string())),
        };
        let node = NodeRecord::read(node_mmap, node_id)?;
        let relations = RelationshipRecord::into_iter(
            relation_mmap,
            &node_id,
            node.first_relationship_id
        ).filter_map(|rel| {
            rel.ok().map(|(id, rel)| {
                if rel.target_id == node_id {
                    Some(id)
                } else {
                    None
                }
            }).flatten()
        }).collect();
        debug!("Node: {node:#?} Relations: {relations:?}", node=node, relations=relations);
        Ok(relations)
    }
    fn if_edge_exists(&self, src_id: &u64, tgt_id: &u64, type_id: &u8) -> Result<bool, ReachdbError> {
        let (node_mmap, relation_mmap) = match self.mmap.as_ref() {
            Some(mmap) => mmap.take_as_ref(),
            None => return Err(ReachdbError::OtherError("Mmap not initialized".to_string())),
        };
        let src_node = NodeRecord::read(node_mmap, *src_id)?;
        debug!("SRC_NODE inloop: {src_node:#?}");

        let exists = RelationshipRecord::into_iter(
            relation_mmap,
            src_id,
            src_node.first_relationship_id
        ).any(|rel| {
            // debug!("RelRec inloop: {rel:#?}");
            if let Ok((_, rel)) = rel {
                rel.target_id == *tgt_id &&
                rel.type_id == *type_id
            } else {
                false
            }
        });
        Ok(exists)
    }

    fn get_or_add_node_id(&mut self, node: &str) -> Result<u64, ReachdbError> {

        let db = sled::open(&Self::get_db_path(&self.path)[0])?;
        let property_db = sled::open(&Self::get_db_path(&self.path)[1])?;

        // Check if the String is already mapped
        if let Some(id_bytes) = db.get(node)? {
            let id = bincode::deserialize::<u64>(&id_bytes)?;
            info!("Found: \"{}\"(id:{})", node, id);
            return Ok(id);
        }

        // Reterieve and update the counter
        let new_id = self.node_count;
        let new_property_id = self.property_count; // property_id != node_id, because it consists of edge properties as well

        // Insert the mapping: string -> new_id, and update the counter.
        db.insert(node, bincode::serialize(&new_id)?)?;
        db.flush()?; // Ensure data is persisted
        
        // Insert the mapping: new_id -> node
        property_db.insert(bincode::serialize(&new_property_id)?, bincode::serialize(&node)?)?;
        property_db.flush()?; // Ensure data is persisted

        info!("Added: \"{}\"(id:{})", node, new_id);
        
        // Adding the NodeRecord
        let node_mmap = self.mmap
            .as_mut()
            .expect("Mmap not initialized")
            .get_node_as_mut();

        NodeRecord::new(new_id, new_property_id).write(node_mmap, new_id)?;
        info!("Added new NodeRecord \"{}\"(id:{})", node, new_id);
        
        self.node_count += 1; // Increment the counter
        self.property_count += 1; // Increment the counter
        
        Ok(new_id)
    }

    fn get_type_id(relation: &str) -> Option<u8> {
        debug!("Relation id: {}", relation);
        match E::get_type_id(relation) {
            Some(rel_type) => Some(rel_type.type_id()),
            None => None
        }
    }
    pub fn get_property(&self, property_id: u64) -> Result<String, ReachdbError> {
        let property_db = sled::open(&Self::get_db_path(&self.path)[1])?;
        match property_db.get(bincode::serialize(&property_id)?)? {
            Some(property) => Ok(bincode::deserialize::<String>(&property)?),
            None => Err(ReachdbError::OtherError(format!("Property not found for property_id: {}", property_id)))
        }
    }
    pub fn add_edge(&mut self, source: &str, target: &str, relationship: &str) -> Result<(), ReachdbError> {
        let src_id = self.get_or_add_node_id(source)?;
        let tgt_id = self.get_or_add_node_id(target)?;
        let type_id = match Self::get_type_id(relationship) {
            Some(id) => id,
            None => {
                info!("\x1b[31mError: Relation type '{}' not found, skipping edge\x1b[0m", relationship);
                return Ok(());
            }
        };

        if !self.if_edge_exists(&src_id, &tgt_id, &type_id)? {
            // Add the relationship
            self.add_relation(&src_id, &tgt_id, &type_id)?;
            info!("\x1b[32mAdded Edge: \"{}\"(id:{}) - [{}] -> \"{}\"(id:{})\x1b[0m", source, src_id, relationship, target, tgt_id);
        } else {
            info!("\x1b[33mFound Edge: \"{}\"(id:{}) - [{}] -> \"{}\"(id:{})\x1b[0m", source, src_id, relationship, target, tgt_id);
        }

        Ok(())
    }

    pub fn print_graph(&self) -> Result<(), ReachdbError> {
        let (node_mmap, relation_mmap) = match self.mmap.as_ref() {
            Some(mmap) => mmap.take_as_ref(),
            None => return Err(ReachdbError::OtherError("Mmap not initialized".to_string())),
        };
        println!("Priniting Records");
        for node_id in 0..self.node_count {
            info!("{:#?}", NodeRecord::read(node_mmap, node_id));
        }
        for rel_id in 0..self.relationship_count {
            info!("{:#?}", RelationshipRecord::read(relation_mmap, rel_id));
        }
        Ok(())
    }
}