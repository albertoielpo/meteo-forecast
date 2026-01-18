# meteo-forecast

A Rust application that fetches weather forecast data from AEMET (Spanish Meteorological Agency), formats it into a human-readable report, and sends it as a JSON payload to a mail service (e.g., rustmail) for email delivery.

## Features

- Downloads weather forecast XML from AEMET
- Parses and formats weather data into a human-readable report
- Sends the report via email using a mail service

## Environment Variables

All environment variables are required:

| Variable | Description | Example |
|----------|-------------|---------|
| `AEMET_LOCATION` | AEMET XML endpoint for the location | `https://www.aemet.es/xml/municipios/localidad_43153.xml` |
| `MAIL_FROM` | Sender email address | `name@example.com` |
| `MAIL_TO` | Comma-separated list of recipient emails | `user1@example.com, user2@example.com` |
| `RUSTMAIL_URL` | Mail service endpoint | `http://localhost:3333/send` |

## Usage

```bash
AEMET_LOCATION="https://www.aemet.es/xml/municipios/localidad_43153.xml" \
MAIL_FROM="sender@example.com" \
MAIL_TO="recipient@example.com" \
RUSTMAIL_URL="http://localhost:3333/send" \
cargo run
```

## Building

### Development build

```bash
cargo build
```

### Static binary for deployment (Linux)

To create a fully static, self-contained binary that runs on any Linux distribution (Ubuntu, Debian, Alpine, etc.):

```bash
# Install the musl target (one-time setup)
rustup target add x86_64-unknown-linux-musl

# Install musl-tools (Ubuntu/Debian)
sudo apt install musl-tools

# Build the static binary
cargo build --release --target x86_64-unknown-linux-musl
```

The binary will be at `target/x86_64-unknown-linux-musl/release/meteo-forecast`.

Verify it's fully static:

```bash
file target/x86_64-unknown-linux-musl/release/meteo-forecast
ldd target/x86_64-unknown-linux-musl/release/meteo-forecast  # should say "not a dynamic executable"
```

### Docker

Build and run using Docker:

```bash
# Build the image
docker build -t meteo-forecast .

# Run
docker run --rm \
  -e AEMET_LOCATION="https://www.aemet.es/xml/municipios/localidad_43153.xml" \
  -e MAIL_FROM="sender@example.com" \
  -e MAIL_TO="recipient@example.com" \
  -e RUSTMAIL_URL="http://localhost:3333/send" \
  meteo-forecast
```

## Logging

Enable logging by setting the `RUST_LOG` environment variable:

```bash
RUST_LOG=info cargo run
```
