<div align="center">
  <a href="https://github.com/HideyoshiNakazone/hideyoshi.com">
    <img src="https://drive.google.com/uc?export=view&id=1ka1kTMcloX_wjAlKLET9VoaRTyRuGmxQ" width="100" height="100" allow="autoplay"\>
  </a>
</div>

# message-hideyoshi.com

Made using Rust and Axum, this project was made as contact microservice for the [hideyoshi.com project](https://github.com/HideyoshiNakazone/hideyoshi.com), as so it is  responsible for everything related to messaging.

All code in this repo is distributed freely by the GPLv3 License.
## Environment Variables

For the execution of this project the following environment variables must be set:

`HOST`

`PORT`

`ALLOWED_ORIGINS`

- Auth Server:

    `AUTH_URL`

- Redis:

    `REDIS_HOST`

    `REDIS_PORT`

    `REDIS_PASSWORD`

- SMTP:
    
    `SMTP_SERVER`

    `SMTP_PORT`

    `SMTP_NAME`

    `SMTP_EMAIL`

    `SMTP_USERNAME`

    `SMTP_PASSWORD`



## Usage

Building project:

```bash
cargo build --release
```

Executing project:

```bash
./target/release/message-hideyoshi-com [option]
```

**options**: 
  - both - _default_
  - server
  - worker


## API Reference

#### Send Message

```http
  POST /message
```

**Requires Authorization Bearer Token**

| Parameter      | Type     | Description                          |
| :--------      | :------- | :-------------------------           |
| `subject`      | `string` | **Required**. Subject of the message |
| `message`      | `string` | **Required**. Content of the message |


## Requirements

This projects requires [Backend Hideyoshi.com](https://github.com/HideyoshiSolutions/backend-hideyoshi.com) as a Auth Server.


## Authors

- [@HideyoshiNakazone](https://github.com/HideyoshiNakazone)

