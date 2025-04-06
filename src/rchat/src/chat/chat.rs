use super::{
    ApiConfig, ApiKeys, ChatContext, HashMap, Message, RawOuts, RchatError, ReachApiError,
    ReachConfig, ReachConfigKeys, Write, gemini_query, io,
};

enum ChatCode {
    Quit,
    Command,
    UserInput,
}

pub struct Chat {
    session_id: i32,
    context: ChatContext,
    sys_prompt: Message,
}

impl<'life> Chat {
    pub fn new(session_id: i32, history_chunk_size: &[usize]) -> Self {
        Self {
            session_id,
            context: ChatContext::new(session_id, history_chunk_size),
            sys_prompt: Message::default_sys_msg(),
        }
    }

    pub fn update_sys_prompt(&mut self, new_sys_prompt: &str) -> &Chat {
        self.sys_prompt = Message::SysMsg(new_sys_prompt.to_string());
        self
    }

    pub async fn start(&mut self) -> Result<(), RchatError> {
        // Start the chat loop
        println!(
            "\x1b[36mStarting chat session {} ...\x1b[0m",
            self.session_id
        );
        
        loop {
            // Get user input
            let (chat_code, query) = self.get_user_input()?;

            // Populate current Chat Context -> Specifically the 'query'
            self.context.populate(query.clone());

            // Process user input -> Adds the Context
            let processed_query = self.process_prompt(query)?;

            match chat_code {
                // Quit the Chat loop if user enters quit command
                ChatCode::Quit => {
                    break;
                }
                // Handle commands like help, show, etc.
                ChatCode::Command => {
                    todo!("Implement command handling");
                    // continue;
                }
                // Handle user input
                ChatCode::UserInput => {
                    // Generate AI response
                    let llm_response = self.get_llm_response(processed_query).await?;

                    // Populate current Chat Context -> Specifically the 'response'
                    self.context.populate(llm_response.clone());

                    // Display AI response
                    self.display_llm_response(llm_response)?;
                }
            }

            // Save the current chat Context to the database
            self.context.push().await?;
        }

        Ok(())
    }

    fn get_user_input(&self) -> Result<(ChatCode, Message), RchatError> {
        print!("\x1b[32m>\x1b[0m ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        let mut chat_code = ChatCode::UserInput;

        if input.starts_with('\\') {
            match input {
                "\\quit" | "\\exit" => {
                    println!("\x1b[36mEnding chat session {} ...\x1b[0m", self.session_id);
                    chat_code = ChatCode::Quit;
                }
                "\\help" => {
                    println!("Available commands:");
                    println!("  \\quit, \\exit - End the chat session");
                    println!("  \\help        - Display this help message");
                    println!("  \\show        - Display current chat session info");
                    chat_code = ChatCode::Command;
                }
                "\\show" => {
                    println!("Chat Session Info:");
                    println!("  Session ID: {}", self.session_id);
                    println!("  System prompt: {}", self.sys_prompt);
                    println!("  System prompt loaded: Yes");
                    chat_code = ChatCode::Command;
                }
                _ => {
                    println!("Unknown command. Type \\help for available commands.");
                    chat_code = ChatCode::Command;
                }
            }
        }

        Ok((chat_code, Message::UserMsg(input.to_string())))
    }

    fn process_prompt(&mut self, user_input: Message) -> Result<String, RchatError> {
        let q = format!(
            r#"
                SYSTEM: 
                {}

                Addition Context:

                {}

                USER: 
                {}

                Answer: 
            "#,
            self.sys_prompt.into_inner(),
            self.context.build()?,
            user_input.into_inner()
        );

        // TODO: Check the `Order` of History Context which is printing from context.history.as_ctx() function
        Ok(q)
    }

    async fn get_llm_response(&self, query: String) -> Result<Message, ReachApiError> {
        let api_config: HashMap<String, String> =
            ApiConfig::read_config().unwrap().into_iter().collect();
        let gemini_api_key = api_config
            .get(&ApiKeys::Gemini.as_str())
            .expect("Gemini API key is not available");

        let mut response = gemini_query(&gemini_api_key, &query).await?;

        let response_str = match response.pop().unwrap() {
            RawOuts::RawGeminiOut(s) => s,
            _ => unreachable!(),
        }
        .trim()
        .to_string();

        Ok(Message::LlmMsg(response_str))
    }

    fn display_llm_response(&self, llm_response: Message) -> Result<(), RchatError> {
        io::stdout().flush()?;
        println!(">{}", llm_response.into_inner());

        Ok(())
    }
}
