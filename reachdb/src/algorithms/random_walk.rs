use super::{
    RelationshipRecord,
    NULL_OFFSET,
};
use crate::records::Record;
use memmap2::MmapMut;
use rand::Rng; 


/// Traverses the linked list of relationships starting at `start_offset` in the relationships file.
/// Returns a vector of RelationshipRecords.
///
/// Use-case: When performing operations like random walks, this function loads all relationships
/// for a given node by following the next_relationship_offset pointers.
pub fn traverse_relationships(mmap: &MmapMut, start_offset: usize) -> Vec<RelationshipRecord> {
    let mut relationships = Vec::new();
    let mut offset = start_offset as u64;
    // Continue traversing until the offset equals NULL_OFFSET.
    while offset != NULL_OFFSET {
        let end = offset as usize + RelationshipRecord::record_size();
        let data = &mmap[offset as usize..end];
        let rel: RelationshipRecord = bincode::deserialize(data).expect("Deserialization failed");
        offset = rel.next_relationship_offset;
        relationships.push(rel);
    }
    relationships
}

/// Picks a random relationship from the provided vector of RelationshipRecords.
/// Returns None if the vector is empty.
///
/// Use-case: For a random walk, after traversing a node's relationships, randomly select one edge.
pub fn pick_random_relationship(relationships: &[RelationshipRecord]) -> Option<&RelationshipRecord> {
    if relationships.is_empty() {
        None
    } else {
        let index = rand::thread_rng().gen_range(0..relationships.len());
        relationships.get(index)
    }
}