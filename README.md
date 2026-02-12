# Haskell Debug Adapter for Zed

A Zed extension that enables debugging Haskell applications using [haskell-debug-adapter](https://github.com/phoityne/haskell-debug-adapter).

## Prerequisites

Install `haskell-debug-adapter` and ensure it's in your `$PATH`:

```bash
cabal install haskell-dap ghci-dap haskell-debug-adapter
```

## Installation

1. **Open Zed editor**

2. **Install the dev extension**:
   - Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
   - Type and select: `zed: install dev extension`
   - Choose this extension folder

3. **Configure debug settings**:
   - Create `.zed/debug.json` in your project root
   - Copy the example configuration from `example/debug.json`

## Configuration Example

Create `.zed/debug.json` with:

```json
[
  {
    "adapter": "haskell-debug-adapter",
    "label": "Haskell debugger",
    "name": "Haskell debugger",
    "type": "ghc",
    "request": "launch",
    "cwd": "$ZED_WORKTREE_ROOT",
    "startup": "$ZED_WORKTREE_ROOT/app/Main.hs",
    "workspace": "$ZED_WORKTREE_ROOT",
    "logFile": "$ZED_WORKTREE_ROOT/.zed/haskell-debug.log",
    "ghciCmd": "cabal repl <your-binary-name> -w ghci-dap --repl-no-load",
    "logLevel": "INFO",
    "startupFunc": "main",
    "startupArgs": "",
    "stopOnEntry": true,
    "mainArgs": "",
    "ghciPrompt": "H>>= ",
    "ghciInitialPrompt": "> ",
    "ghciEnv": {},
    "forceInspect": false
  }
]
```

**Important**: Replace `<your-binary-name>` in `ghciCmd` with your actual executable name from your `.cabal` file.

## Usage

1. Open your Haskell project in Zed
2. Set breakpoints in your code
3. Press `F5` or run `debug: start` from command palette
4. Select "Haskell debugger"

**Voilà, let's debug!**

## Troubleshooting

If debugging doesn't start:
- Verify `haskell-debug-adapter` is in your PATH: `which haskell-debug-adapter`
- Check the log file at `.zed/haskell-debug.log`
- View Zed logs: `Cmd+Shift+P` → `zed: open log`
