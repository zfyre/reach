use bincode;
use memmap2::MmapMut;
use serde::{Deserialize, Serialize};

use super::{Record, ReachdbError};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RelationshipRecord {
    pub id: u64,
    pub source_id: u64,
    pub target_id: u64,
    pub first_property_offset: u64,
    pub next_relationship_offset: u64,
}

impl Record for RelationshipRecord {
    fn read(mmap: &MmapMut, id: u64) -> Result<Self, ReachdbError>
    where
        Self: Sized,
    {
        let offset = id as usize * Self::record_size();
        
        let end = offset + Self::record_size();
        let data = &mmap[offset..end];
        Ok(bincode::deserialize(data)?)
    }
    fn write(&self, mmap: &mut MmapMut) -> Result<(), ReachdbError> {
        let offset = self.id as usize * Self::record_size();

        let encoded = bincode::serialize(self)?;
        let end = offset + encoded.len();
        mmap[offset..end].copy_from_slice(&encoded);
        mmap.flush()?;

        Ok(())
    }
}
