use diesel::prelude::*;
// use serde::{Deserialize, Serialize};
// use serde_json::Value;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{Output, ToSql};
use diesel::{deserialize, serialize};
use diesel::sql_types::{Record, Text, Array};

// ================== Rust Schema for the history table ==================

#[derive(Debug, Clone, FromSqlRow, AsExpression, PartialEq, Eq)]
#[diesel(sql_type=crate::schema::rchat::sql_types::Content)]
pub struct Content {
    pub user: String,
    pub system: String,
    pub tags: Vec<String>,
}

impl Content {
    pub fn new(user: String, system: String, tags: Vec<String>) -> Self {
        Self {user, system, tags}
    }
}

impl ToSql<crate::schema::rchat::sql_types::Content, Pg> for Content {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        serialize::WriteTuple::<(Text, Text, Array<Text>)>::write_tuple(
            &(
                &self.user,
                &self.system,
                &self.tags,
            ),
            &mut out.reborrow(),
        )
    }
}
impl FromSql<crate::schema::rchat::sql_types::Content, Pg> for Content {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let (user, system, tags) =
            FromSql::<Record<(Text, Text, Array<Text>)>, Pg>::from_sql(bytes)?;

        Ok(Content {
            user,
            system,
            tags
        })
    }
}


#[derive(Queryable, Selectable,Debug)]
#[diesel(table_name = crate::schema::rchat::history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HistoryEntry {
    pub session_id: i32,
    pub message_id: i32,
    pub level: i32,
    pub content: Content,  // Use the manually implemented Content struct
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::rchat::history)]
pub struct NewHistoryEntry<'life> {
    pub session_id: i32,
    pub level: i32,
    pub content: &'life Content,
    // #[diesel(sql_type = Array<Text>)]  // Explicitly set the type mapping
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct HistoryContent<'life> {
//     pub user: &'life str,
//     pub system: &'life str,
// }
 