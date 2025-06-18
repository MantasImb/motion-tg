import os
import sys
from telegram import Bot
import asyncio


TELEGRAM_TOKEN = os.environ["TELEGRAM_TOKEN"]
CHAT_ID = os.environ["CHAT_ID"]


async def send_video(video_path):
    if not TELEGRAM_TOKEN or not CHAT_ID:
        print("Error: TELEGRAM_TOKEN and CHAT_ID environment variables must be set.")
        return
    bot = Bot(token=TELEGRAM_TOKEN)
    try:
        with open(video_path, "rb") as video:
            await bot.send_video(chat_id=CHAT_ID, video=video)
    except Exception as e:
        print(f"An error occurred while sending the video: {e}")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: send_to_telegram.py <video_path>")
        sys.exit(1)
    asyncio.run(send_video(sys.argv[1]))
