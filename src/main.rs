use reqwest::multipart;
use std::env;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename("/etc/motion/.env").ok();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <video_file_path>", args[0]);
        std::process::exit(1);
    }

    let video_path = &args[1];

    if !Path::new(video_path).exists() {
        return Err(format!("Video file not found: {}", video_path).into());
    }

    let telegram_token = env::var("TELEGRAM_TOKEN")
        .map_err(|_| "TELEGRAM_TOKEN environment variable must be set")?;
    let chat_id = env::var("CHAT_ID").map_err(|_| "CHAT_ID environment variable must be set")?;

    // send_telegram_notification(&telegram_token, &chat_id, video_path).await?;
    send_telegram_video(&telegram_token, &chat_id, video_path).await?;

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

async fn send_telegram_video(
    token: &str,
    chat_id: &str,
    video_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let video_name = Path::new(video_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("video");

    let caption = format!("🎥 Motion detected! Video captured: {}", video_name);

    let url = format!("https://api.telegram.org/bot{}/sendVideo", token);

    // Build multipart form
    let form = multipart::Form::new()
        .text("chat_id", chat_id.to_string())
        .text("caption", caption)
        .file("video", video_path)
        .await?; // attach the video file

    let response = client.post(&url).multipart(form).send().await?;

    if response.status().is_success() {
        println!("✅ Telegram video sent successfully");
        Ok(())
    } else {
        eprintln!("❌ Failed to send Telegram video: {}", response.status());
        let error_text = response.text().await?;
        eprintln!("Error response: {}", error_text);
        // Return error
        Err("Failed to send video".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_send_telegram_video_real() {
        dotenv::dotenv().ok();
        let token = env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN not set");
        let chat_id = env::var("CHAT_ID").expect("CHAT_ID not set");
        let video_path = "test_video.mov"; // Make sure this file exists

        let result = send_telegram_video(&token, &chat_id, video_path).await;
        assert!(result.is_ok(), "Failed to send video: {:?}", result.err());
    }
}
