use std::env;

use reachdb::{Reachdb, UserDefinedRelationType};
use reach_cli::{errors::ReachError, rsearch::build_kg_iteratively};

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
    fn get_type_str(id: u8) -> Option<String> {
        match id {
            0 => Some("IS-A".to_string()),
            1 => Some("RELATES-TO".to_string()),
            2 => Some("INFLUENCES".to_string()),
            _ => None
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

#[tokio::main]
async fn main() -> Result<(), ReachError> {

    unsafe {
        // env::set_var("RUST_LOG", "reachdb=trace");
        env::set_var("RUST_LOG", "reach=info");
    }
    let _ = env_logger::try_init();

    let mut db = Reachdb::<TypeId>::open("data/iter_test", Some(10000), Some(10000)).unwrap();
    let query = "Learning about Ethereum Blockchain";
    build_kg_iteratively(&mut db, &query, "", 2, 5, 3).await?;
    db.close()?;

    Ok(())
}

// TODO: Refactor the code heavily!! make it more modular and readable.
// TODO: Add a method to dynamically update the size of db in reachdb crate.

// TODO: Sometimes Crawler.py file produces error so implement retry as well either in python only or in rust.
// TODO: Complete the total implementation of the project.

// TODO: Start working on the frontend of the project.
// TODO: Use Box ptr for storing db in reachdb crate.

// TODO: Add Url as a property of a relationship.
// TODO: add db.save() method to save partial data in db.

// TODO: Improve the prompt for generating KG from MD ouput, because sometimes it's not able to generate the KG.

// TODO: Since Website don't have much of knowledge, so next target is to use pdf and arxive pdfs