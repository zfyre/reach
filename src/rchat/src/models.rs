use diesel::prelude::*;
use serde::{Deserialize, Serialize};
// use serde_json::Value;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{Output, ToSql};
use diesel::{deserialize, serialize};
use diesel::sql_types::{Record, Text};

// ================== Rust Schema for the history table ==================


// #[derive(Insertable)]
// #[diesel(table_name = crate::schema::history)]
// pub struct NewHistoryEntry<'life> {
//     pub session_id: i32,
//     pub level: i32,
//     pub content: Value,
//     // #[diesel(sql_type = Array<Text>)]  // Explicitly set the type mapping
//     pub tags: Option<Vec<&'life str>>,
// }


#[derive(Debug, Clone, FromSqlRow, AsExpression, PartialEq, Eq)]
#[diesel(sql_type=crate::schema::rchat::sql_types::Content)]
pub struct Content {
    pub user: String,
    pub system: String,
}

impl Content {
    pub fn new(user: String, system: String) -> Self {
        Self {user, system}
    }
}

impl ToSql<crate::schema::rchat::sql_types::Content, Pg> for Content {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        serialize::WriteTuple::<(Text, Text)>::write_tuple(
            &(
                &self.user,
                &self.system,
            ),
            &mut out.reborrow(),
        )
    }
}
impl FromSql<crate::schema::rchat::sql_types::Content, Pg> for Content {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let (user, system) =
            FromSql::<Record<(Text, Text)>, Pg>::from_sql(bytes)?;

        Ok(Content {
            user,
            system,
        })
    }
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::rchat::history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HistoryEntry {
    pub session_id: i32,
    pub message_id: i32,
    pub level: i32,
    pub content: Content,  // Use the manually implemented Content struct
    pub tags: Vec<Option<String>>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryContent<'life> {
    pub user: &'life str,
    pub system: &'life str,
}
 