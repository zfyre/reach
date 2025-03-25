use super::{
    Content, HistoryEntry, Message, RchatError, create_history, establish_connection,
    get_history_by_level,
};

pub struct ChatContext {
    session_id: i32,
    history: ChatHistory,
    // retrieved: Vec<Message>,
    // other: Vec<Message>,
    query: String,    // Current User Query
    response: String, // LLM Response of Current Query
}

impl ChatContext {
    pub fn new(session_id: i32, history_chunk_size: Vec<usize>) -> Self {
        Self {
            session_id,
            history: ChatHistory::new(session_id, history_chunk_size),
            // retrieved: vec![],
            // other: vec![],
            query: String::new(),
            response: String::new(),
        }
    }

    /// Build the context
    ///
    /// # Returns
    ///
    /// * `Result<String, RchatError>` - The result of the build operation
    pub fn build(&mut self) -> Result<String, RchatError> {
        // Get all the data into the current context
        self.pull()?;

        // Process the data
        let ctx_str = format!(
            r#"
                Previous Messages:
                {}
            "#,
            self.history.as_ctx()
        );

        // Return the context
        return Ok(ctx_str);
    }

    /// Populate the context with the given message currently only supports user and llm messages
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to populate the context with
    pub fn populate(&mut self, msg: Message) {
        match msg {
            Message::UserMsg(query) => self.query = query,
            Message::LlmMsg(response) => self.response = response,
            _ => (),
        }
    }

    /// Push the current context into the database
    ///
    /// # Returns
    ///
    /// * `Result<(), RchatError>` - The result of the push operation
    pub fn push(&mut self) -> Result<(), RchatError> {
        // For Now only History Messages as Context
        self.history.push(&self.query, &self.response)?;

        //TODO: push for knowledge and documents later!!

        Ok(())
    }
    /// Pull the current context from the database
    ///
    /// # Returns
    ///
    /// * `Result<(), RchatError>` - The result of the pull operation
    fn pull(&mut self) -> Result<(), RchatError> {
        // For Now only History Messages as Context
        self.history.pull()?;

        //TODO: pull for knowledge and documents later!!

        Ok(())
    }
}

// ======================== ChatHistory ========================
struct ChatHistory {
    session_id: i32,
    chunk_size: Vec<usize>, // describes the chunk size at each level of summarization
    history: Vec<HistoryEntry>,
}

impl ChatHistory {
    fn new(session_id: i32, chunk_size: Vec<usize>) -> Self {
        Self {
            session_id,
            chunk_size,
            history: vec![],
        }
    }
    fn pull(&mut self) -> Result<(), RchatError> {
        // Pull the history from the database
        let connection = &mut establish_connection();

        // Pull the History for each level based on it's chunk size
        for (lvl, chunk_size) in self.chunk_size.iter().enumerate() {
            let entry = get_history_by_level(connection, self.session_id, lvl as i32, *chunk_size);
            self.history.extend(entry);
        }

        Ok(())
    }

    fn push(&mut self, query: &str, response: &str) -> Result<(), RchatError> {
        // Push the history to the database
        let connection = &mut establish_connection();
        let content = Content::new(query.to_owned(), response.to_owned(), vec![]);

        let _entry: HistoryEntry = create_history(connection, self.session_id, 0, &content);

        // eprintln!("Saved history entry {:?}", _entry);// TODO: Use Tracing library for logging

        // Empty the history
        self.history.clear();

        Ok(())
    }

    /// Formats the Vec<HistoryEntry> into a String
    fn as_ctx(&self) -> String {
        let mut ctx_str = String::new();
        for entry in self.history.iter().rev() {
            ctx_str.push_str(&format!(
                r#"
                User: {}
                System: {}
                ---
                "#,
                entry.content.user, entry.content.system
            ));
        }
        ctx_str
    }
}

// ======================== ChatKnowledge =======================

// ======================== ChatDocuments ========================
