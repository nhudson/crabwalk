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
> just watch
```
