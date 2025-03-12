
use std::{env, fs::File, io::Read};
use log::trace;
use serde_json::Value;
use reachdb::{{Reachdb, UserDefinedRelationType}, ReachdbError, records::NULL_OFFSET};

fn get_data() -> Result<Value, serde_json::Error> {
    let mut f = File::open("tempdata/c.json")
        .expect("Could not open file");
    let mut buf = String::new();
    let _ = f.read_to_string(&mut buf);

    serde_json::from_str::<Value>(&buf)
}

#[derive(Debug)]
enum TypeId {
    IsA(u8),
    RelatesTo(u8),
    Influences(u8)
}
impl UserDefinedRelationType for TypeId {
    fn get_type_id(relation: &str) -> Option<Self> {
        match relation {
            "IS-A" => Some(Self::IsA(0)),
            "RELATES-TO" => Some(Self::RelatesTo(1)),
            "INFLUENCES" => Some(Self::Influences(2)),
            _ => None
        }
    }
    fn type_id(&self) -> u8 {
        match self {
            Self::IsA(id) => *id,
            Self::RelatesTo(id) => *id,
            Self::Influences(id) => *id,
        }
    }
}
impl TypeId {
    fn from_id(id: u8) -> Self {
        match id {
            0 => Self::IsA(id),
            1 => Self::RelatesTo(id),
            2 => Self::Influences(id),
            _ => panic!("Invalid type id")
        }
    }
}

fn main() -> Result<(), ReachdbError> {

    unsafe {
        // env::set_var("RUST_LOG", "reachdb=trace");
        env::set_var("RUST_LOG", "reachdb=info");
    }
    let _ = env_logger::try_init();
    trace!("NULL_OFFSET: {}", NULL_OFFSET);
    
    // let mut db = Reachdb::<TypeId>::new()?;
    // db.prepare(Some(10000), Some(10000))?;
    let mut db = Reachdb::<TypeId>::open("data", Some(60000), Some(60000))?;

    // let data = get_data().unwrap();
    // // db.print_graph()?;
    // for (url, edges) in data.as_object().unwrap() {
    //     trace!("url: {}", url);
    //     for edge in edges.as_array().unwrap() {
    //         let source = edge["source"].as_str().unwrap();
    //         let target = edge["target"].as_str().unwrap();
    //         let relationship = edge["relationship"].as_str().unwrap();
    //         // println!("{} - {} -> {}", source, relationship, target);
    //         db.add_edge(source, target, relationship)?;            
    //     }
    // }

    // let path = db.random_walk(0, 10)?;
    // for rel_id in path {
    //     let rel = db.get_relation(rel_id)?;
    //     println!("{:#?}[{:#?}] -> {:#?}", db.get_property(rel.source_id)?, TypeId::from_id(rel.type_id), db.get_property(rel.target_id)?);
    // }

    println!("{}", "-----------------".repeat(5));
    for i in 0..10 {
        let node = db.get_all_node_relations(i)?;
        for rel_id in node {
            let rel = db.get_relation(rel_id)?;
            println!("{:#?}[{:?}] -> {:#?}", db.get_property(rel.source_id)?, TypeId::from_id(rel.type_id), db.get_property(rel.target_id)?);
        }
        println!("{}", "-----------------".repeat(5));
    }
    db.close()?;    

    Ok(())
}

// mod tests {
//     use super::*;

//     #[test]
//     fn test_reachdb_generation() {
//         let _ = env_logger::try_init();
//         let _ = std::fs::remove_dir_all("data");
//         let mut db = Reachdb::<TypeId>::open("data", Some(10000), Some(10000)).unwrap();
//         let data = get_data().unwrap();
//         for (url, edges) in data.as_object().unwrap() {
//             trace!("url: {}", url);
//             for edge in edges.as_array().unwrap() {
//                 let source = edge["source"].as_str().unwrap();
//                 let target = edge["target"].as_str().unwrap();
//                 let relationship = edge["relationship"].as_str().unwrap();
//                 db.add_edge(source, target, relationship).unwrap();            
//             }
//         }
//         db.close().unwrap();
//     }

//     #[test]
//     fn test_reachdb_random_walk() {
//         let _ = env_logger::try_init();
//         let mut db = Reachdb::<TypeId>::open("data", Some(10000), Some(10000)).unwrap();
//         let path = db.random_walk(0, 10).unwrap();
//         for rel_id in path {
//             let rel = db.get_relation(rel_id).unwrap();
//             println!("{} --> {}: {:?}", rel.source_id, rel.target_id, TypeId::from_id(rel.type_id));
//         }
//         db.close().unwrap();
//     }
// }