//########################################## EMITTING FOLLOWING MODULES ##########################################//

mod chat_ctx;
pub use chat_ctx::ChatHistory;
mod document_ctx;
mod knowledge_ctx;

//############################################### EXTERNAL IMPORTS ###############################################//

use std::collections::HashMap;
use reachapi::{
    ApiConfig, ApiKeys, RawOuts, ReachConfig, ReachConfigKeys, gemini_query,
};

//############################################### INTERNAL IMPORTS ###############################################//

use super::{
    Content, HistoryEntry, Message, RchatError, create_history, establish_connection, 
    get_history_by_level, get_num_history_entries, delete_histories_by_level,
};

//################################################ MEMBER IMPORTS ################################################//

//############################ COMMON FUNCTIONS/TRAITS/ENUMS (MAY/MAY-NOT BE EMITTED) ############################//


pub struct ChatContext {
    #[allow(dead_code)]
    session_id: i32,
    history: ChatHistory,
    // retrieved: DocCtx,
    // other: KnowledgeCtx,
    query: String,    // Current User Query
    response: String, // LLM Response of Current Query
}

impl ChatContext {
    pub fn new(session_id: i32, history_chunk_size: &[usize]) -> Self {
        Self {
            session_id,
            history: ChatHistory::new(session_id, history_chunk_size),
            // retrieved: DocCtx::new(session_id, ...),
            // other: KnowledgeCtx::new(session_id, ...),
            query: String::new(),
            response: String::new(),
        }
    }
    pub fn build(&mut self) -> Result<String, RchatError> {
        // Get all the data into the current context
        self.pull()?;

        // Process the data
        let ctx_str = format!(
            r#"
                Previous Messages:
                {}
            "#,
            self.history.as_ctx(),
            // self.retrieved.as_ctx(),
            // self.other.as_ctx()
        );

        // Return the context
        return Ok(ctx_str);
    }
    pub fn populate(&mut self, msg: Message) {
        match msg {
            Message::UserMsg(query) => self.query = query,
            Message::LlmMsg(response) => self.response = response,
            _ => (),
        }
    }
    pub async fn push(&mut self) -> Result<(), RchatError> {
        // For Now only History Messages as Context
        self.history.push(&self.query, &self.response).await?;

        //TODO: push for knowledge and documents later!!

        Ok(())
    }
    fn pull(&mut self) -> Result<(), RchatError> {
        // For Now only History Messages as Context
        self.history.pull()?;

        //TODO: pull for knowledge and documents later!!

        Ok(())
    }
}