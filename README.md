# `bunnypmsl` — Smart browser bookmarks with Rust

[![Crates.io](https://img.shields.io/crates/v/bunnypmsl.svg?style=flat-square)](https://crates.io/crates/bunnypmsl)
[![Downloads](https://img.shields.io/crates/d/bunnypmsl.svg?style=flat-square)](https://crates.io/crates/bunnypmsl)
[![Contributors](https://img.shields.io/github/contributors/daveio/bunnypmsl.svg?style=flat-square)](https://github.com/daveio/bunnypmsl/graphs/contributors)
[![Stargazers](https://img.shields.io/github/stars/daveio/bunnypmsl.svg?style=flat-square)](https://github.com/daveio/bunnypmsl/stargazers)
[![License](https://img.shields.io/github/license/daveio/bunnypmsl?style=flat-square)](https://github.com/daveio/bunnypmsl/blob/master/LICENSE)

> A modern rust clone of [bunny1](https://github.com/ccheever/bunny1).

## Demo

Enter `gh facebook/react` in your browser's address bar to open the React repository on GitHub.

Or run the CLI:

```sh
$ bunnypmsl gh facebook/react
```

## Installation

Install from [crates.io](https://crates.io/crates/bunnypmsl):

```sh
# Install both CLI and server (3.9MB)
$ cargo install bunnypmsl

# Install just the CLI (1.4MB - recommended for terminal use only)
$ cargo install bunnypmsl --features cli --no-default-features

# Install just the server (3.6MB - recommended for web server deployments)
$ cargo install bunnypmsl --features server --no-default-features
```

Or build from the source:

```sh
# Clone the repository
$ git clone https://github.com/daveio/bunnypmsl.git
$ cd bunnypmsl

# Install both CLI and server
$ cargo install --path .

# Install just the CLI
$ cargo install --path . --features cli --no-default-features

# Install just the server
$ cargo install --path . --features server --no-default-features
```

## CLI Quickstart

Use `bunnypmsl` to open URLs directly from your terminal!

### Basic Usage

```sh
# Open GitHub
$ bunnypmsl gh

# Open Instagram Reels
$ bunnypmsl ig reels

# Open a specific GitHub repository
$ bunnypmsl gh facebook/react

# Preview URL without opening browser (dry-run)
$ bunnypmsl --dry-run gh facebook/react
# Output: https://github.com/facebook/react

# List all available commands with a beautiful table
$ bunnypmsl list
```

### Quick Examples

| CLI Command                   | What it does                              |
| ----------------------------- | ----------------------------------------- |
| `bunnypmsl gh`                | Open GitHub homepage                      |
| `bunnypmsl gh facebook/react` | Open facebook/react repository            |
| `bunnypmsl ig reels`          | Open Instagram Reels                      |
| `bunnypmsl tw @facebook`      | Open Twitter profile                      |
| `bunnypmsl r r/rust`          | Open r/rust subreddit                     |
| `bunnypmsl --dry-run meta ai` | Print Meta AI URL without opening         |
| `bunnypmsl --help`            | Show help information                     |
| `bunnypmsl --version`         | Show version information                  |
| `bunnypmsl list`              | Display all commands in a formatted table |

### Recommended: Create a Shell Alias

For even faster access, add an alias to your shell configuration:

```sh
# Add to ~/.bashrc or ~/.zshrc
alias b="bunnypmsl"

# Then use it like this:
$ b ig reels
$ b gh facebook/react
$ b list
```

## CLI Configuration

The bunnypmsl CLI supports optional configuration via a TOML file following the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html).

### Configuration File Location

Bunnypmsl uses different config file locations depending on how it's run:

**For CLI and manual server usage (`bunnypmsl serve`):**

- **Linux/macOS**: `~/.config/bunnypmsl/config.toml` (or `$XDG_CONFIG_HOME/bunnypmsl/config.toml` if set)
- **Windows**: `%APPDATA%\bunnypmsl\config.toml`

**For system service (`sudo bunnypmsl service install`):**

- **Linux**: `/etc/bunnypmsl/config.toml`

The config file is automatically created with sensible defaults when you first run bunnypmsl.

### Configuration Features

The CLI works perfectly fine without any configuration file. However, you can customise the following features:

#### 1. **Default Browser Selection**

Specify which browser to open URLs in:

```toml
# ~/.config/bunnypmsl/config.toml
browser = "firefox"  # or "chrome", "chromium", "safari", etc.
```

If not specified, the system default browser is used.

#### 2. **Custom Command Aliases**

Create your own personalised shortcuts:

```toml
[aliases]
work = "gh mycompany"
blog = "gh username/blog"
dotfiles = "gh username/dotfiles"
```

Then use them like any built-in command:

```sh
$ bunnypmsl work
# Opens: https://github.com/mycompany

$ bunnypmsl blog
# Opens: https://github.com/username/blog
```

#### 3. **Custom Default Search Engine**

Override Google as the fallback search engine:

```toml
default_search = "ddg"  # Options: "google" (default), "ddg", "bing"
```

When a command isn't recognised, it will search using your configured engine instead of Google.

#### 4. **Command History Tracking**

Track your recently used commands (enabled by default):

```toml
[history]
enabled = true
max_entries = 1000
```

History is stored at:

- **Linux/macOS**: `~/.local/share/bunnypmsl/history` (or `$XDG_DATA_HOME/bunnypmsl/history` if set)
- **Windows**: `%APPDATA%\bunnypmsl\history`

### Complete Configuration Example

Here's a full example with all available options:

```toml
# ~/.config/bunnypmsl/config.toml

# Browser to open URLs in (optional)
browser = "firefox"

# Custom command aliases (optional)
[aliases]
work = "gh mycompany"
blog = "gh username/blog"
dotfiles = "gh username/dotfiles"
notes = "gh username/notes"

# Default search engine when command not recognised (optional)
# Options: "google" (default), "ddg", "bing"
default_search = "ddg"

# Command history settings (optional)
[history]
enabled = true
max_entries = 1000

# Server configuration (for bunnypmsl serve) (optional)
[server]
port = 8000
address = "127.0.0.1"  # Use "0.0.0.0" for network access
log_level = "normal"   # Options: "normal", "debug", "critical", "off"
server_display_url = "https://bunny.example.com"  # Public URL shown on bindings page
```

### Platform-Specific Directory Structure

The CLI uses platform-appropriate directories for configuration and data:

| Platform        | Type          | Path                                                                               |
| --------------- | ------------- | ---------------------------------------------------------------------------------- |
| **Linux/macOS** | User Config   | `~/.config/bunnypmsl/config.toml`<br>(or `$XDG_CONFIG_HOME/bunnypmsl/config.toml`) |
| **Linux**       | System Config | `/etc/bunnypmsl/config.toml`<br>(when running as system service)                   |
| **Linux/macOS** | Data          | `~/.local/share/bunnypmsl/`<br>(or `$XDG_DATA_HOME/bunnypmsl/`)                    |
| **Windows**     | Config        | `%APPDATA%\bunnypmsl\config.toml`                                                  |
| **Windows**     | Data          | `%APPDATA%\bunnypmsl\`                                                             |

## Quickstart - Web Server

After [installing](#installation) bunnypmsl, start the server:

```sh
$ bunnypmsl serve
```

Or use Docker:

```sh
$ git clone https://github.com/daveio/bunnypmsl.git
$ cd bunnypmsl
$ docker compose up -d
```

Or build from source:

```sh
$ git clone https://github.com/daveio/bunnypmsl.git
$ cd bunnypmsl
$ cargo run -- serve
```

### Installing as a System Service

For production use on **Linux**, install bunnypmsl as a `systemd` service that starts automatically on boot:

```sh
# Install bunnypmsl first
$ cargo install bunnypmsl

# Install as system service (requires sudo, Linux only)
# Default: localhost only (127.0.0.1)
$ sudo bunnypmsl service install

# For network access (production servers)
$ sudo bunnypmsl service install --network

# The installer will:
# - Create /etc/systemd/system/bunnypmsl.service
# - Create /etc/bunnypmsl/config.toml with server settings
# - Enable autostart on boot
# - Start the service immediately

# Manage the service
$ sudo bunnypmsl service status
$ sudo bunnypmsl service logs -f
$ sudo bunnypmsl service restart

# Uninstall
$ sudo bunnypmsl service uninstall
```

**Network Access:**

- **Without `--network`** (default): Binds to `127.0.0.1` (localhost only, secure default)
- **With `--network`**: Binds to `0.0.0.0` (accessible from network, for production servers)

The service installer works on:

- **Linux**: `systemd` (Ubuntu 16.04+, Debian 8+, CentOS 7+, etc.)

**macOS and Windows:** Use Docker instead (see above) or run `bunnypmsl serve` directly.

For more details, see the [Deployment Guide](deploy/DEPLOYMENT.md).

Open your web browser and navigate to `http://localhost:8000/?cmd=fb` to get redirected to Facebook.

Open `http://localhost:8000/?cmd=gh daveio/bunnypmsl` to be redirected to this repo.

## Setting `bunnypmsl` to be your default search engine

You can set your default search engine to `http://localhost:8000/?cmd=%s` and use `bunnypmsl` for everything. For this to work, you will need to have the server deployed and running locally or on a server.

**Note:** For best results, deploy bunnypmsl on a networked server accessible from all your devices, rather than just running it locally.

### Desktop Browsers

- [Guide for doing this in Desktop Chrome](https://support.google.com/chrome/answer/95426?hl=en&co=GENIE.Platform%3DDesktop)
- [Guide for doing this in Desktop Firefox](https://support.mozilla.org/en-US/kb/add-custom-search-engine-firefox)

### Mobile Browsers

**Note:** iOS Safari does not support custom search engines, so you'll need to use Firefox (or another browser that does) instead.

#### iOS (Firefox)

1. Install Firefox and [set it as the default browser](https://support.covenanteyes.com/hc/en-us/articles/12223357002267-How-do-I-set-a-default-browser-on-an-iPhone)
2. Change your [default search engine in Firefox for iOS](https://support.mozilla.org/en-US/kb/change-your-default-search-engine-firefox-ios)

#### Android (Firefox)

- [Guide for managing default search engines in Firefox for Android](https://support.mozilla.org/en-US/kb/manage-my-default-search-engines-firefox-android)

<!-- USAGE EXAMPLES -->

## Command Reference

<details>
<summary><strong>📚 Click to view all available commands (46 commands, 82+ bindings)</strong></summary>

<br>

### Development & Package Managers

| Command     | Aliases           | Description                                                         | Example                    |
| ----------- | ----------------- | ------------------------------------------------------------------- | -------------------------- |
| `gh`        | —                 | Navigate to GitHub repositories                                     | `gh facebook/react`        |
| `gitlab`    | `gl`              | Navigate to GitLab projects or search GitLab                        | `gitlab gitlab-org/gitlab` |
| `cargo`     | `crates`          | Navigate to crates.io or search for Rust crates                     | `cargo serde`              |
| `npm`       | `npmjs`           | Navigate to npmjs.com or search for npm packages                    | `npm react`                |
| `pypi`      | `pip`             | Navigate to pypi.org or search for Python packages                  | `pypi requests`            |
| `rubygems`  | `gem`, `gems`     | Navigate to rubygems.org or search for Ruby gems                    | `gem rails`                |
| `go`        | `golang`, `gopkg` | Navigate to pkg.go.dev or search for Go packages                    | `go http`                  |
| `nuget`     | —                 | Navigate to nuget.org or search for .NET packages                   | `nuget newtonsoft`         |
| `packagist` | `composer`        | Navigate to packagist.org or search for PHP packages                | `packagist symfony`        |
| `brew`      | `homebrew`        | Navigate to formulae.brew.sh or search for Homebrew packages        | `brew wget`                |
| `choco`     | `chocolatey`      | Navigate to community.chocolatey.org or search for Windows packages | `choco git`                |
| `dockerhub` | `docker`          | Navigate to Docker Hub or search for container images               | `docker nginx`             |

### Programming Documentation

| Command         | Aliases        | Description                                                      | Example             |
| --------------- | -------------- | ---------------------------------------------------------------- | ------------------- |
| `rust`          | —              | Navigate to Rust documentation or search Rust std docs           | `rust HashMap`      |
| `python`        | `pydocs`, `py` | Navigate to Python documentation or search for Python resources  | `python list`       |
| `node`          | `nodejs`       | Navigate to Node.js API documentation or specific module docs    | `node fs`           |
| `godocs`        | —              | Navigate to Go language documentation                            | `godocs`            |
| `hack`          | —              | Navigate to Hack documentation or search Hack docs               | `hack async`        |
| `mdn`           | —              | Navigate to MDN Web Docs or search for web development resources | `mdn flexbox`       |
| `stackoverflow` | `so`           | Navigate to Stack Overflow or search for programming questions   | `so rust ownership` |

### Social Media

| Command   | Aliases     | Description                                                           | Example                |
| --------- | ----------- | --------------------------------------------------------------------- | ---------------------- |
| `ig`      | `instagram` | Navigate to Instagram profiles, search, or access Reels/Messages      | `ig @instagram`        |
| `tw`      | —           | Navigate to Twitter profiles or search Twitter                        | `tw @MetaOpenSource`   |
| `threads` | —           | Navigate to Threads profiles or search Threads                        | `threads @zuck`        |
| `fb`      | —           | Navigate to Facebook pages or search Facebook                         | `fb Meta`              |
| `li`      | `linkedin`  | Navigate to LinkedIn or search                                        | `li software engineer` |
| `reddit`  | `r`         | Navigate to Reddit or search subreddits                               | `r r/rust`             |
| `yt`      | `youtube`   | Navigate to YouTube or search for videos (supports: `studio`, `subs`) | `yt rust programming`  |
| `wa`      | `whatsapp`  | Navigate to WhatsApp                                                  | `wa`                   |

### Google Services

| Command   | Aliases   | Description                                                   | Example               |
| --------- | --------- | ------------------------------------------------------------- | --------------------- |
| `g`       | (default) | Search Google (default fallback for any unrecognised command) | `g rust programming`  |
| `gmail`   | `mail`    | Navigate to Gmail                                             | `mail`                |
| `docs`    | `gdoc`    | Navigate to Google Docs                                       | `docs`                |
| `gsheets` | —         | Navigate to Google Sheets                                     | `gsheets`             |
| `gslides` | —         | Navigate to Google Slides                                     | `gslides`             |
| `gchat`   | —         | Navigate to Google Chat                                       | `gchat`               |
| `gmaps`   | `maps`    | Navigate to Google Maps or search for a location              | `gmaps san francisco` |

### Meta / AI Services

| Command   | Aliases  | Description                                                                           | Example           |
| --------- | -------- | ------------------------------------------------------------------------------------- | ----------------- |
| `meta`    | `metaai` | Navigate to Meta, Meta AI, Meta Accounts Center, or Meta Pay                          | `meta accounts`   |
| `claude`  | —        | Navigate to Claude AI (supports: `billing`, `cost`, `artifacts`, `chats`, `projects`) | `claude projects` |
| `chatgpt` | —        | Navigate to ChatGPT                                                                   | `chatgpt`         |

### Shopping & Finance

| Command  | Aliases                          | Description                                                                                | Example                 |
| -------- | -------------------------------- | ------------------------------------------------------------------------------------------ | ----------------------- |
| `az`     | `amzn`, `azn`, `amazon`          | Navigate to Amazon or search for products                                                  | `az headphones`         |
| `rei`    | —                                | Navigate to REI or search for outdoor gear                                                 | `rei hiking boots`      |
| `schwab` | —                                | Charles Schwab shortcuts (`billpay`, `orders`, `trade`, `transfer`, `security`, `contact`) | `schwab trade`          |
| `stock`  | `stocks`, `finance`, `$<ticker>` | Look up stock prices on Yahoo Finance                                                      | `stock META` or `$META` |

### Other Services

| Command      | Aliases             | Description                                | Example                 |
| ------------ | ------------------- | ------------------------------------------ | ----------------------- |
| `1password`  | `1p`, `onepassword` | 1Password home page                        | `1p`                    |
| `soundcloud` | `sc`                | Navigate to SoundCloud (supports: `likes`) | `sc edm`                |
| `wiki`       | `wikipedia`         | Search on Wikipedia                        | `wiki rust programming` |
| `ddg`        | `duckduckgo`        | Search DuckDuckGo                          | `ddg rust programming`  |

### Bunnypmsl Development Tools

| Command    | Aliases                                             | Description                                         | Example    |
| ---------- | --------------------------------------------------- | --------------------------------------------------- | ---------- |
| `bindings` | `commmands`, `list`, `bunny`, `cmd`, `cmds`, `help` | View all Bunnypmsl command bindings in a web portal | `bindings` |

### Special Syntax

- **Stock tickers**: Prefix with `$` → `$AAPL` redirects to Yahoo Finance
- **Twitter profiles**: Prefix with `@` → `tw @username`
- **Instagram profiles**: Prefix with `@` → `ig @username`
- **Threads profiles**: Prefix with `@` → `threads @username`
- **Subreddits**: Use `r/` prefix → `r r/rust`
- **Default fallback**: Any unrecognised command searches Google

</details>

### Built With

- [Rust](https://www.rust-lang.org/)
- [Rocket](https://rocket.rs/) - Web framework
- [Leptos](https://leptos.dev/) - Frontend framework for the bindings page
- [clap](https://github.com/clap-rs/clap) - CLI argument parser
- [tabled](https://github.com/zhiburt/tabled) - Beautiful terminal tables

<!-- GETTING STARTED -->

## Getting Started

See the [Installation](#installation) section to install bunnypmsl from crates.io.

To build from the source or contribute to the project, read on.

### Manual Setup

Make sure you have [Rust installed](https://rust-lang.org/tools/install/).

```sh
$ git clone https://github.com/daveio/bunnypmsl.git
$ cd bunnypmsl

# Run the web server
$ cargo run -- serve

# OR run the CLI (in a separate terminal)
$ cargo run -- gh facebook/react

# OR install globally for easier access
$ cargo install --path .
```

## Deployment with Docker

`Bunnypmsl` is designed to be easy to deploy anywhere using Docker.

```sh
# run on default port 8000
$ docker compose up -d

# run on custom port 9000
$BUNNYPMSL_PORT=9000·docker compose up
```

The application will be running at `http://localhost:8000` by default.

### Where to Deploy

Docker makes it easy to deploy anywhere:

- Any cloud provider (AWS, GCP, Azure, DigitalOcean, Hetzner, etc.)
- VPS / home servers

For detailed deployment instructions, reverse proxy setup, and troubleshooting, see the **[Deployment Guide](deploy/DEPLOYMENT.md)**.

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**. See [`CONTRIBUTING`](CONTRIBUTING.md) for more information.

## License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for more information.

## Acknowledgments

- [The Rust Community](https://www.rust-lang.org/community)
- [Rocket.rs](https://rocket.rs/)
- [@othneildrew](https://github.com/othneildrew) - for the [README template](https://github.com/othneildrew/Best-README-Template)
