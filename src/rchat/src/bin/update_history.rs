use diesel::prelude::*;
use rchat::models::{Content, HistoryEntry};
use rchat::*;

fn main() {
    use self::schema::rchat::history::dsl::{content, history, level, session_id}; // Keep it under the fundtional to prevent polluting the module namespace

    let connection = &mut establish_connection();

    let sess_id = 69;
    let msg_id = 1;
    let level_id = 0;

    // find() is based up of primary key
    let updated_history = diesel::update(history.find((sess_id, msg_id, level_id)))
        .set((
            level.eq(0),
            session_id.eq(69),
            content.eq(Content::new(
                "Updated User Question".to_string(),
                "Updated system Response".to_string(),
            )),
        ))
        .returning(HistoryEntry::as_returning())
        .get_result(connection)
        .expect("Error updating history");

    println!("Updated history entry {:?}", updated_history);
}
