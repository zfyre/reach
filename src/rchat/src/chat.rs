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


struct ChatHistory<'chat_history> {
    session: &'chat_history Session,
    chunk_size: Vec<u8>, // describes the chunk size at each level of summarization
}

struct ChatContext<'context> {
    session: &'context Session, 
    history: Vec<ChatHistory<'context>>,
    // retrieved: Vec<Message>,
    // other: Vec<Message>,
}

enum Message {
    system_msg(String),
    user_msg(String),
    ai_msg(String),
}
struct Chat<'chat> {
    context: Vec<ChatContext<'chat>>,
    sys_prompt: Message,
    query: Message,
    response: Message,
    session_id: String,
}

impl<'chat> Chat<'chat> {
    fn system_prompt(&mut self) -> &Chat<'chat> {
        // API for changing the system prompt
        todo!()
    }
}
