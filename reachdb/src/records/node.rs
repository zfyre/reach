use bincode;
use memmap2::MmapMut;
use serde::{Deserialize, Serialize};

use super::{Record, ReachdbError};


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NodeRecord {
    pub id: u64,
    pub first_relationship_offset: u64,
    pub first_property_offset: u64,
}

impl NodeRecord {
    
}

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