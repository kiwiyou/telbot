# telbot

[![Active Development](https://img.shields.io/badge/Maintenance%20Level-Actively%20Developed-brightgreen.svg)](https://github.com/kiwiyou/telbot)

Telbot provides telegram bot types and API wrappers.

Specifically, telbot now supports:

- telbot-types: basic telegram types / requests / responses

```toml
[dependencies]
telbot-types = "0.2.0"
```

- telbot-cf-worker: API wrapper for cloudflare workers

```toml
[dependencies]
telbot-cf-worker = "0.2.0"
```

- telbot-ureq: API wrapper for [ureq](https://github.com/algesten/ureq) client

```toml
[dependencies]
telbot-ureq = "0.2.0"
```

- telbot-reqwest: API wrapper for [reqwest](https://github.com/seanmonstar/reqwest) client

```toml
[dependencies]
telbot-reqwest = "0.1.0"
```

## Examples

- [cloudflare workers](https://github.com/kiwiyou/telbot/tree/main/telbot-cf-worker/examples)
  - [get-me request example](https://github.com/kiwiyou/telbot/tree/main/telbot-cf-worker/examples/get-me)
  - [echo bot example](https://github.com/kiwiyou/telbot/tree/main/telbot-cf-worker/examples/echo)
  - [file(photo) send example](https://github.com/kiwiyou/telbot/tree/main/telbot-cf-worker/examples/file)
- [ureq](https://github.com/kiwiyou/telbot/tree/main/telbot-ureq/examples)
  - [echo example](https://github.com/kiwiyou/telbot/blob/main/telbot-ureq/examples/echo.rs)
  - [file(photo) send example](https://github.com/kiwiyou/telbot/blob/main/telbot-ureq/examples/file.rs)
- [reqwest](https://github.com/kiwiyou/telbot/tree/main/telbot-reqwest/examples)
  - [echo example](https://github.com/kiwiyou/telbot/blob/main/telbot-reqwest/examples/echo.rs)

## Extending telbot

You can implement two methods, `send_json` and `send_file` with your own http client to create your own API wrapper.

`send_json` should serialize `JsonMethod` into json format and send it to the API endpoint.

`send_file` should serialize `FileMethod` into `multipart/form-data` format and send it to the API endpoint.

`files(&self)` method in `FileMethod` helps getting fields of type `InputFileVariant`.

For the ease of serializing, both `JsonMethod` and `FileMethod` implements `serde::Serialize`.

`TelegramMethod`, the super trait of both `JsonMethod` and `FileMethod`, provides `name()` method,
which can be used to get the method's name in `&str` format.

## Contributing

telbot is not a mature project yet, so your help will be very helpful.

Please leave an issue if you find wrongly coded piece or get a nice idea to improve telbot.
