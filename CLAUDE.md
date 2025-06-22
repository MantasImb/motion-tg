# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a personal home security camera project that integrates with the `motion` daemon to detect motion events and send notifications to Telegram. The project is implemented in Rust.

## Architecture

- **Motion Detection**: Uses the system `motion` daemon for capturing motion events
- **Event Handler**: Rust application that processes motion events and sends notifications
- **Telegram Integration**: Sends notifications and media to a Telegram chat
- **Configuration**: Motion daemon configured via `/etc/motion/motion.conf`

## Environment Setup

Required environment variables:
- `TELEGRAM_TOKEN`: Bot token for Telegram API
- `CHAT_ID`: Target Telegram chat ID

Environment variables should be set in `.env` file (not tracked in git).

## Development Commands

- `cargo build` - Build the project
- `cargo run` - Run the application
- `cargo test` - Run tests
- `cargo check` - Check code without building

## Integration with Motion

The application is designed to be triggered by the motion daemon. Motion events are configured to execute the Rust handler script when motion is detected.

## File Structure

- Motion captures video files (`.h264`, `.mp4`, `.avi`) - excluded from git
- Log files (`.log`) - excluded from git