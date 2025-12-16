# Go Debug

This extension provides an enhanced Go debugging experience for Zed, aiming to match the convenience and power of GoLand. It wraps the Delve debugger (`dlv`) and provides configuration templates for common debugging scenarios.

## Features

- **Custom Debug Adapter**: Registers `go-debug` adapter.
- **Auto-detection**: Smart detection of `dlv` binary in common locations (`$GOPATH/bin`, Homebrew, etc.).
- **GoLand-like Workflows**: Instructions and configurations to replicate GoLand's debugging capabilities.

## Prerequisites

You must have [Delve](https://github.com/go-delve/delve) installed:

```bash
go install github.com/go-delve/delve/cmd/dlv@latest
```

Ensure `dlv` is in your `PATH` or `$GOPATH/bin`.

## Usage

To use the "GoLand-like" debugging features, you should configure your `.zed/debug.json` with the following templates.

### 1. Debug Main (Current Package)

Use this to debug the `main` package in the current directory.

```json
{
  "label": "Debug Main",
  "program": "go",
  "args": ["run", "."],
  "env": {},
  "adapter": "go-debug"
}
```

*Note: Zed's built-in Go support handles `func main` automatically, but you can customize it here.*

### 2. Debug Test (Current Package)

```json
{
  "label": "Debug Tests",
  "program": "go",
  "args": ["test", "-v", "."],
  "adapter": "go-debug"
}
```

### 3. Attach to Process (GoLand "Attach to Process")

This requires running `dlv` in headless mode or using `dlv attach` externally, but via DAP we can attach to a running server if configured.

However, the most common "Attach" workflow in local dev is attaching to a running local process.

```json
{
  "label": "Attach to Process ID",
  "adapter": "go-debug",
  "configuration": {
    "request": "attach",
    "mode": "local",
    "processId": "${integer_prompt:Process ID}"
  }
}
```

### 4. Remote Debugging

Connect to a headless Delve server running on a remote machine (or Docker container).

```json
{
  "label": "Remote Debug",
  "adapter": "go-debug",
  "configuration": {
    "request": "attach",
    "mode": "remote",
    "port": 2345,
    "host": "127.0.0.1",
    "substitutePath": [
      {
        "from": "${workspaceFolder}",
        "to": "/app"
      }
    ]
  }
}
```

## Recommended Keybindings (GoLand Style)

To make Zed feel like GoLand, add these to your `~/.config/zed/keymap.json`:

```json
[
  {
    "context": "Workspace",
    "bindings": {
      "shift-f9": "task: spawn", // Similar to "Debug"
      "shift-f10": "task: spawn" // Similar to "Run"
    }
  },
  {
    "context": "Editor && mode == full",
    "bindings": {
      "f8": "debug: step over",
      "f7": "debug: step into",
      "shift-f8": "debug: step out",
      "alt-f9": "debug: run to cursor",
      "f9": "debug: resume"
    }
  }
]
```

## Installation (Development)

1. Clone this repository.
2. Open Zed.
3. Open the Extensions view (`cmd-shift-x` or `ctrl-shift-x`).
4. Click "Install Dev Extension".
5. Select the folder containing this repository.

## Troubleshooting

- **dlv not found**: Make sure `dlv` is installed and in your PATH. The extension tries to find it in `$GOPATH/bin` (`~/go/bin`) as well.
- **Check Logs**: Run `zed: open log` to see if the extension failed to locate the binary.
