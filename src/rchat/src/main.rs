mod chat {

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
}

mod chat_database {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct HistoryEntry {
        session_id: i32,
        message_id: i32,
        level: i32,
        content: ChatContent,
        tag: String,
    }

    pub fn init() -> () {
        let db = DbInstance::new(
            "sqlite",
            "data/test_session.db",
            Default::default()
        ).unwrap();
        
        let script = r#"
            :create history{
                session_id: Int,
                message_id: Int,
                level: Int,
                =>
                content: Any,
                tag: String
            }
        "#;
        let result = db.run_script(
            script,
            Default::default(),
            ScriptMutability::Mutable
        ).unwrap();
        
        println!("{:?}", result);
    
        let script = r#"
            ?[session_id, message_id, level, content, tag] <- [[1, 1, 0, {}, "abc"], [1, 2, 0, {}, "def"]]
            :put history {session_id, message_id => level, content, tag}
        "#;
        let result = db.run_script(
            script,
            Default::default(),
            ScriptMutability::Mutable
        ).unwrap(); 
        println!("{:?}", result);

        let script = r#"
            ?[a, b, c, d, e] := *history[a, b, c, d, e]
        "#;
        let result = db.run_script(
            script,
            Default::default(),
            ScriptMutability::Immutable
        ).unwrap();
        let a = result.into_json();
        println!("{:?}", a);
    }
    
}

/*
    To let top-k messages of certain level
    ?[session_id, message_id, content, tag] := *history[session_id, message_id, {level}, content, tag]
    :limit {k}
*/

fn main() {
    println!("Hello, world!");

    use chat_database::init;
    init();
}
