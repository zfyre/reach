use diesel::prelude::*;
use dotenvy::dotenv;
use std::env; 
pub mod models;
pub mod schema;

// TODO: Use the dotenv for the reachdb database URL

// pub fn establish_connection() -> SqliteConnection {
//     dotenv().ok();

//     let database_url = env::var("RCHAT_DATABASE_URL").expect("DATABASE_URL must be set");
//     SqliteConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

