use std::vec;
use rchat::{Chat, RchatError};
use flexi_logger::{Logger, WriteMode, FileSpec};

#[tokio::main]
async fn main() -> Result<(), RchatError>{

    // Initialize the logger
    Logger::try_with_str("debug") // or "rchat=info, rchat=debug"
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("logs")
                .suffix("log")
                .use_timestamp(true)
        )
        .format(flexi_logger::detailed_format) // Includes timestamp in each log entry
        .write_mode(WriteMode::Direct)
        .start()
        .unwrap();
    
    // Start up a new chat session
    let session_id = 0;
    let history_chunk_size = vec![2, 2, 3];
    let mut chat = Chat::new(session_id, &history_chunk_size);

    // Start the chat loop
    chat.start().await?;
    
    Ok(())
}