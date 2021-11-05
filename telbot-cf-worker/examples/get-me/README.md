# Example: get-me

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
curl 127.0.0.1:8787
```
