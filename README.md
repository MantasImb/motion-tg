# Python motion handler script for tg

Create a folder in `home/{device}` named `motion`

Install dependencies:

```
sudo apt install python3 python3-pip
pip3 install python-telegram-bot
```

Init the environment variables:

```
export TELEGRAM_TOKEN="your_token"
export CHAT_ID="your_chat_id"
```

Edit `/etc/motion/motion.conf`:

```
on_movie_end python3 /home/pi/motion/send_to_telegram.py %f
```
