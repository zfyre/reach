use rchat::diesel_api::*;
use rchat::models::Content;
use std::vec;

fn main() {
    let connection = &mut establish_connection();

    let session_id = 0;
    let level = 0;
    let content = Content::new(
        "Hi from zfyre!".to_string(),
        "Hello there!, How can I help you?".to_string(),
        vec![],
    );

    let history = create_history(connection, session_id, level, &content);
    println!("Saved history entry {:?}", history);
}
