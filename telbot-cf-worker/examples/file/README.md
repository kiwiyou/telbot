# Example: file

## How to run

You must create a temporary bot account using [@BotFather](https://t.me/botfather).

Then, build this example:

```bash
wrangler build
```

Before running the example, you should store your bot token to Cloudflare secrets.

```bash
wrangler secret put BOT_TOKEN
```

You can now run the example.

```bash
wrangler dev
```

You may want to use tunnelling tool such as ngrok to expose your dev webhook server to Telegram.

Visit `https://api.telegram.org/bot<your bot token>/setWebhook?url=<your webhook url>`
to set up the webhook.
