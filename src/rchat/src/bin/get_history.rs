use diesel::prelude::*;
use rchat::diesel_api::*;
use rchat::models::HistoryEntry;
use rchat::*;

fn main() {
    use self::schema::rchat::history::dsl::history;

    let connection = &mut establish_connection();

    let hist = history
        .find((69, 1, 69))
        .select(HistoryEntry::as_select())
        // .first attempts to load a single record from the database
        .first::<HistoryEntry>(connection)
        .optional();

    match hist {
        Ok(Some(entry)) => println!("Found history entry {:?}", entry),
        Ok(None) => println!("Could not find history entry"),
        Err(e) => println!("Error: {:?}", e),
    }
}
