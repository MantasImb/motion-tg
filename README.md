# Rust Motion Camera Handler

## Overview

This project is for my personal home camera that records motion events captured
by a camera and sends notifications to a Telegram chat.
I chose to use `motion` for motion detection and `Rust` for the handler script.

## Setup

### 1. Environment Variables

Create a `.env` file in the project directory:

```
TELEGRAM_TOKEN=your_bot_token
CHAT_ID=your_chat_id
```

### 2. Build the Application

```bash
cargo build --release
```

### 3. Configure Motion

Edit `/etc/motion/motion.conf` and add:

```
on_movie_end /path/to/rpimotioncamera/target/release/rpimotioncamera %f
```

Replace `/path/to/rpimotioncamera` with the actual path to your project directory.

## Usage

The application automatically runs when motion is detected and a video file is saved. It will:

1. Receive the video file path from the motion daemon
2. Send a Telegram notification with the video filename
3. Log success or error messages

## Manual Testing

You can test the application manually:

```bash
cargo run -- /path/to/test/video.mp4
```

## Telegram Bot Setup

1. Create a new bot by messaging @BotFather on Telegram
2. Get your bot token from BotFather
3. Get your chat ID by messaging @userinfobot or checking https://api.telegram.org/bot{TOKEN}/getUpdates after sending a message to your bot
