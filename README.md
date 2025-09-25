# sb - Sandbox CLI Tool

A command-line tool for managing containerized development environments with Docker.

## Features

- **Container Management**: Automatically creates and manages Docker containers for isolated development environments
- **Claude Code Integration**: Provides seamless integration with Claude Code environments
- **Directory-based Containers**: Each directory gets its own uniquely named container based on the directory path
- **Persistent Storage**: Maintains persistent volumes for your development work

## Installation

### Build from Source

1. Ensure you have Rust and Docker installed on your system
2. Clone this repository
3. Build the project:

```bash
make build
```

4. Add the binary to your PATH:

```bash
export PATH="$(pwd)/bin:$PATH"
```

## Usage

### Basic Shell Access

Launch a shell in a sandbox container for the current directory:

```bash
sb
```

### Claude Code Environment

Launch Claude Code in a containerized environment:

```bash
sb claude
```

Pass additional arguments to Claude Code:

```bash
sb claude --help
sb claude [other-claude-args]
```

### Clean Up

Remove all sandbox containers, images, and volumes:

```bash
sb clean
```

## How It Works

- Each directory gets a unique container name based on a hash of the directory path
- Containers are created using Docker Compose with persistent volumes
- The tool automatically handles container lifecycle (creation, starting, stopping)
- Files in your working directory are mounted into the container for development

## Requirements

- Rust (for building)
- Docker
- Docker Compose

## Project Structure

- `src/main.rs` - CLI interface and main application logic
- `src/bridge.rs` - Docker container management functions
- `src/constants.rs` - Configuration constants
- `templates/` - Docker configuration templates
- `Makefile` - Build and cleanup scripts

## License

See LICENSE file for details.