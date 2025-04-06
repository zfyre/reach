use super::models::{Content, HistoryEntry, NewHistoryEntry};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_history(
    conn: &mut PgConnection,
    session_id: i32,
    level: i32,
    content: &Content,
    level_entry_counts: &mut [usize],
) -> HistoryEntry {
    use super::schema::rchat::history;

    let new_history = NewHistoryEntry {
        session_id,
        level,
        content,
    };

    let entry = diesel::insert_into(history::table)
        .values(&new_history)
        .returning(HistoryEntry::as_returning())
        .get_result(conn)
        .expect("Error saving new history");
    
    // Increment the level entry count for the current level
    level_entry_counts[level as usize] += 1;

    entry

    /* If we want to just save the new entries without returning the added entries */
    // diesel::insert_into(history::table)
    //     .values(&new_history)
    //     .execute(conn)
    //     .expect("Error saving new history");
}

#[allow(dead_code)]
fn create_histories(
    conn: &mut PgConnection,
    new_histories: &[NewHistoryEntry],
) -> Vec<HistoryEntry> {
    use super::schema::rchat::history;

    diesel::insert_into(history::table)
        .values(new_histories)
        .returning(HistoryEntry::as_returning())
        .get_results(conn)
        .expect("Error saving new histories")
}

#[allow(dead_code)]
fn get_history_by_key(
    conn: &mut PgConnection,
    sess_id: i32,
    msg_id: i32,
    lvl: i32,
) -> Option<HistoryEntry> {
    use super::schema::rchat::history;

    let chat_hist = history::table
        .find((sess_id, msg_id, lvl))
        .first::<HistoryEntry>(conn)
        .optional()
        .expect("Error loading history");

    chat_hist
}

pub fn get_history_by_level(
    conn: &mut PgConnection,
    sess_id: i32,
    lvl: i32,
    chunk_size: usize,
) -> Vec<HistoryEntry> {
    use super::schema::rchat::history;

    let chat_hist = history::table
        .filter(history::session_id.eq(sess_id))
        .filter(history::level.eq(lvl))
        .order(history::message_id.desc())
        .limit(chunk_size as i64)
        .load::<HistoryEntry>(conn)
        .expect("Error loading history");

    chat_hist
}

#[allow(dead_code)]
fn get_history_by_session(
    conn: &mut PgConnection,
    sess_id: i32,
    chunk_size: usize,
) -> Vec<HistoryEntry> {
    use super::schema::rchat::history;

    let chat_hist = history::table
        .filter(history::session_id.eq(sess_id))
        .limit(chunk_size as i64)
        .load::<HistoryEntry>(conn)
        .expect("Error loading history");

    chat_hist
}

#[allow(dead_code)]
fn update_history(
    conn: &mut PgConnection,
    sess_id: i32,
    msg_id: i32,
    lvl: i32,
    new_content: &Content,
) -> HistoryEntry {
    use super::schema::rchat::history;

    let updated_history = diesel::update(history::table.find((sess_id, msg_id, lvl)))
        .set(history::content.eq(new_content))
        .returning(HistoryEntry::as_returning())
        .get_result(conn)
        .expect("Error updating history");

    updated_history
}

#[allow(dead_code)]
fn update_history_with_level(
    // Updates the level to level + 1
    conn: &mut PgConnection,
    sess_id: i32,
    msg_id: i32,
    lvl: i32,
    new_content: &Content,
) -> HistoryEntry {
    use super::schema::rchat::history;

    let updated_history = diesel::update(history::table.find((sess_id, msg_id, lvl)))
        .set((history::content.eq(new_content), history::level.eq(lvl + 1)))
        .returning(HistoryEntry::as_returning())
        .get_result(conn)
        .expect("Error updating history");

    updated_history
}

pub fn get_num_history_entries(conn: &mut PgConnection, sess_id: i32, lvl: i32) -> i64 {
    use super::schema::rchat::history;

    let num_entries = history::table
        .filter(history::session_id.eq(sess_id))
        .filter(history::level.eq(lvl))
        .count()
        .get_result(conn)
        .expect("Error getting number of history entries");

    num_entries
}


pub fn delete_histories_by_level(
    conn: &mut PgConnection,
    sess_id: i32,
    entries: &[HistoryEntry]
) -> () 
{
    use super::schema::rchat::history;

    let msg_ids: Vec<i32> = entries.iter().map(|entry| entry.message_id).collect();
    let levels: Vec<i32> = entries.iter().map(|entry| entry.level).collect();

    diesel::delete(history::table.filter(history::session_id.eq(sess_id))
        .filter(history::message_id.eq_any(&msg_ids))
        .filter(history::level.eq_any(&levels))
    )
    .execute(conn)
    .expect("Error deleting history");

}