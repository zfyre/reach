use diesel::prelude::*;
use rchat::diesel_api::*;
use rchat::models::*;
use rchat::*;

// #[macro_use]
// extern crate diesel;

fn main() {
    use self::schema::rchat::history::dsl::*; // Keep it under the fundtional to prevent polluting the module namespace

    let connection = &mut establish_connection();

    let results = history
        .filter(level.eq(0))
        .limit(5)
        .select(HistoryEntry::as_select())
        .load::<HistoryEntry>(connection)
        .expect("Error loading posts");

    println!("Displaying {} history", results.len());
    for entry in results {
        println!("{:#?}", entry);
    }
}
