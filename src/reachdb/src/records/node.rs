use bincode;
use memmap2::MmapMut;
use serde::{Deserialize, Serialize};

use super::{ReachdbError, Record, NULL_OFFSET};


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NodeRecord {
    pub id: u64,
    pub first_relationship_id: u64,
    pub first_property_id: u64,
}

impl NodeRecord {
    pub fn new(id: u64, property_id: u64) -> Self {
        Self {
            id,
            first_relationship_id: NULL_OFFSET,
            first_property_id: property_id,
        }
    }
    pub fn update(
        &mut self,
        first_relationship_id: Option<u64>,
        first_property_id: Option<u64>,
    ) {
        self.first_relationship_id = first_relationship_id.unwrap_or(self.first_relationship_id);
        self.first_property_id = first_property_id.unwrap_or(self.first_property_id);
    } 
}


impl Record for NodeRecord {
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
                    first_relationship_id: i*2,
                    first_property_id: i*i,
                };
                node.write(&mut mmap, i).unwrap();
            }
            // mmap will be flushed and dropped here
        }

        // Create file and write node to a different offset
        {
            let mut mmap = create_mmap(file_path, 4096).unwrap();
            for i in 100..105 {
                let node = super::NodeRecord {
                    id: i,
                    first_relationship_id: i*2,
                    first_property_id: i*i,
                };
                node.write(&mut mmap, i).unwrap();
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
                let read_node = NodeRecord::read(&mmap, i).unwrap();

                assert_eq!(read_node.id, i);
                assert_eq!(read_node.first_relationship_id, i*2);
                assert_eq!(read_node.first_property_id, i*i);
            }
        }

        // Clean up
        // std::fs::remove_file(file_path).unwrap();
    }
}