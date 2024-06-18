use glide_rs::lang::chat::ChatRequest;
use glide_rs::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::default();
    client.health().await?;

    let list = client.lang.list().await?;
    let router = list.routers.first().unwrap();
    let name = router.routers.as_str();

    let request = ChatRequest::new("Hello!");
    let response = client.lang.chat(&name, request).await?;
    let content = response.model_response.message.content;
    println!("response: {content}");

    Ok(())
}
