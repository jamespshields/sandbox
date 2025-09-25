#!/bin/bash

# Setup Node.js environment via nvm
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"

# Verify Node.js and Claude are available
if ! command -v node >/dev/null 2>&1; then
    echo "Warning: Node.js not found"
fi

if ! command -v claude >/dev/null 2>&1; then
    echo "Warning: Claude command not found"
fi

# If arguments are provided, handle them safely; otherwise start interactive bash
if [ $# -gt 0 ]; then
    # Only allow specific whitelisted commands for security
    case "$1" in
        "claude")
            # Remove the first argument (claude) and pass the rest
            shift
            exec claude "$@"
            ;;
        "bash"|"sh")
            exec bash
            ;;
        *)
            echo "Error: Command '$1' not allowed in sandbox"
            echo "Allowed commands: claude, bash, python3"
            exit 1
            ;;
    esac
else
    exec bash
fi
