
use std::{env, fs::File, io::Read};
use log::{info, trace, debug, error, warn};
use serde_json::Value;
use reachdb::{data_base::{Reachdb, UserDefinedRelationType}, errors::ReachdbError, records::NULL_OFFSET};

fn get_data() -> Result<Value, serde_json::Error> {
    let mut f = File::open("tempdata/knowledge_graph.json")
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

fn main() -> Result<(), ReachdbError> {

    unsafe {
        // env::set_var("RUST_LOG", "reachdb=trace");
        env::set_var("RUST_LOG", "reachdb=info");
    }
    let _ = env_logger::try_init();
    trace!("NULL_OFFSET: {}", NULL_OFFSET);
    
    // let mut db = Reachdb::<TypeId>::new()?;
    // db.prepare(Some(10000), Some(10000))?;
    let mut db = Reachdb::<TypeId>::open("data", Some(10000), Some(10000))?;

    let data = get_data().unwrap();
    // db.print_graph()?;
    for (url, edges) in data.as_object().unwrap() {
        trace!("url: {}", url);
        for edge in edges.as_array().unwrap() {
            let source = edge["source"].as_str().unwrap();
            let target = edge["target"].as_str().unwrap();
            let relationship = edge["relationship"].as_str().unwrap();
            // println!("{} - {} -> {}", source, relationship, target);
            db.add_edge(source, target, relationship)?;            
        }
    }
    // db.print_graph()?;
    db.close()?;    
    Ok(())
}