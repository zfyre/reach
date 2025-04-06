use std::vec;

use rchat::{Chat, RchatError};

#[tokio::main]
async fn main() -> Result<(), RchatError>{
    // Start up a new chat session
    let session_id = 0;
    let history_chunk_size = vec![2, 2, 3];
    let mut chat = Chat::new(session_id, &history_chunk_size);

    // Start the chat loop
    chat.start().await?;

    Ok(())
}