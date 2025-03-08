use bincode;
use memmap2::MmapMut;
use serde::{Deserialize, Serialize};

use super::{ReachdbError, Record, NULL_OFFSET};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RelationshipRecord {
    pub source_id: u64,
    pub target_id: u64,   
    pub type_id: u64,
    pub first_property_offset: u64,
    next_src_relationship_offset: u64,
    prev_src_relationship_offset: u64,
    next_tgt_relationship_offset: u64,
    prev_tgt_relationship_offset: u64,
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
        id as usize
    }
}

impl RelationshipRecord {
    pub fn new(
        source_id: u64,
        target_id: u64,
        type_id: u64,
        first_property_offset: Option<u64>,
        next_src_relationship_offset: Option<u64>,
        prev_src_relationship_offset: Option<u64>,
        next_tgt_relationship_offset: Option<u64>,
        prev_tgt_relationship_offset: Option<u64>,
        ) -> Self {
        Self {
            source_id,
            target_id,
            type_id,
            first_property_offset: first_property_offset.unwrap_or(NULL_OFFSET),
            next_src_relationship_offset: next_src_relationship_offset.unwrap_or(NULL_OFFSET),
            prev_src_relationship_offset: prev_src_relationship_offset.unwrap_or(NULL_OFFSET),
            next_tgt_relationship_offset: next_tgt_relationship_offset.unwrap_or(NULL_OFFSET),
            prev_tgt_relationship_offset: prev_tgt_relationship_offset.unwrap_or(NULL_OFFSET),
        }
    }
    
    pub fn update(
        &mut self,
        // source_id: Option<u64>,
        // target_id: Option<u64>,
        first_property_offset: Option<u64>,
        next_src_relationship_offset: Option<u64>,
        prev_src_relationship_offset: Option<u64>,
        next_tgt_relationship_offset: Option<u64>,
        prev_tgt_relationship_offset: Option<u64>,
    ) {
        // self.source_id = source_id.unwrap_or(self.source_id);
        // self.target_id = target_id.unwrap_or(self.target_id);
        // self.type_id = type_id.unwrap_or(self.type_id);
        self.first_property_offset = first_property_offset.unwrap_or(self.first_property_offset);
        self.next_src_relationship_offset = next_src_relationship_offset.unwrap_or(self.next_src_relationship_offset);
        self.prev_src_relationship_offset = prev_src_relationship_offset.unwrap_or(self.prev_src_relationship_offset);
        self.next_tgt_relationship_offset = next_tgt_relationship_offset.unwrap_or(self.next_tgt_relationship_offset);
        self.prev_tgt_relationship_offset = prev_tgt_relationship_offset.unwrap_or(self.prev_tgt_relationship_offset);
    }

    /// Initializes an iterator externally by providing `current_offset` and `mmap`
    pub fn into_source_iter<'a>(mmap: &'a MmapMut, current_offset: u64) -> RelationshipSourceIterator<'a> {
        RelationshipSourceIterator {
            current_offset,
            mmap,
        }
    }
    /// Initializes an iterator externally by providing `current_offset` and `mmap`
    pub fn into_target_iter<'a>(mmap: &'a MmapMut, current_offset: u64) -> RelationshipTargetIterator<'a> {
        RelationshipTargetIterator {
            current_offset,
            mmap,
        }
    }
}

pub struct RelationshipSourceIterator<'a> {
    current_offset: u64,
    mmap: &'a MmapMut,
}

impl<'a> Iterator for RelationshipSourceIterator<'a> {
    type Item = Result<RelationshipRecord, ReachdbError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_offset == NULL_OFFSET {
            return None;
        }

        // Read the current record
        match RelationshipRecord::read(self.mmap, self.current_offset) {
            Ok(record) => {
                // First try to go to prev relationships, if that's NULL,
                // switch to next relationships
                if record.prev_src_relationship_offset != NULL_OFFSET {
                    self.current_offset = record.prev_src_relationship_offset;
                } else {
                    self.current_offset = record.next_src_relationship_offset;
                }
                Some(Ok(record))
            }
            Err(e) => {
                // Return the error and stop iteration
                self.current_offset = NULL_OFFSET;
                Some(Err(e))
            }
        }
    }
}

pub struct RelationshipTargetIterator<'a> {
    current_offset: u64,
    mmap: &'a MmapMut,
}

impl<'a> Iterator for RelationshipTargetIterator<'a> {
    type Item = Result<RelationshipRecord, ReachdbError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_offset == NULL_OFFSET {
            return None;
        }

        // Read the current record
        match RelationshipRecord::read(self.mmap, self.current_offset) {
            Ok(record) => {
                // First try to go to prev relationships, if that's NULL,
                // switch to next relationships
                if record.prev_tgt_relationship_offset != NULL_OFFSET {
                    self.current_offset = record.prev_tgt_relationship_offset;
                } else {
                    self.current_offset = record.next_tgt_relationship_offset;
                }
                Some(Ok(record))
            }
            Err(e) => {
                // Return the error and stop iteration
                self.current_offset = NULL_OFFSET;
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
