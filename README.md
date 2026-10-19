# EliteStocks TV Desktop

A Windows IPTV player foundation using the locked stack:

- **React + TypeScript** for the interface.
- **Tauri + WebView2** for the Windows desktop shell.
- **MPV** for media playback.

## Current milestone

The first milestone establishes the UI and native desktop shell, with a basic MPV process adapter. Playback currently opens MPV in its own window; embedded video rendering and reliable MPV packaging are follow-up engineering tasks, not yet complete.

The MPV executable must currently be installed on Windows and available as `mpv.exe` on `PATH`.

## Run locally

Requirements: Node.js 22+, Rust stable with the MSVC Windows build tools, WebView2 Runtime, and MPV.

```powershell
npm install
npm run tauri dev
```

Create a Windows installer locally with:

```powershell
npm run tauri build
```

## Manual GitHub Actions builds

The Windows installer workflow is triggered **only by `workflow_dispatch`**. It does not run on pushes, pull requests, or releases. Open **Actions → Manual Windows Build → Run workflow** to launch a build. The installer is uploaded as a workflow artifact.

## Implementation principles

- Keep the UI responsive during playlist parsing and media operations.
- Virtualize large channel lists when playlist browsing is introduced.
- Keep channel browsing separate from the active playback process.
- Validate playback with streams you are authorized to access.
- Test stream compatibility and GPU decoding on real Windows machines.
- Package MPV and its licensing notices deliberately before distributing a release.
