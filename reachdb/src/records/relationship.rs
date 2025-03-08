use bincode;
use memmap2::MmapMut;
use serde::{Deserialize, Serialize};

use super::{Record, ReachdbError};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RelationshipRecord {
    pub source_id: u64,
    pub target_id: u64,
    pub first_property_offset: u64,
    pub next_relationship_offset: u64,
}

// impl RelationshipRecord {
//     pub fn get_id(&self) -> String {
//         format!("{} -> {}", self.source_id, self.target_id)
//     }
//     pub fn get_relation(&self, db_path: &str) -> Result<String, ReachdbError> {
//         let id = self.get_id();

//         let db = sled::open(db_path)?;

//         // Check if the String is already mapped
//         if let Some(id_bytes) = db.get(id)? {
//             let id = bincode::deserialize::<u64>(&id_bytes)?;
//             return Ok(id);
//         }

//         // Reterieve and update the counter
//         let counter_key = "$COUNTER";
//         let new_id = match db.get(counter_key)? {
//             Some(value) => {
//                 let current_id = bincode::deserialize::<u64>(&value)?;
//                 current_id + 1
//             },
//             None => 0,
//         };

//         // Insert the mapping: string -> new_id, and update the counter.
//         db.insert(node_name, bincode::serialize(&new_id)?)?;
//         db.insert(counter_key, bincode::serialize(&new_id)?)?;
//         db.flush()?; // Ensure data is persisted
        
//         Ok(new_id)
//     }
//     pub fn new(source_id: u64, target_id: u64) -> Self {
//         Self {
//             source_id,
//             target_id,
//             first_property_offset: 0,
//             next_relationship_offset: 0,
//         }
//     }
// }

impl Record for RelationshipRecord {
    fn read(mmap: &MmapMut, offset: usize) -> Result<Self, ReachdbError>
    where
        Self: Sized,
    {
        let end = offset + Self::record_size();
        let data = &mmap[offset..end];
        Ok(bincode::deserialize(data)?)
    }
    fn write(&self, mmap: &mut MmapMut, offset: usize) -> Result<(), ReachdbError> {
        let encoded = bincode::serialize(self)?;
        let end = offset + encoded.len();
        mmap[offset..end].copy_from_slice(&encoded);
        mmap.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::records::{relationship::RelationshipRecord, Record};

    #[test]
    fn writing_relationship_records() {
        use crate::utils::create_mmap;
        use memmap2::MmapOptions;
        use std::fs::OpenOptions;

        let file_path = "reachdb.relationship.db";

        // Create file and write relationship
        {
            let mut mmap = create_mmap(file_path, 4096).unwrap();
            for i in 0..100 {
                let relationship = RelationshipRecord {
                    source_id: i,
                    target_id: i + 1,
                    first_property_offset: i * 2,
                    next_relationship_offset: i * 3,
                };
                relationship.write(&mut mmap, i as usize * RelationshipRecord::record_size()).unwrap();
            }
            // mmap will be flushed and dropped here
        }
        // Create file and write relationship
        {
            let mut mmap = create_mmap(file_path, 4096).unwrap();
            for i in 100..104 {
                let relationship = RelationshipRecord {
                    source_id: i,
                    target_id: i + 1,
                    first_property_offset: i * 2,
                    next_relationship_offset: i * 3,
                };
                relationship.write(&mut mmap, i as usize * RelationshipRecord::record_size()).unwrap();
            }
            // mmap will be flushed and dropped here
        }
        

        // Read relationship
        {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(file_path)
                .unwrap();
            let mmap = unsafe { MmapOptions::new().map_mut(&file).unwrap() };
            for i in 0..104 {
                let relationship = RelationshipRecord::read(&mmap, i as usize * RelationshipRecord::record_size()).unwrap();
                assert_eq!(relationship.source_id, i);
                assert_eq!(relationship.target_id, i + 1);
                assert_eq!(relationship.first_property_offset, i * 2);
                assert_eq!(relationship.next_relationship_offset, i * 3);
            }
        }
    }
}
