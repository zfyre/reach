use super::{
    Content, HistoryEntry, Message, RchatError, create_history, establish_connection, 
    get_history_by_level, get_num_history_entries, delete_histories_by_level,
    HashMap, ApiConfig, ApiKeys, gemini_query, RawOuts, ReachConfig, ReachConfigKeys
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
    pub fn new(session_id: i32, history_chunk_size: &[usize]) -> Self {
        Self {
            session_id,
            history: ChatHistory::new(session_id, history_chunk_size),
            // retrieved: vec![],
            // other: vec![],
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
            self.history.as_ctx()
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

// ======================== ChatHistory ========================
struct ChatHistory {
    session_id: i32,
    chunk_size: Vec<usize>, // describes the chunk size at each level of summarization
    history: Vec<HistoryEntry>,
    level_entry_counts: Vec<usize>, // describes the number of entries at each level of summarization
}

impl ChatHistory {
    fn new(session_id: i32, chunk_size: &[usize]) -> Self {
        Self {
            session_id,
            chunk_size: chunk_size.to_vec(),
            history: vec![],
            level_entry_counts: {
                let connection = &mut establish_connection();
                let mut counts = vec![];
                for lvl in 0..chunk_size.len() {
                    counts.push(
                        get_num_history_entries(
                            connection,
                            session_id,
                            lvl as i32
                        ) as usize
                    );
                }

                counts
            },
        }
    }
    fn pull(&mut self) -> Result<(), RchatError> {
        // Pull the history from the database
        let connection = &mut establish_connection();

        // Pull the History for each level based on it's chunk size
        for (lvl, chunk_size) in self.chunk_size.iter().enumerate() {
            let entries = get_history_by_level(
                connection,
                self.session_id,
                lvl as i32,
                *chunk_size
            );
            self.history.extend(entries);
        }

        Ok(())
    }

    async fn push(&mut self, query: &str, response: &str) -> Result<(), RchatError> {
        // Push the history to the database
        let connection = &mut establish_connection();
        let content = Content::new(query.to_owned(), response.to_owned(), vec![]);

        let _entry = create_history(
            connection,
            self.session_id,
            0,
            &content
        );
        self.level_entry_counts[0] += 1;

        // Iterate over the levels except the last one & summarize the entries
        for (lvl, &chunk_size) in self.chunk_size[..self.chunk_size.len() - 1]
            .iter()
            .enumerate()
        {
            if self.level_entry_counts[lvl] >= 2 * chunk_size {
                let summarized_content = self.summarize_entries(lvl as i32).await?;
                let _summarized_entry = create_history(
                    connection,
                    self.session_id,
                    (lvl as i32) + 1,
                    &summarized_content,
                );
            }
        }

        // eprintln!("Saved history entry {:?}", _entry);// TODO: Use Tracing library for logging

        // Empty the history for the next iteration of `pull`
        self.history.clear();

        Ok(())
    }

    async fn summarize_entries(&self, lvl: i32) -> Result<Content, RchatError> {
        // Get the #chunk_size of entries for a given level
        let connection = &mut establish_connection();
        let entries = get_history_by_level(
            connection,
            self.session_id,
            lvl,
            2 * self.chunk_size[lvl as usize],
        );
        let relevant_entries = &entries[entries.len()/2 ..];

        // Summarize the entries
        let content = Self::generate_new_content(relevant_entries).await?;

        // Delete the entries
        delete_histories_by_level(connection, self.session_id, relevant_entries);

        Ok(content)
    }
    async fn generate_new_content(entries: &[HistoryEntry]) -> Result<Content, RchatError> {
        let api_config: HashMap<String, String> =
            ApiConfig::read_config().unwrap().into_iter().collect();
        let gemini_api_key = api_config
            .get(&ApiKeys::Gemini.as_str())
            .expect("Gemini API key is not available");

        let mut entry_str = String::new();

        for entry in entries {
            entry_str.push_str(&format!(
            r#"
            User: {}
            System: {}
            ---
            "#,
            entry.content.user, entry.content.system
            ));
        }

        let query = format!(
            r#"Summarize the following conversation into a "single" exchange between User and System. 
            Keep the essential information and context. Format the output as following:

            User: <summarized user messages in less than 50 words>
            [split]
            System: <summarized system responses in less than 50 words>

            Conversation to summarize:
            {}
            "#,
            entry_str
        );

        let mut response = gemini_query(&gemini_api_key, &query).await?;
        
        let response_str = match response.pop().unwrap() {
            RawOuts::RawGeminiOut(s) => s,
            _ => unreachable!(),
        }.trim().to_string();

        // Split response into parts and extract user and system messages
        let parts: Vec<&str> = response_str
            .trim()
            .trim_start_matches("\"")
            .trim_end_matches("\"")
            .split("[split]")
            .collect();

        let mut user_msg = String::new();
        let mut system_msg = String::new();

        for line in parts {
            if line.contains("User:") {
                user_msg = line
                    .trim()
                    .trim_start_matches("\"")
                    .trim_end_matches("\"")
                    .trim_start_matches("User:")
                    .to_string();

            } else if line.contains("System:") {
                system_msg = line
                    .trim()
                    .trim_start_matches("\"")
                    .trim_end_matches("\"")
                    .trim_start_matches("System:")
                    .to_string();
            }
        }
                
        Ok(Content::new(user_msg, system_msg, vec![]))
        
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
