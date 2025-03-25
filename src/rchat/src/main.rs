use std::vec;

use rchat::{Chat, RchatError};

#[tokio::main]
async fn main() -> Result<(), RchatError>{
    // Start up a new chat session
    let mut chat = Chat::new(0, vec![3]);

    // Start the chat loop
    chat.start().await?;

    Ok(())
}