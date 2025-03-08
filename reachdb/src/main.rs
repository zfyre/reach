
use std::{env, fs::File, io::Read};
use log::{info, trace, debug, error, warn};
use serde_json::Value;
use reachdb::{data_base::Reachdb, errors::ReachdbError};

fn get_data() -> Result<Value, serde_json::Error> {
    let mut f = File::open("data/knowledge_graph.json")
        .expect("Could not open file");
    let mut buf = String::new();
    let _ = f.read_to_string(&mut buf);

    serde_json::from_str::<Value>(&buf)
}

fn main() -> Result<(), ReachdbError> {

    unsafe {
        env::set_var("RUST_LOG", "reachdb=trace");
    }
    let _ = env_logger::try_init();
    
    
    let mut db = Reachdb::new()?;

     let data = get_data().unwrap();

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

    Ok(())
}