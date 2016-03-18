# server

## Building

Because of dependency on Serde, I am using rust nightly.

```bash
$ multirust override nightly
$ cargo build
```

## openssl

OSX does not ship with openssl on 10.11. To build install openssl with brew
and export environment variable.
See https://github.com/sfackler/rust-openssl/issues/255 and https://github.com/servo/servo/issues/7303

```bash
$ brew install openssl
$ export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include
$ cargo build
```

## Database

I am using the postgres docker image for the database.

For database setup and communication I am using [diesel](https://github.com/sgrif/diesel).
```bash
$ cargo install diesel_cli
# setup database
$ diesel migration run
```
NOTE: I am giving up on diesel for now since it's not compiling on my version of rust nightly (rustc 1.8.0-nightly (57c357d89 2016-02-16))

I will keep using the diesel cli for database migrations though. It's a pretty decent tool.

## Testing

```bash
$ cargo test
$ cargo test --features it_test
```