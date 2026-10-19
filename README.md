# EliteStocks TV Desktop

A Windows IPTV player using the locked architecture:

- **React + TypeScript** for the interface.
- **Tauri + WebView2** for the native desktop shell.
- **MPV** for playback, embedded into a dedicated native Win32 child window positioned over the player frame.

## Current milestone

The player surface is embedded inside the application window. The frontend reports the player-frame bounds to the Tauri backend, which positions a native Windows child surface; MPV attaches to that surface with its --wid option.

MPV must currently be installed on Windows and available as mpv.exe on PATH. The MPV executable and associated DLLs are not bundled in the installer yet; packaging and licensing checks remain a separate distribution task.

Playlist import, channel/EPG models, stream browsing, advanced playback controls, and playback-engine packaging are planned follow-up milestones.

## Run locally

Requirements: Node.js 22+, Rust stable with the MSVC Windows build tools, WebView2 Runtime, and MPV.

```powershell
npm install
npm run icons
npm run tauri dev
```

Create a Windows installer locally with:

```powershell
npm run icons
npm run build
npm run tauri build
```

## Manual GitHub Actions builds

The Windows installer workflow is triggered only by workflow_dispatch. It has no push, pull-request, or release triggers. Once this workflow is merged into the repository's default branch, open Actions → Manual Windows Build → Run workflow to build the NSIS installer. The installer is uploaded as a workflow artifact.

## Development principles

- Keep the UI responsive during playlist parsing and media operations.
- Virtualize large channel lists.
- Keep channel browsing separate from the active playback process.
- Validate playback with streams you are authorized to access.
- Test stream compatibility and GPU decoding on real Windows machines.
