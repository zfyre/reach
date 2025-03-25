//########################################## EMITTING FOLLOWING MODULES ##########################################//

pub mod chat;
mod contexts;
pub use contexts::ChatContext;

//############################################### EXTERNAL IMPORTS ###############################################//

use reachapi::{
    ApiConfig, ApiKeys, RawOuts, ReachApiError, ReachConfig, ReachConfigKeys, gemini_query,
};
use std::{
    collections::HashMap,
    io::{self, Write},
};

//############################################### INTERNAL IMPORTS ###############################################//

use crate::diesel_api::{
    create_history, delete_histories_by_level, establish_connection, get_history_by_level,
    get_num_history_entries,
};
use crate::error::RchatError;
use crate::models::{Content, HistoryEntry};

//################################################ MEMBER IMPORTS ################################################//

//############################ COMMON FUNCTIONS/TRAITS/ENUMS (MAY/MAY-NOT BE EMITTED) ############################//

// struct Session {
//     session_id: String,
//     start_time: String,
//     end_time: String,
//     status: String,
// }
// impl Session {
//     // low level functions to retrieve data from the database
// }

#[derive(Clone)]
pub enum Message {
    SysMsg(String),
    UserMsg(String),
    LlmMsg(String),
    RchatMsg(String),
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::SysMsg(msg) => write!(f, "System: {}", msg),
            Message::UserMsg(msg) => write!(f, "User: {}", msg),
            Message::LlmMsg(msg) => write!(f, "LLM: {}", msg),
            Message::RchatMsg(msg) => write!(f, "RChat: {}", msg),
        }
    }
}

impl Message {
    fn default_sys_msg() -> Message {
        Message::SysMsg(
            r#"
            You are an advanced AI assistant with expertise in programming, artificial intelligence, and scientific research. Your responses prioritize accuracy, efficiency, and clarity, avoiding unnecessary complexity or oversimplification.

            You assume the user has a strong technical background and prefer to engage in high-level discussions while providing structured, step-by-step guidance when necessary.

            Your goal is to deliver precise, actionable insights to help the user navigate advanced software development, AI concepts, and research challenges. You maintain a direct and professional tone, ensuring all technical details are well-founded.

            Keep responses concise, avoiding unnecessary elaboration. If updated information is required, retrieve and verify the latest sources before responding.
            "#.to_string()
        )
    }

    fn into_inner(&self) -> String {
        match self {
            Message::SysMsg(msg) => msg.to_owned(),
            Message::UserMsg(msg) => msg.to_owned(),
            Message::LlmMsg(msg) => msg.to_owned(),
            Message::RchatMsg(msg) => msg.to_owned(),
        }
    }
}
