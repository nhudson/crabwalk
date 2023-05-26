# crabwalk

A work in progress github webhook notifier written in rust

## About

The idea would be to write a Github webhook router that could route webhooks
wherever I want them to go.  The initial concept is to have them routed to a
Slack application that will notify me of everything in Github.

## Installation

TBD

## Build

This is still under development, so nothing really works yet.

I am using [just](https://github.com/casey/just) to kick off local builds

You will also need to use the latest version of rust (1.69).  So make sure you're
up to date `rustup update`.

The `justfile` also uses [cargo-watch](https://crates.io/crates/cargo-watch), so
if you wish to use it install it as well.

The application takes a few environment variables that must be defined

| Vairbale       | Defaults    |
| -------------- | ----------- |
| WEBHOOK_SECRET | `zzhUNwm8OlyDFQGKztGPMPVQ2ayFv8r3EzfJOjpp2yA` |
| GITHUB_TOKEN   | n/a         |
| SLACK_TOKEN    | n/a         |
| SERVER_HOST    | `0.0.0.0`   |
| SERVER_PORT    | `8080`      |

```bash
> SLACK_TOKEN=<slack token> GITHUB_TOKEN=<github token> just watch
```

## Testing

To test you will need a sample payload.  Save this as a `payload.json` file.

```bash
{
  "action": "created",
  "issue": {
    "number": 1,
    "title": "An issue",
    "body": "Please fix this."
  },
  "comment": {
    "body": "A comment on the issue."
  }
}
```

Next you will need to generate an HMAC signature

```bash
HMAC_SIGNATURE=$(openssl dgst -sha1 -hmac YOUR_SECRET_KEY payload.json | awk '{print "sha1="$2}')
```

Now you can send the payload with the `cURL` request

```bash
curl -X POST \
     -H 'Content-Type: application/json' \
     -H "X-Hub-Signature: $HMAC_SIGNATURE" \
     -d @payload.json \
     http://localhost:8080/github
```
