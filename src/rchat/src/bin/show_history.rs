use rchat::models::*;
use diesel::prelude::*;
use rchat::*;

// #[macro_use]
// extern crate diesel;

fn main() {
    use self::schema::rchat::history::dsl::*;

    let connection = &mut establish_connection();

    let results = history
        .filter(level.eq(0))
        .limit(5)
        .select(HistoryEntry::as_select())
        .load::<HistoryEntry>(connection)
        .expect("Error loading posts");

    println!("Displaying {} history", results.len());
    for entry in results { 
        println!("{}", entry.session_id);
        println!("{}", entry.message_id);
        println!("{}", entry.level);
        println!("-----------\n");
        println!("{:#?}", entry.content);
        println!("{:#?}", entry.tags);
    }
}