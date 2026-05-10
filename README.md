# Gitsock

A lightweight CLI tool for managing multiple GitHub accounts on the same machine.

Gitsock handles account switching, SSH identity management, and commit authorship so you never accidentally push from the wrong account.

---

## Features

- **Multi-Account Management** – Add, remove, list, and switch between GitHub accounts.
- **SSH Key Orchestration** – Generate RSA-4096 SSH keys and configure `~/.ssh/config` per account.
- **Profile Switching** – Switch globally or only for the current repository.
- **Intelligent Commit** – Auto-detects the correct account based on the repo's git log history.
- **Account-scoped Clone** – Clone repos with the right SSH identity pre-configured.

---

## How Intelligent Commit Works

When you run `gitsock commit`, it:

1. Checks for a local `git config` override first — if one exists, it commits under that identity.
2. If no local config exists, it scans the repo's git log and scores each configured account by how often their username appears.
3. If one account matches clearly, it sets that identity and commits.
4. If multiple accounts match equally, it prompts you to pick one.
5. Falls back to the globally active account if no matches are found.

This prevents accidentally committing company work from a personal account (or vice versa) without needing to think about it.

---

## Installation

### Option 1: Prebuilt Binary (Recommended)

1. Download the binary for your OS from the [Releases](https://github.com/bedantH/gitsock/releases) page.
2. On Linux/macOS, make it executable:

```sh
chmod +x gitsock
```

3. Run the setup command (places the binary in `~/gitsock/` and adds it to `PATH`):

```sh
./gitsock setup
```

Restart your terminal (or `source ~/.bashrc` / `source ~/.zshrc`) for the PATH change to take effect.

---

### Option 2: Build from Source

Requires [Rust](https://rustup.rs).

```sh
git clone https://github.com/bedantH/gitsock.git
cd gitsock
cargo build --release
./target/release/gitsock setup
```

---

## Requirements

- Git must be installed and available in `PATH`.
- For SSH features, your GitHub account must have a public email set. You can remove it after adding the account.
- `gitsock ssh add` requires the account to have an alias configured (set during `gitsock account add`).

---

## Commands

### Account Management

**Add an account** (opens browser for GitHub OAuth login):

```sh
gitsock account add
```

You will be shown a device code. Complete authentication in the browser, then optionally set an alias for the account.

**Remove an account:**

```sh
gitsock account remove --username <USERNAME>
```

**List all accounts:**

```sh
gitsock ls
```

**Switch active account:**

```sh
gitsock use <USERNAME or ALIAS>
```

Options:
- `-l, --local` — Apply the switch only to the current repository (must be inside a git repo).

**Show current active account:**

```sh
gitsock me
```

---

### SSH Management

**Set up SSH for an account:**

```sh
gitsock ssh add <USERNAME or ALIAS> [OPTIONS]
```

Options:
- `-d, --default` — Configure this key for the default `github.com` host (use for your primary account).

This generates an RSA-4096 key pair, writes an entry to `~/.ssh/config`, prints the public key, and tests the connection interactively.

> **Note:** The account must have an alias configured before running `ssh add`.

**List SSH connections:**

```sh
gitsock ssh ls
```

---

### Git Operations

**Commit with automatic account detection:**

```sh
gitsock commit -m "Your commit message"
```

Options:
- `-m, --message <MESSAGE>` — Commit message. If omitted, you will be prompted.
- `-a <USERNAME or ALIAS>` — Explicitly choose which account to commit as.

**Clone a repository:**

```sh
gitsock clone <SSH_URL> [USERNAME or ALIAS] [PATH]
```

- `SSH_URL` — Must be an SSH URL (e.g. `git@github.com:user/repo.git`). HTTPS URLs are not supported.
- `USERNAME or ALIAS` — Account to use. Omit to use the currently active account.
- `PATH` — Directory to clone into. Defaults to the repo name.

---

## Usage Examples

```sh
# Add a new GitHub account
gitsock account add

# Switch globally to a different account
gitsock use work-account

# Switch only for the current repo
gitsock use personal --local

# Commit using auto-detected account
gitsock commit -m "Fix login bug"

# Commit explicitly as a specific account
gitsock commit -m "Fix login bug" -a work-account

# Clone a repo using a specific account
gitsock clone git@github.com:myorg/repo.git work-account

# Set up SSH for an account (must have an alias)
gitsock ssh add work-account

# Set up SSH as the default github.com identity
gitsock ssh add personal --default
```

---

## Data Storage

Gitsock stores its data in `~/gitsock/`:

| Path | Contents |
|---|---|
| `~/gitsock/config.json` | Paths to data files |
| `~/gitsock/.config/accounts.json` | All registered accounts |
| `~/gitsock/.config/active.json` | Currently active account |
| `~/gitsock/.secret/secret.bin` | AES-256 encryption key |
| `~/gitsock/.secret/token.bin` | Encrypted OAuth token |

OAuth tokens are encrypted at rest using AES-256-GCM.

---

## Contributing

Issues and pull requests are welcome on the [GitHub repository](https://github.com/bedantH/gitsock).

---

## License

MIT — see [LICENSE](LICENSE).
