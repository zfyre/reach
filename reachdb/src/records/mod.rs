pub mod node;
pub mod relationship;

use super::{
    utils::create_mmap,
    errors::ReachdbError,
};
use memmap2::MmapMut;

// Define a constant to represent a null offset for node & relationship records
pub const NULL_OFFSET: u64 = u64::MAX;

// Trait for a record that can be written to a memory-mapped file
pub trait Record {
    fn write(&self, mmap: &mut MmapMut, id: u64) -> Result<(), ReachdbError>;
    fn read(mmap: &MmapMut, id: u64) -> Result<Self, ReachdbError>
    where
        Self: Sized;

    fn record_size() -> usize 
    where
        Self: Sized,
    {
        std::mem::size_of::<Self>()
    }
    fn id2offset(id: u64) -> usize;
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tempfile::NamedTempFile;
//     // use std::io::Write;

//     /// Helper function to create a temporary memory-mapped file of a given size.
//     /// **Use-case:** Allows tests to operate on temporary files that are automatically cleaned up.
//     fn create_temp_mmap(size: usize) -> (NamedTempFile, MmapMut) {
//         let tmpfile = NamedTempFile::new().expect("Failed to create temp file");
//         let path = tmpfile.path().to_str().unwrap().to_string();
//         let mmap = create_mmap(&path, size);
//         (tmpfile, mmap.unwrap())
//     }

//     #[test]
//     fn test_persistent_storage_with_local_file() {
//         let file_path = "reachdb.db";

//         // Create file and write node
//         {
//             let mut mmap = create_mmap(file_path, 4096).unwrap();
//                 for i in 0..100 {
//                     let node = NodeRecord {
//                         id: i,
//                     first_relationship_offset: i*2,
//                     first_property_offset: i*i,
//                 };
//                 write_node(&mut mmap, i as usize * NODE_RECORD_SIZE, &node).unwrap();
//             }
//             // mmap will be flushed and dropped here
//         }

//         // Reopen file and read node
//         {
//             let file = OpenOptions::new()
//                 .read(true)
//                 .write(true)
//                 .open(file_path)
//                 .unwrap();
//             let mmap = unsafe { MmapOptions::new()
//                     .map_mut(&file)
//                     .unwrap()
//             };
//             for i in 0..100 {
//                 let read_node = read_node(&mmap, i as usize * NODE_RECORD_SIZE).unwrap();

//                 assert_eq!(read_node.id, i);
//                 assert_eq!(read_node.first_relationship_offset, i*2);
//                 assert_eq!(read_node.first_property_offset, i*i);
//             }
//         }

//         // Clean up
//         // std::fs::remove_file(file_path).unwrap();
//     }

//     #[test]
//     fn test_node_and_relationship_size() {
//         let node = NodeRecord {
//             id: 42,
//             first_relationship_offset: NULL_OFFSET,
//             first_property_offset: NULL_OFFSET,
//         };
//         let rel = RelationshipRecord {
//             source_id: 42,
//             target_id: 84,
//             first_property_offset: NULL_OFFSET,
//             next_relationship_offset: NULL_OFFSET,
//         };
//         assert_eq!(std::mem::size_of_val(&node), NODE_RECORD_SIZE);
//         assert_eq!(std::mem::size_of_val(&rel), RELATIONSHIP_RECORD_SIZE);
//     }

//     #[test]
//     fn test_write_read_node() {
//         let (_temp_file, mut mmap) = create_temp_mmap(1024);
//         let node = NodeRecord {
//             id: 42,
//             first_relationship_offset: NULL_OFFSET,
//             first_property_offset: NULL_OFFSET,
//         };
//         let offset = 0;
//         let _ = write_node(&mut mmap, offset, &node);
//         let read_back = read_node(&mmap, offset);
//         assert_eq!(node, read_back.unwrap());
//     }

//     #[test]
//     fn test_write_read_relationship() {
//         let (_temp_file, mut mmap) = create_temp_mmap(1024);
//         let rel = RelationshipRecord {
//             source_id: 42,
//             target_id: 84,
//             first_property_offset: NULL_OFFSET,
//             next_relationship_offset: NULL_OFFSET,
//         };
//         let offset = 0;
//         let _ = write_relationship(&mut mmap, offset, &rel);
//         let read_back = read_relationship(&mmap, offset);
//         assert_eq!(rel, read_back.unwrap());
//     }

//     #[test]
//     fn test_traverse_relationships() {
//         // Create a temporary mmap for relationships.
//         let (_temp_file, mut mmap) = create_temp_mmap(2048);

//         // Create three RelationshipRecords and chain them:
//         // rel1 -> rel2 -> rel3
//         let rel1 = RelationshipRecord {
//             source_id: 1,
//             target_id: 2,
//             first_property_offset: NULL_OFFSET,
//             next_relationship_offset: 512,  // pointing to where rel2 will be stored
//         };
//         let rel2 = RelationshipRecord {
//             source_id: 1,
//             target_id: 3,
//             first_property_offset: NULL_OFFSET,
//             next_relationship_offset: 1024, // pointing to where rel3 will be stored
//         };
//         let rel3 = RelationshipRecord {
//             source_id: 1,
//             target_id: 4,
//             first_property_offset: NULL_OFFSET,
//             next_relationship_offset: NULL_OFFSET, // end of chain
//         };

//         // Write the relationships at predetermined offsets.
//         let _ = write_relationship(&mut mmap, 0, &rel1);
//         let _ = write_relationship(&mut mmap, 512, &rel2);
//         let _ = write_relationship(&mut mmap, 1024, &rel3);

//         // Traverse starting from offset 0.
//         let rels = traverse_relationships(&mmap, 0);
//         assert_eq!(rels.len(), 3);
//         assert_eq!(rels[0].target_id, 2);
//         assert_eq!(rels[1].target_id, 3);
//         assert_eq!(rels[2].target_id, 4);
//     }

//     #[test]
//     fn test_pick_random_relationship() {
//         // Create a sample vector of relationships.
//         let relationships = vec![
//             RelationshipRecord { source_id: 1, target_id: 2, first_property_offset: NULL_OFFSET, next_relationship_offset: NULL_OFFSET },
//             RelationshipRecord { source_id: 1, target_id: 3, first_property_offset: NULL_OFFSET, next_relationship_offset: NULL_OFFSET },
//             RelationshipRecord { source_id: 1, target_id: 4, first_property_offset: NULL_OFFSET, next_relationship_offset: NULL_OFFSET },
//         ];
//         // Run the random selection multiple times.
//         for _ in 0..10 {
//             let chosen = pick_random_relationship(&relationships);
//             assert!(chosen.is_some());
//             let rel = chosen.unwrap();
//             assert!(rel.target_id == 2 || rel.target_id == 3 || rel.target_id == 4);
//         }
//     }
// }
