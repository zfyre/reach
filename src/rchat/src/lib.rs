// use diesel::prelude::*;
// use dotenvy::dotenv;
// use std::env; 
pub mod models;
pub mod schema;
pub mod diesel_api;
pub mod chat;
pub mod error;

pub use chat::chat::Chat;
pub use error::RchatError;

// use models::{Content, HistoryEntry};

// TODO: Use the dotenv for the reachdb database URL

// pub fn establish_connection() -> SqliteConnection {
    //     dotenv().ok();
    
    //     let database_url = env::var("RCHAT_DATABASE_URL").expect("DATABASE_URL must be set");
    //     SqliteConnection::establish(&database_url)
    //         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    // }
    
// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();
    
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
//     PgConnection::establish(&database_url)
//     .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }

// pub fn create_history(
//     conn: &mut PgConnection,
//     session_id: i32,
//     level: i32,
//     content: &Content,
//     tags: Vec<Option<&str>>
// ) -> models::HistoryEntry {
//     use schema::rchat::history;

//     let new_history = models::NewHistoryEntry {
//         session_id,
//         level,
//         content,
//         tags,
//     };

//     diesel::insert_into(history::table)
//         .values(&new_history)
//         .returning(HistoryEntry::as_returning())
//         .get_result(conn)
//         .expect("Error saving new history")

//     /* If we want to just save the new entries without returning the added entries */ 
//     // diesel::insert_into(history::table)
//     //     .values(&new_history)
//     //     .execute(conn)
//     //     .expect("Error saving new history");

    
// }

// pub fn create_histories(
//     conn: &mut PgConnection,
//     new_histories: &[models::NewHistoryEntry]
// ) -> Vec<models::HistoryEntry> {
//     use schema::rchat::history;

//     diesel::insert_into(history::table)
//         .values(new_histories)
//         .returning(HistoryEntry::as_returning())
//         .get_results(conn)
//         .expect("Error saving new histories")
// }