use diesel::{dsl::sql, prelude::*, sql_types::Bool};
use rchat::diesel_api::*;
use rchat::{models::HistoryEntry, *};

fn main() {
    use self::schema::rchat::history::dsl::*;

    let target = "zfyre";
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();

    // let deleted = diesel::delete(history.filter(content.like(pattern)))
    //     .execute(connection)
    //     .expect("Error deleting history");

    let his_to_delete = history
        .filter(sql::<Bool>(&format!(
            r#"(content).user LIKE '{}'"#,
            pattern
        )))
        .select(HistoryEntry::as_select())
        .load::<HistoryEntry>(connection)
        .expect("Error loading history");

    println!("Displaying {} history", his_to_delete.len());
    for entry in his_to_delete {
        println!("{:#?}", entry);
    }

    let deleted = diesel::delete(history.filter(sql::<Bool>(&format!(
        r#"(content).user LIKE '{}'"#,
        pattern
    ))))
    .execute(connection)
    .expect("Error deleting history");

    println!("Deleted {} history entries", deleted);
}
