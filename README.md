# Gitsock: Your Personalized Git Management Assistant

Gitsock is a lightweight command-line tool that makes managing multiple Git accounts effortless.

Whether you’re switching between personal projects, client work, or company repos, Gitsock handles the context for you so Git always behaves as expected — no manual tweaks, no mistakes, just smooth workflows.

---

## Features

- **Multi-Account Management** – Add, remove, list, and switch between accounts in seconds.  
- **SSH Key Orchestration** – Manage multiple SSH identities and set defaults effortlessly.  
- **Seamless Repo Operations** – Clone, commit, and push with the correct account every time.  
- **Profile Switching** – Jump between accounts with a single command.  
- **Intelligent Commit** – Automatically validates commit authorship and prevents mistakes.  

---

## Intelligent Commit

One of Gitsock’s standout features is Intelligent Commit.  

When you commit changes, Gitsock:  
- Detects your active profile before committing.  
- Validates that the commit author and email match the current account.  
- Warns or auto-corrects if you’re about to commit under the wrong identity.  

This ensures you never accidentally push company code from your personal account, or vice versa. It saves time, prevents errors, and removes the need for fixing history later with `git rebase`.  

---

## Installation

### Option 1: Download Prebuilt Binary (Recommended)

1. Go to the [Releases](https://github.com/bedantH/gitsock/releases) page.  
2. Download the binary for your OS:

   * **Linux** → `gitsock-linux-x86_64`  
   * **macOS** → `gitsock-macos`  
   * **Windows** → `gitsock-windows-x86_64.exe`  

3. Move the binary to a directory in your **PATH**.  

Example (Linux/macOS):  

```sh
chmod +x gitsock-linux-x86_64
sudo mv gitsock-linux-x86_64 /usr/local/bin/gitsock
````

Example (Windows PowerShell):

```powershell
move .\gitsock-windows-x86_64.exe C:\Program Files\Gitsock\gitsock.exe
```

Then add `C:\Program Files\Gitsock\` to your system **PATH**.

---

### Option 2: Build from Source

1. Install [Rust](https://rustup.rs).
2. Clone the repository:

```sh
git clone https://github.com/your-repo/gitsock.git
cd gitsock
```

3. Build the binary:

```sh
cargo build --release
```

4. Copy it into your PATH:

```sh
sudo mv target/release/gitsock /usr/local/bin/
```

---

## Commands

### Account Management

* **Add Account**

```sh
gitsock account add
```

* **Remove Account**

```sh
gitsock account remove <USERNAME>
```

* **List Accounts**

```sh
gitsock list
```

* **Switch Accounts**

```sh
gitsock use <USERNAME>
```

* **Current Account**

```sh
gitsock me
```

---

### SSH Management

* **Add SSH Connection**

```sh
gitsock ssh add [OPTIONS] <USERNAME or ALIAS>
```

Options:
`-d, --default` → Set as default SSH account

* **List SSH Connections**

```sh
gitsock ssh list
```

---

### Git Operations

* **Commit Changes**

```sh
gitsock commit -m "Commit message"
```

(If no `-m`, you’ll be prompted for a message.)

* **Clone Repository**

```sh
gitsock clone [OPTIONS] <URL>
```

Options:
`-u, --username <USERNAME>` → Specify account for cloning

---

## Usage Examples

* **Add a New Account**

```sh
gitsock account add
```

* **Remove an Account**

```sh
gitsock account remove myuser
```

* **Clone a Repo with a Specific Account**

```sh
gitsock clone https://github.com/myuser/repo.git -u myuser
```

* **Commit Changes**

```sh
gitsock commit -m "Updated README.md"
```

* **Switch Accounts**

```sh
gitsock use myuser
```

---

## Contributing

We welcome contributions from the community!
Open issues and pull requests on the [GitHub repository](https://github.com/your-repo/gitsock).

---

## License

Gitsock is released under the MIT license.
See the [LICENSE](LICENSE) file for details.
