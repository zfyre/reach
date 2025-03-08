use bincode;
use memmap2::MmapMut;
use serde::{Deserialize, Serialize};

use super::{ReachdbError, Record, NULL_OFFSET};


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NodeRecord {
    pub id: u64,
    pub first_relationship_offset: u64,
    pub first_property_offset: u64,
}

// /// Create a db instance for storing the nodes-str to id and also the count of nodes
// impl NodeRecord {
//     fn assign_id(db_path: &str, node_name: &str) -> Result<u64, ReachdbError> {
        
//         let db = sled::open(db_path)?;

//         // Check if the String is already mapped
//         if let Some(id_bytes) = db.get(node_name)? {
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
//     pub fn new(db_path: &str, node_name: &str) -> Result<Self, ReachdbError> {
//         let id = Self::assign_id(db_path, node_name)?;
//         Ok(Self {
//             id,
//             first_relationship_offset: NULL_OFFSET,
//             first_property_offset: NULL_OFFSET,
//         })
//     }
// }


impl Record for NodeRecord {
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
        let end = self.id as usize * Self::record_size() + encoded.len();

        mmap[offset..end].copy_from_slice(&encoded);
        mmap.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::records::{node::NodeRecord, Record};

    #[test]
    fn writing_node_records() {
        
        use crate::utils::create_mmap;
        use memmap2::MmapOptions;
        use std::fs::OpenOptions;

        let file_path = "reachdb.node.db";

        // Create file and write node
        {
            let mut mmap = create_mmap(file_path, 4096).unwrap();
                for i in 0..100 {
                    let node = super::NodeRecord {
                        id: i,
                        first_relationship_offset: i*2,
                        first_property_offset: i*i,
                    };
                    let offset = i as usize * NodeRecord::record_size();
                    node.write(&mut mmap, offset).unwrap();
                }
            // mmap will be flushed and dropped here
        }

        // Create file and write node to a different offset
        {
            let mut mmap = create_mmap(file_path, 4096).unwrap();
                for i in 100..105 {
                    let node = super::NodeRecord {
                        id: i,
                        first_relationship_offset: i*2,
                        first_property_offset: i*i,
                    };
                    let offset = i as usize * NodeRecord::record_size();
                    node.write(&mut mmap, offset).unwrap();
                }
            // mmap will be flushed and dropped here
        }

        // Reopen file and read node
        {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(file_path)
                .unwrap();
            let mmap = unsafe { MmapOptions::new()
                    .map_mut(&file)
                    .unwrap()
            };
            for i in 0..105 {
                let offset = i as usize * NodeRecord::record_size();
                let read_node = NodeRecord::read(&mmap, offset).unwrap();

                assert_eq!(read_node.id, i);
                assert_eq!(read_node.first_relationship_offset, i*2);
                assert_eq!(read_node.first_property_offset, i*i);
            }
        }

        // Clean up
        // std::fs::remove_file(file_path).unwrap();
    }
}