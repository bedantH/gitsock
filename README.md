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
2. Download the binary for your OS from Releases.  
3. Give execute permissions to the gitsock binary on Linux/MacOS:
```sh
chmod +x gitsock
````
4. Run `gitsock setup` command to add gitsock to you PATH env, it will complete the initial setup on its own, and then you will be ready to use it right away. 

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

4. Run in-built setup command:

```sh
gitsock setup
```
This will setup ENV path variables and create the appropriate folders for you.

---

## Commands

### Account Management

* **Add Account**

**NOTE**: Before adding an account, please make sure you have a public email on your GitHub, which will allow gitsock to access email and username from your GitHub account. Once you've logged in you can remove the public email later on!
```sh
gitsock account add
```
This will show a 6-digit device code, unique to your system and open a browser with the designated login URL, complete the login process from there.

* **Remove Account**

```sh
gitsock account remove <USERNAME>
```

* **List Accounts**

```sh
gitsock ls
```

* **Switch Accounts**

```sh
gitsock use <USERNAME>
```

Options: `-l, --local` Switch account only for current repository (PWD should be a Git Repository)

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
gitsock ssh ls
```

---

### Git Operations

* **Commit Changes**

```sh
gitsock commit -m "Commit message"
```
Options: `-a` Mention which account you want to choose for this commit, this is optional.

(If no `-m`, you’ll be prompted for a message.)

* **Clone Repository**

```sh
gitsock clone <URL> [OPTIONS]
```

Options:
- `[USERNAME or ALIAS]` → Specify account to use for cloning
- `[PATH]` → Path where you want to clone the repository to

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
gitsock clone https://github.com/myuser/repo.git -a myuser
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
Open issues and pull requests on the [GitHub repository](https://github.com/bedantH/gitsock).

---

## License

Gitsock is released under the MIT license.
See the [LICENSE](https://github.com/bedantH/gitsock/blob/master/LICENSE) file for details.
