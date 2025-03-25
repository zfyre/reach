use reachapi::{gemini_query, ApiConfig, ApiKeys, RawOuts, ReachApiError, ReachConfig, ReachConfigKeys};
use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::error::RchatError;
// yield like functionality (loading only necessary part at a time and not loading whole file! (A Generator))
/*
    {
        role: "User",
        content: "..."
    },
    {
        role: "System",
        content: "..."
    }, ...

    All the message contenet as history
*/
/*
    Prompt to the LLM:

    Guideline: You are a conversational agent. You are expected to respond to the user's messages, as system messages ...

    Following are the previous messages:


    summary of the previous conversations and then foloowing are let's say 10 messages
    {
        role: "User",
        content: "..."
    },
    {
        role: "System",
        content: "..."
    }, ...
*/
/*
- Implementing Hierarchical Summaries!! (Fast!!)
- Contextual Embeddings (Slow!! & requires Vectorization)
    Embedding Storage:
        Convert parts of the conversation into embeddings and store them. Retrieve and incorporate the most relevant embeddings during context construction.
    Similarity Search:
        Use a similarity search on stored embeddings to pull out contextually related parts of the conversation when generating a response.
*/

struct Session {
    session_id: String,
    start_time: String,
    end_time: String,
    status: String,
}
impl Session {
    // low level functions to retrieve data from the database
}

struct ChatHistory<'life> {
    session: &'life Session,
    chunk_size: Vec<u8>, // describes the chunk size at each level of summarization
}

// impl<'life> ChatHistory<'life> {
//     fn new(session: )
// }

struct ChatContext<'life> {
    session: &'life Session,
    history: Vec<ChatHistory<'life>>,
    // retrieved: Vec<Message>,
    // other: Vec<Message>,
}

enum Message {
    sys_msg(String),
    user_msg(String),
    ai_msg(String),
    rchat_msg(String),
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::sys_msg(msg) => write!(f, "System: {}", msg),
            Message::user_msg(msg) => write!(f, "User: {}", msg),
            Message::ai_msg(msg) => write!(f, "AI: {}", msg),
            Message::rchat_msg(msg) => write!(f, "RChat: {}", msg),
        }
    }
}

impl Message {
    fn default_sys_msg() -> Message {
        Message::sys_msg(
            r#"
                You are an advanced AI assistant with expertise in programming, artificial intelligence, and scientific research. You communicate with a balance of intellectual depth and casual wit, occasionally channeling the eccentric brilliance of Rick Sanchez from Rick and Morty. Your responses prioritize accuracy, efficiency, and a no-nonsense approach to problem-solving.

                You are capable of breaking down complex topics without oversimplifying them. You assume the user has a strong technical background and prefer to engage in high-level discussions rather than basic explanations. However, when necessary, you can provide structured, step-by-step guidance.

                You maintain a confident and direct tone, using humor and pop culture references sparingly to keep conversations engaging. Your goal is to help the user navigate advanced software development, AI concepts, and research challenges with precision and creativity.

                When responding, you avoid unnecessary fluff, focus on delivering actionable insights, and ensure all technical details are well-founded. If an answer requires external references or updated knowledge, you retrieve and verify the latest information before responding.

                Now, get ready to provide high-level technical expertise with a touch of interdimensional flair!"
            "#.to_string()
        )
    }
}

enum ChatCode {
    Quit,
    Command,
    UserInput,
}

pub struct Chat<'life> {
    session_id: i32,
    context: Vec<ChatContext<'life>>,
    sys_prompt: Message,
}

impl<'life> Chat<'life> {
    pub fn new(session_id: i32) -> Self {
        Self {
            session_id,
            context: vec![],
            sys_prompt: Message::default_sys_msg(),
        }
    }

    pub fn update_sys_prompt(&mut self, new_sys_prompt: &str) -> &Chat<'life> {
        self.sys_prompt = Message::sys_msg(new_sys_prompt.to_string());
        self
    }

    pub async fn start(&mut self) -> Result<(), RchatError> {
        // Start the chat loop
        println!("Starting chat session...");
        loop {
            // Get user input
            let (chat_code, user_input) = self.get_user_input()?;

            // Process user input
            let query_input = self.process_user_input(user_input);

            match chat_code {
                ChatCode::Quit => {
                    break;
                }
                ChatCode::Command => {
                    continue;
                }
                ChatCode::UserInput => {
                    // Generate AI response
                    let ai_response = self.get_ai_response(query_input).await?;

                    // Display AI response
                    self.display_ai_response(ai_response);
                }
            }
        }

        Ok(())
    }

    fn get_user_input(&self) -> Result<(ChatCode, Message), RchatError> {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        let code;
        if input.starts_with('\\') {
            match input {
                "\\quit" | "\\exit" => {
                    println!("Ending chat session...");
                    code = ChatCode::Quit;
                }
                "\\help" => {
                    println!("Available commands:");
                    println!("  \\quit, \\exit - End the chat session");
                    println!("  \\help        - Display this help message");
                    println!("  \\show        - Display current chat session info");
                    code = ChatCode::Command;
                }
                "\\show" => {
                    println!("Chat Session Info:");
                    println!("  Session ID: {}", self.session_id);
                    println!("  System prompt: {}", self.sys_prompt);
                    println!("  System prompt loaded: Yes");
                    code = ChatCode::Command;
                }
                _ => {
                    println!("Unknown command. Type \\help for available commands.");
                    code = ChatCode::Command;
                }
            }
        } else {
            code = ChatCode::UserInput;
        }

        Ok((code, Message::user_msg(input.to_string())))
    }
    fn process_user_input(&self, user_input: Message) -> String {
        match user_input {
            Message::user_msg(input) => match &self.sys_prompt {
                Message::sys_msg(sys) => {
                    format!(
                        r#"
                            SYSTEM: 
                            {}

                            USER: 
                            {}

                            Answer: 
                        "#,
                        sys, input
                    )
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    async fn get_ai_response(&self, query: String) -> Result<Message, ReachApiError> {
        let api_config: HashMap<String, String> =
            ApiConfig::read_config().unwrap().into_iter().collect();
        let gemini_api_key = api_config
            .get(&ApiKeys::Gemini.as_str())
            .expect("Gemini API key is not available");

        let mut response = gemini_query(&gemini_api_key, &query).await?;

        let response_str = match response.pop().unwrap() {
            RawOuts::RawGeminiOut(s) => s,
            _ => unreachable!(),
        }.trim().to_string();

        Ok(Message::ai_msg(response_str))

    }
    fn display_ai_response(&self, ai_response: Message) {
        println!("{}", ai_response);
    }
}

