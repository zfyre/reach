use bincode;
use memmap2::MmapMut;
use serde::{Deserialize, Serialize};

use super::{ReachdbError, Record, NULL_OFFSET};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RelationshipRecord {
    pub source_id: u64,
    pub target_id: u64,   
    pub type_id: u8,
    pub first_property_id: u64,
    next_src_relationship_id: u64,
    prev_src_relationship_id: u64,
    next_tgt_relationship_id: u64,
    prev_tgt_relationship_id: u64,
}

impl Record for RelationshipRecord {
    fn read(mmap: &MmapMut, id: u64) -> Result<Self, ReachdbError>
    where
        Self: Sized,
    {   
        let offset = Self::id2offset(id);
        let end = offset + Self::record_size();
        let data = &mmap[offset..end];
        Ok(bincode::deserialize(data)?)
    }
    fn write(&self, mmap: &mut MmapMut, id: u64) -> Result<(), ReachdbError> {

        let offset = Self::id2offset(id);
        let encoded = bincode::serialize(self)?;
        let end = offset + encoded.len();
        mmap[offset..end].copy_from_slice(&encoded);
        mmap.flush()?;

        Ok(())
    }
    fn id2offset(id: u64) -> usize {
        match id {
            NULL_OFFSET => id as usize,
            _ => id as usize * Self::record_size()
        }
    }
}

impl RelationshipRecord {
    pub fn new(
        source_id: u64,
        target_id: u64,
        type_id: u8,
        first_property_id: Option<u64>,
        next_src_relationship_id: Option<u64>,
        prev_src_relationship_id: Option<u64>,
        next_tgt_relationship_id: Option<u64>,
        prev_tgt_relationship_id: Option<u64>,
        ) -> Self {
        Self {
            source_id,
            target_id,
            type_id,
            first_property_id: first_property_id.unwrap_or(NULL_OFFSET),
            next_src_relationship_id: next_src_relationship_id.unwrap_or(NULL_OFFSET),
            prev_src_relationship_id: prev_src_relationship_id.unwrap_or(NULL_OFFSET),
            next_tgt_relationship_id: next_tgt_relationship_id.unwrap_or(NULL_OFFSET),
            prev_tgt_relationship_id: prev_tgt_relationship_id.unwrap_or(NULL_OFFSET),
        }
    }
    
    pub fn update(
        &mut self,
        // source_id: Option<u64>,
        // target_id: Option<u64>,
        first_property_id: Option<u64>,
        next_src_relationship_id: Option<u64>,
        prev_src_relationship_id: Option<u64>,
        next_tgt_relationship_id: Option<u64>,
        prev_tgt_relationship_id: Option<u64>,
    ) {
        // self.source_id = source_id.unwrap_or(self.source_id);
        // self.target_id = target_id.unwrap_or(self.target_id);
        // self.type_id = type_id.unwrap_or(self.type_id);
        self.first_property_id = first_property_id.unwrap_or(self.first_property_id);
        self.next_src_relationship_id = next_src_relationship_id.unwrap_or(self.next_src_relationship_id);
        self.prev_src_relationship_id = prev_src_relationship_id.unwrap_or(self.prev_src_relationship_id);
        self.next_tgt_relationship_id = next_tgt_relationship_id.unwrap_or(self.next_tgt_relationship_id);
        self.prev_tgt_relationship_id = prev_tgt_relationship_id.unwrap_or(self.prev_tgt_relationship_id);
    }

    /// Initializes an iterator externally by providing `current_offset` and `mmap`
    pub fn into_iter<'a>(mmap: &'a MmapMut, node_id: &u64, current_id: u64) -> RelationshipIterator<'a> {

        if current_id == NULL_OFFSET {
            // Create an empty iterator when current_id is NULL_OFFSET
            RelationshipIterator {
            node_id: *node_id,
            initial_id: NULL_OFFSET,
            current_id: NULL_OFFSET,
            visited_prev: true, // Already visited to ensure next() returns None
            mmap,
            }
        } else {
            RelationshipIterator {
            node_id: *node_id,
            initial_id: current_id,
            current_id,
            visited_prev: false,
            mmap,
            }
        }
    }

}

pub struct RelationshipIterator<'a> {
    node_id: u64,
    initial_id: u64,
    current_id: u64,
    visited_prev: bool, // Tracks if we finished iterating in the prev direction
    mmap: &'a MmapMut,
}

