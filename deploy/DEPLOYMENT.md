# Server Deployment Guide for Bunnypmsl

This guide covers deploying `bunnypmsl` using either native service installation or Docker.

## Table of Contents

- [Quick Start: Automated Setup](#quick-start-automated-setup)
- [Native Service Installation](#native-service-installation)
- [Docker Deployment](#docker-deployment)
- [Rebuilding and Redeploying](#rebuilding-and-redeploying)
- [Auto-Deployment](#auto-deployment)
- [Configuration](#configuration)
- [Running on Boot](#running-on-boot)
- [Reverse Proxy Setup](#reverse-proxy-setup)
- [Troubleshooting](#troubleshooting)

## Quick Start: Automated Setup

For new Ubuntu cloud machines (Ubuntu 22.04+), we provide an automated setup script that installs Rust, bunnypmsl, and sets it up as a `systemd` service in one command.

### Prerequisites

- Linux server running systemd
- `cargo` installed
- Root or sudo access
- Internet connection

### Usage

Download and run the setup script:

```bash
curl -fsSL https://raw.githubusercontent.com/alichtman/bunnypmsl/main/deploy/setup-ubuntu-server.sh -o setup-ubuntu-server.sh
chmod +x setup-ubuntu-server.sh
sudo ./setup-ubuntu-server.sh
```

Or clone the repository first and run locally:

```bash
git clone https://github.com/alichtman/bunnypmsl.git
cd bunnypmsl
sudo deploy/setup-ubuntu-server.sh
```

### What the Script Does

The automated setup script will:

1. ✓ Verify you're running a supported Ubuntu version
2. ✓ Update system packages
3. ✓ Install build prerequisites (build-essential, pkg-config, libssl-dev, etc.)
4. ✓ Install Rust (rustup)
5. ✓ Install bunnypmsl from crates.io (`cargo install bunnypmsl`)
6. ✓ Install bunnypmsl as a `systemd` service
7. ✓ Configure service to start on boot
8. ✓ Start the service immediately
9. ✓ Verify the installation

After completion, bunnypmsl will be running on port 8000 as a `systemd` service. The script is safe to run multiple times - it will skip already-installed components and update Rust if it already exists.

---

## Native Service Installation

The recommended deployment method for **Linux** is to install bunnypmsl as a native `systemd` service. This provides better integration with your OS and doesn't require Docker.

**Note:** Native service installation is only supported on Linux (`systemd`). For macOS and Windows, please use [Docker Deployment](#docker-deployment) instead.

### Native Service Prerequisites

- Rust (only needed if binary not in PATH)
- **Linux with `systemd`** (Ubuntu 16.04+, Debian 8+, CentOS 7+, etc.)
- sudo/root access

### Installation

```bash
# Install bunnypmsl first
$ cargo install bunnypmsl

# Install as system service (requires sudo, Linux only)
# Default: localhost only (127.0.0.1)
$ sudo bunnypmsl service install

# For network access (production servers)
$ sudo bunnypmsl service install --network

# The installer will:
# - Find the bunnypmsl binary in PATH
# - Create systemd service file at /etc/systemd/system/bunnypmsl.service
# - Create config file at /etc/bunnypmsl/config.toml (if not exists)
# - Configure autostart on boot (always enabled)
# - Start the service immediately
```

**Network Access:**

- **Without `--network`** (default): Binds to `127.0.0.1` (localhost only, secure default)
- **With `--network`**: Binds to `0.0.0.0` (accessible from network, for production servers)

Configuration is managed through the system config file at `/etc/bunnypmsl/config.toml`:

- **Port**: 8000 (default)
- **Address**: 127.0.0.1 (default) or 0.0.0.0 (with `--network` flag)
- **Autostart**: Enabled (always)
- **Run as**: root
- **Auto-restart**: On failure (5 second delay)

To customize these settings after installation, edit `/etc/bunnypmsl/config.toml` and restart the service.

### Managing the Service

```bash
# Check service status
$ sudo bunnypmsl service status

# View logs (last 20 lines)
$ sudo bunnypmsl service logs

# View logs (follow mode)
$ sudo bunnypmsl service logs -f

# View more lines
$ sudo bunnypmsl service logs -n 100

# Restart the service
$ sudo bunnypmsl service restart

# Stop the service
$ sudo bunnypmsl service stop

# Start the service
$ sudo bunnypmsl service start
```

### Customizing Server Settings

Server settings for the system service are configured in `/etc/bunnypmsl/config.toml` (created automatically during installation):

```toml
[server]
port = 8000              # Change to your preferred port
address = "127.0.0.1"    # Use "127.0.0.1" for localhost, "0.0.0.0" for network access
log_level = "normal"     # Options: normal, debug, critical
```

**Network Access:**

- `address = "127.0.0.1"` - Localhost only (secure default, installed without `--network`)
- `address = "0.0.0.0"` - Network accessible (production servers, installed with `--network`)

After editing the config file, restart the service:

```bash
sudo bunnypmsl service restart
```

**Note:** For running `bunnypmsl serve` manually (non-service), the config file is at `~/.config/bunnypmsl/config.toml`.

### Uninstalling

```bash
# Uninstall system service (stops service and removes systemd files)
$ sudo bunnypmsl service uninstall
```

### Platform-Specific Details

#### Linux (`systemd`)

- Service file: `/etc/systemd/system/bunnypmsl.service`
- Logs: `journalctl -u bunnypmsl -f` or `sudo bunnypmsl service logs -f`
- Binary location: Detected automatically from PATH (typically `/usr/local/bin/bunnypmsl`)
- Service runs as: root
- Restart policy: On failure with 5 second delay

#### macOS / Windows

Native service installation is **not supported** on macOS or Windows.

Recommended alternatives:\*\*
**Docker**: Use `docker compose up -d` (see [Docker Deployment](#docker-deployment))
**Direct run**: Use `bunnypmsl serve` (runs in foreground, doesn't auto-start on boot)

---

## Docker Deployment

Docker provides an alternative deployment method that's useful for containerized environments.

### Using Docker Compose

The easiest way to deploy bunnypmsl with Docker is using Docker Compose:

1. **Clone the repository**:

   ```bash
   git clone https://github.com/daveio/bunnypmsl.git
   cd bunnypmsl
   ```

2. **Start the service**:

   ```bash
   docker compose up -d
   ```

   To use a custom port, set the `BUNNYPMSL_PORT` environment variable:

   ```bash
   BUNNYPMSL_PORT=9000 docker compose up -d
   ```

   This maps port 9000 on the host to port 8000 in the container.

3. **Access the application**:
   Open your browser to `http://localhost:8000`

4. **View logs**:

   ```bash
   docker compose logs -f
   ```

5. **Stop the service**:
   ```bash
   docker compose down
   ```

### Using Docker directly

1. **Build the image**:

   ```bash
   docker build -t bunnypmsl .
   ```

2. **Run the container**:
   ```bash
   docker run -d \
     --name bunnypmsl \
     -p 8000:8000 \
     --restart unless-stopped \
     bunnypmsl
   ```

## Rebuilding and Redeploying

When you've made code changes and need to deploy them to your running server:

### Quick Rebuild (Recommended)

The simplest way to rebuild and redeploy:

```bash
docker compose up --build -d
```

This command will:

- Build a new image with your latest changes
- Stop the old container
- Start a new container with the updated image

### Full Rebuild (Clean Build)

If you need to rebuild without using cached layers:

```bash
docker compose down
docker compose build --no-cache
docker compose up -d
```

### Remote Server Rebuild

If your server is running on a remote machine (e.g., Hetzner, AWS, etc.):

1. **SSH into your server and navigate to the project directory**:

   ```bash
   ssh your-server
   cd bunnypmsl
   ```

2. **Pull the latest changes** (if using Git):

   ```bash
   git pull
   ```

3. **Rebuild and redeploy**:

   ```bash
   docker compose up --build -d
   ```

4. **Verify the deployment**:
   ```bash
   docker ps
   docker logs --tail=20 bunnypmsl
   ```

### One-Liner for Remote Rebuild

If you have SSH configured with a host alias (e.g., `hetzner`), you can rebuild from your local machine:

```bash
ssh your-server "cd bunnypmsl && git pull && docker compose up --build -d"
```

### Verifying the Deployment

After rebuilding, check that:

- The container was created recently: `docker ps` (check CREATED column)
- The application is running: `curl http://localhost:8000/health`
- Logs look healthy: `docker logs --tail=50 bunnypmsl`

## Auto-Deployment

For production servers, you can set up automatic deployment that checks for upstream changes and redeploys automatically.

### How It Works

The auto-deployment system:

1. Checks for new commits on the remote repository every 5 minutes
2. If changes are detected, pulls them and rebuilds the Docker container
3. Logs all activity to `/var/log/bunnypmsl-deploy.log`
4. Only rebuilds when there are actual changes (no unnecessary rebuilds)

### Setup

On your server, run the setup script:

```bash
sudo /path/to/bunnypmsl/deploy/setup-auto-deploy.sh
```

Or if using an SSH alias:

```bash
ssh your-server "sudo /root/bunnypmsl/deploy/setup-auto-deploy.sh"
```

This will:

- Make the auto-deploy script executable
- Create the log file and directory
- Configure git settings for automated pulling
- Set up a cron job to run every 5 minutes
- Test the deployment script

### Customization

You can customize the behavior with environment variables:

```bash
# Check every 10 minutes instead of 5
CRON_SCHEDULE="*/10 * * * *" sudo deploy/setup-auto-deploy.sh

# Use a different branch
BRANCH="production" sudo deploy/setup-auto-deploy.sh

# Custom log location
LOG_FILE="/var/log/custom-deploy.log" sudo deploy/setup-auto-deploy.sh
```

### Monitoring

**View deployment logs:**

```bash
tail -f /var/log/bunnypmsl-deploy.log
```

**Check cron job status:**

```bash
crontab -l
```

**Manually trigger deployment:**

```bash
sudo /path/to/bunnypmsl/deploy/auto-deploy.sh
```

### Removing Auto-Deployment

To disable auto-deployment:

```bash
# Remove the cron job
crontab -e
# Delete the line containing "auto-deploy.sh"
```

## Configuration

### Config File Locations

Bunnypmsl uses different config file locations depending on how it's run:

- **System service** (installed with `sudo bunnypmsl service install`): `/etc/bunnypmsl/config.toml`
- **User/manual run** (running `bunnypmsl serve` directly): `~/.config/bunnypmsl/config.toml`

The config file is automatically created with defaults if it doesn't exist.

### Native Service

For native systemd installations, configuration is in `/etc/bunnypmsl/config.toml`.

Example configuration:

```toml
# Browser to open URLs in (optional)
# browser = "firefox"

# Default search engine when command not recognized
# Options: "google" (default), "ddg", "bing"
default_search = "google"

# Custom command aliases
[aliases]
# work = "gh mycompany/repo"

# Command history settings
[history]
enabled = true
max_entries = 1000

# Server configuration
[server]
port = 8000
address = "127.0.0.1"    # Use "0.0.0.0" for network access
log_level = "normal"     # Options: normal, debug, critical
```

After editing the config file, restart the service:

```bash
sudo bunnypmsl service restart
```

### Docker

For Docker deployments, you can customize the host port using the `BUNNYPMSL_PORT` environment variable:

```bash
# .env file (optional)
BUNNYPMSL_PORT=9000
```

Then start with:

```bash
docker compose up -d
```

Or set it inline:

```bash
BUNNYPMSL_PORT=9000 docker compose up -d
```

To customize other settings in Docker, you can mount a config file:

```yaml
# docker-compose.yml
services:
  bunnypmsl:
    volumes:
      - ./config:/etc/bunnypmsl
```

Then create a local `config/config.toml` file with your settings, and it will be mounted into the container at `/etc/bunnypmsl/config.toml`.

## Running on Boot

Docker containers can automatically start on system boot using restart policies.

### Enable Docker Service

First, ensure the Docker daemon starts on boot:

```bash
sudo systemctl enable docker
```

### Restart Policies

When running containers, use a restart policy:

**With Docker Compose** (add to your `docker-compose.yml`):

```yaml
services:
  bunnypmsl:
    restart: unless-stopped
```

**With Docker run**:

```bash
docker run -d --restart unless-stopped ...
```

Available restart policies:

- `always`: Always restart, even if manually stopped and system reboots
- `unless-stopped`: Restart unless explicitly stopped by user
- `on-failure`: Only restart on crashes

## Reverse Proxy Setup

For production deployments with HTTPS, use a reverse proxy like Caddy or nginx.

> TODO: Finish this section

<!-- ### Using Caddy (Easiest for HTTPS) -->
<!---->
<!-- 1. **Install Caddy**: -->
<!--    ```bash -->
<!--    sudo apt install -y debian-keyring debian-archive-keyring apt-transport-https curl -->
<!--    curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg -->
<!--    curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list -->
<!--    sudo apt update -->
<!--    sudo apt install caddy -->
<!--    ``` -->
<!---->
<!-- 2. **Configure Caddy**: -->
<!--    ```bash -->
<!--    sudo nano /etc/caddy/Caddyfile -->
<!--    ``` -->
<!---->
<!--    Add: -->
<!--    ``` -->
<!--    your-domain.com { -->
<!--        reverse_proxy localhost:8000 -->
<!--    } -->
<!--    ``` -->
<!---->
<!-- 3. **Restart Caddy**: -->
<!--    ```bash -->
<!--    sudo systemctl restart caddy -->
<!--    ``` -->
<!---->
<!-- Caddy automatically handles SSL certificates via Let's Encrypt. -->
<!---->
<!-- ### Using nginx -->
<!---->
<!-- 1. **Install nginx**: -->
<!--    ```bash -->
<!--    sudo apt install nginx -->
<!--    ``` -->
<!---->
<!-- 2. **Configure nginx**: -->
<!--    ```bash -->
<!--    sudo nano /etc/nginx/sites-available/bunnypmsl -->
<!--    ``` -->
<!---->
<!--    Add: -->
<!--    ```nginx -->
<!--    server { -->
<!--        listen 80; -->
<!--        server_name your-domain.com; -->
<!---->
<!--        location / { -->
<!--            proxy_pass http://localhost:8000; -->
<!--            proxy_set_header Host $host; -->
<!--            proxy_set_header X-Real-IP $remote_addr; -->
<!--        } -->
<!--    } -->
<!--    ``` -->
<!---->
<!-- 3. **Enable and restart**: -->
<!--    ```bash -->
<!--    sudo ln -s /etc/nginx/sites-available/bunnypmsl /etc/nginx/sites-enabled/ -->
<!--    sudo systemctl restart nginx -->
<!--    ``` -->

## Troubleshooting

### Docker Issues

**Container won't start:**

```bash
# Check logs
docker compose logs bunnypmsl

# Check if port is already in use
sudo netstat -tlnp | grep 8000
```

**Permission denied errors:**

```bash
# Add your user to docker group
sudo usermod -aG docker $USER
# Log out and back in
```

**Container not starting on boot:**

```bash
# Verify Docker service is enabled
sudo systemctl status docker

# Check restart policy
docker inspect bunnypmsl | grep -A 3 RestartPolicy
```

### Build Issues

**Docker build fails:**

```bash
# Clean build cache
docker system prune -a

# Rebuild without cache
docker build --no-cache -t bunnypmsl .
```

## Support

For issues or questions:

- GitHub Issues: <https://github.com/daveio/bunnypmsl/issues>
