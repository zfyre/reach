// @generated automatically by Diesel CLI.

pub mod rchat {
    pub mod sql_types {
        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "content", schema = "rchat"))]
        pub struct Content;
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::Content;

        rchat.history (session_id, message_id, level) {
            session_id -> Int4,
            message_id -> Int4,
            level -> Int4,
            content -> Content,
        }
    }
}