impl<'a> Iterator for RelationshipIterator<'a> {
    type Item = Result<(u64, RelationshipRecord), ReachdbError>;

    fn next(&mut self) -> Option<Self::Item> {
        
        // If current_id is NULL_OFFSET, check if we should switch to next direction
        if self.current_id == NULL_OFFSET {
            if !self.visited_prev {
                // trace!("SWITCHING TO NEXT RELATIONSHIPS FROM INITIAL ID");
                self.visited_prev = true;
                self.current_id = self.initial_id; // Restart from initial_id
            } else {
                return None; // Fully exhausted both directions
            }
        }
        let this_id = self.current_id;
        // Read the current record
        match RelationshipRecord::read(self.mmap, self.current_id) {
            Ok(record) => {
                let is_source = record.source_id == self.node_id;
                let is_target = record.target_id == self.node_id;
                
                if !self.visited_prev {
                    // Iterate using prev relationships first
                    if is_source && record.prev_src_relationship_id != NULL_OFFSET {
                        // trace!("INSIDE PREV SRC");
                        self.current_id = record.prev_src_relationship_id;
                    } else if is_target && record.prev_tgt_relationship_id != NULL_OFFSET {
                        // trace!("INSIDE PREV TGT");
                        self.current_id = record.prev_tgt_relationship_id;
                    } else {
                        // If no more prev links, signal a switch
                        self.current_id = NULL_OFFSET;
                    }
                } else {
                    // Now iterate using next relationships
                    if is_source && record.next_src_relationship_id != NULL_OFFSET {
                        // trace!("INSIDE NEXT SRC");
                        self.current_id = record.next_src_relationship_id;
                    } else if is_target && record.next_tgt_relationship_id != NULL_OFFSET {
                        // trace!("INSIDE NEXT TGT");
                        self.current_id = record.next_tgt_relationship_id;
                    } else {
                        self.current_id = NULL_OFFSET;
                    }
                }
                Some(Ok((this_id, record)))
            }
            Err(e) => {
                // Return the error and stop iteration
                self.current_id = NULL_OFFSET;
                Some(Err(e))
            }
        }
    }
}


// #[cfg(test)]
// mod tests {
//     use crate::records::{relationship::RelationshipRecord, Record};

//     #[test]
//     fn writing_relationship_records() {
//         use crate::utils::create_mmap;
//         use memmap2::MmapOptions;
//         use std::fs::OpenOptions;

//         let file_path = "reachdb.relationship.db";

//         // Create file and write relationship
//         {
//             let mut mmap = create_mmap(file_path, 4096).unwrap();
//             for i in 0..100 {
//                 let relationship = RelationshipRecord {
//                     source_id: i,
//                     target_id: i + 1,
//                     first_property_offset: i * 2,
//                     next_relationship_offset: i * 3,
//                 };
//                 relationship.write(&mut mmap, i as usize * RelationshipRecord::record_size()).unwrap();
//             }
//             // mmap will be flushed and dropped here
//         }
//         // Create file and write relationship
//         {
//             let mut mmap = create_mmap(file_path, 4096).unwrap();
//             for i in 100..104 {
//                 let relationship = RelationshipRecord {
//                     source_id: i,
//                     target_id: i + 1,
//                     first_property_offset: i * 2,
//                     next_relationship_offset: i * 3,
//                 };
//                 relationship.write(&mut mmap, i as usize * RelationshipRecord::record_size()).unwrap();
//             }
//             // mmap will be flushed and dropped here
//         }
        

//         // Read relationship
//         {
//             let file = OpenOptions::new()
//                 .read(true)
//                 .write(true)
//                 .open(file_path)
//                 .unwrap();
//             let mmap = unsafe { MmapOptions::new().map_mut(&file).unwrap() };
//             for i in 0..104 {
//                 let relationship = RelationshipRecord::read(&mmap, i as usize * RelationshipRecord::record_size()).unwrap();
//                 assert_eq!(relationship.source_id, i);
//                 assert_eq!(relationship.target_id, i + 1);
//                 assert_eq!(relationship.first_property_offset, i * 2);
//                 assert_eq!(relationship.next_relationship_offset, i * 3);
//             }
//         }
//     }
// }
