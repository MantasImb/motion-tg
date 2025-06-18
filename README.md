# Python motion handler script for tg

Init the environment variables:

```
export TELEGRAM_TOKEN="your_token"
export CHAT_ID="your_chat_id"
```

Edit `/etc/motion/motion.conf`:

```
on_movie_end python3 /home/pi/motion/send_to_telegram.py %f
```
