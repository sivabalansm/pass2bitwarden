# pass2bitwarden

A CLI tool to export your [pass](https://www.passwordstore.org/) (the standard Unix password manager) entries into a Bitwarden-compatible CSV for easy import into Bitwarden or Vaultwarden.

## How It Works

1. Recursively scans your pass store directory for `.gpg` entries
2. Decrypts each entry via `pass show`
3. Detects the entry format and extracts credentials
4. Writes a `bitwarden.csv` ready for import

### Supported Entry Formats

| Format | Structure | Email Source |
|---|---|---|
| Pass Firefox extension | `password\nlogin: user@example.com\nurl: ...` | Extracted from entry |
| Firefox extension (no email) | `password\nurl: ...` | Prompted via stdin |
| Password + email | `password\nemail` | Extracted from entry |
| Password only | `password` | Prompted via stdin |

When prompted for an email, pressing Enter without input will use the configured `DEFAULT_EMAIL`.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- [pass](https://www.passwordstore.org/) installed and configured with a GPG key
- GPG agent running (entries are decrypted during export)

## Configuration

Edit the constants at the top of `src/main.rs` before building:

```rust
const PASS_DIR: &str = "";       // Path to your pass store (e.g. "/home/user/.password-store")
const DEFAULT_EMAIL: &str = "";  // Fallback email for entries missing one
const EXPORT_CSV: &str = "bitwarden.csv"; // Output filename
```

## Usage

```sh
# Build
cargo build --release

# Run
./target/release/pass2bitwarden
```

The tool will iterate through every entry, print the detected format, and prompt for an email when one can't be extracted. The result is written to `bitwarden.csv` in the current directory.

### Importing into Bitwarden

1. Log into your Bitwarden (or Vaultwarden) vault
2. Go to **Tools > Import Data**
3. Select format **Bitwarden (csv)**
4. Upload the generated `bitwarden.csv`

## CSV Output Format

```
name,login_username,login_password,login_uri
https://example.com,user@example.com,hunter2,https://example.com
```

## License

This is a personal utility. No license specified.
