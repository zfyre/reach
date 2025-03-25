use rchat::{chat::Chat, error::RchatError};

#[tokio::main]
async fn main() -> Result<(), RchatError>{
    // Start up a new chat session
    let mut chat = Chat::new(0);

    // Start the chat loop
    chat.start().await?;

    Ok(())
}