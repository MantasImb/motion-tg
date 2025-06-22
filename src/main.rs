use std::env;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <video_file_path>", args[0]);
        std::process::exit(1);
    }

    let video_path = &args[1];
    let telegram_token =
        env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN environment variable must be set");
    let chat_id = env::var("CHAT_ID").expect("CHAT_ID environment variable must be set");

    send_telegram_notification(&telegram_token, &chat_id, video_path).await?;

    Ok(())
}

async fn send_telegram_notification(
    token: &str,
    chat_id: &str,
    video_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let video_name = Path::new(video_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("video");

    let message = format!("🎥 Motion detected! Video captured: {}", video_name);

    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);

    let response = client
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": message
        }))
        .send()
        .await?;

    if response.status().is_success() {
        println!("Telegram notification sent successfully");
    } else {
        eprintln!(
            "Failed to send Telegram notification: {}",
            response.status()
        );
        let error_text = response.text().await?;
        eprintln!("Error response: {}", error_text);
    }

    Ok(())
}
