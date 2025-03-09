use bincode;
use log::trace;
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
    pub fn into_source_iter<'a>(mmap: &'a MmapMut, current_id: u64) -> RelationshipSourceIterator<'a> {
        RelationshipSourceIterator {
            initial_id: current_id,
            current_id,
            visited_prev: false,
            mmap,
        }
    }
    /// Initializes an iterator externally by providing `current_offset` and `mmap`
    pub fn into_target_iter<'a>(mmap: &'a MmapMut, current_id: u64) -> RelationshipTargetIterator<'a> {
        RelationshipTargetIterator {
            current_id,
            mmap,
        }
    }
}

pub struct RelationshipSourceIterator<'a> {
    initial_id: u64,
    current_id: u64,
    visited_prev: bool, // Tracks if we finished iterating in the prev direction
    mmap: &'a MmapMut,
}

impl<'a> Iterator for RelationshipSourceIterator<'a> {
    type Item = Result<RelationshipRecord, ReachdbError>;

    fn next(&mut self) -> Option<Self::Item> {
        trace!("REL-ID: {}", self.current_id);
        
        // If current_id is NULL_OFFSET, check if we should switch to next direction
        if self.current_id == NULL_OFFSET {
            if !self.visited_prev {
                trace!("SWITCHING TO NEXT RELATIONSHIPS FROM INITIAL ID");
                self.visited_prev = true;
                self.current_id = self.initial_id; // Restart from initial_id
            } else {
                return None; // Fully exhausted both directions
            }
        }

        // Read the current record
        match RelationshipRecord::read(self.mmap, self.current_id) {
            Ok(record) => {
                if !self.visited_prev {
                    // Iterate using prev_src_relationship_id first
                    if record.prev_src_relationship_id != NULL_OFFSET {
                        trace!("INSIDE PREV");
                        self.current_id = record.prev_src_relationship_id;
                    } else {
                        // If no more prev links, signal a switch
                        self.current_id = NULL_OFFSET;
                    }
                } else {
                    // Now iterate using next_src_relationship_id
                    if record.next_src_relationship_id != NULL_OFFSET {
                        trace!("INSIDE NEXT");
                        self.current_id = record.next_src_relationship_id;
                    } else {
                        self.current_id = NULL_OFFSET;
                    }
                }
                Some(Ok(record))
            }
            Err(e) => {
                // Return the error and stop iteration
                self.current_id = NULL_OFFSET;
                Some(Err(e))
            }
        }
    }
}


pub struct RelationshipTargetIterator<'a> {
    current_id: u64,
    mmap: &'a MmapMut,
}

impl<'a> Iterator for RelationshipTargetIterator<'a> {
    type Item = Result<RelationshipRecord, ReachdbError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_id == NULL_OFFSET {
            return None;
        }

        // Read the current record
        match RelationshipRecord::read(self.mmap, self.current_id) {
            Ok(record) => {
                // First try to go to prev relationships, if that's NULL,
                // switch to next relationships
                if record.prev_tgt_relationship_id != NULL_OFFSET {
                    self.current_id = record.prev_tgt_relationship_id;
                } else {
                    self.current_id = record.next_tgt_relationship_id;
                }
                Some(Ok(record))
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
