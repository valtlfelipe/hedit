# Hedit

_(pronounced **H-edit**)_

Fast and intuitive hosts file management. The modern hosts file editor for people who care about their tools.

![image](https://github.com/user-attachments/assets/cd843e7b-6563-495a-82fa-a62b18c6dd78)

<div align="center">
  <a href="https://github.com/valtlfelipe/hedit/releases">
    <img src="https://img.shields.io/badge/Download-Hedit-blue?style=for-the-badge" alt="Download Hedit" />
  </a>
</div>

## ✨ Features

This application is available on macOS and Linux. Windows support is planned for future releases.

*   **Intuitive UI:** Clean and user-friendly interface for easy hosts file management with Light & Dark mode.
*   **Fast & Secure:** Built with modern tech for performance and system-level access and Open-source codebase.
*   **Multiple Files:** Manage multiple files and activate the one you need.
*   **Remote Files:** Add and Sync remote hosts files locally (like StevenBlack/hosts, etc.) from a URL.
*   **Combo Files:** Create combo files by including dynamically other local files or remote URL's.
*   **Auto Sync:** Enable automatic remote file synchronization so remote files are always up to date.
*   **Run in background:** Allow Hedit to run in the background (system tray) so Auto Sync can operate continuously.
*   **Syntax highlighting:** Edit your hosts file with syntax highlighting and validation.

## 📖 Syntax

Hedit supports standard hosts file entries and two directives for combining local and remote files.

### Hosts entries

Each entry must contain a valid IPv4 or IPv6 address followed by one hostname. You can also use blank lines, full-line comments, and inline comments.

```hosts
# Local development
127.0.0.1 app.local
127.0.0.1 api.app.local # Optional inline comment
::1 ipv6.app.local
```

Hedit validates entries before activation and rejects invalid addresses, invalid hostnames, multiple hostnames on the same line, and duplicate custom hostnames.

### Combo files

Combo files let you build one hosts file from other files managed by Hedit and from remote sources. Add each directive on its own line; Hedit expands them in order when the combo file is activated.

#### Include another Hedit file

```hosts
@local(550e8400-e29b-41d4-a716-446655440000)
```

Right-click the file in the sidebar, select **Copy ID**, and use that ID inside `@local(...)`.

#### Include a remote hosts file

```hosts
@remote(https://example.com/hosts.txt)
```

Remote URLs must use HTTPS and return a `text/plain` response. Included files should contain standard hosts entries; directives are not expanded recursively.

#### Complete example

```hosts
# Project-specific entries
127.0.0.1 app.local
127.0.0.1 api.app.local

# Include another file managed by Hedit
@local(550e8400-e29b-41d4-a716-446655440000)

# Include a remote blocklist
@remote(https://example.com/hosts.txt)
```

<hr>

These people help make it happen. You can <a href="https://github.com/sponsors/valtlfelipe">become a sponsor</a> to support Hedit. Thanks for your support! 🫶🏻
<p>
  <!-- sponsors --><!-- sponsors -->
</p>

## ⚠️ Known issue

### Permission denied when saving on macOS

On some macOS systems, activating a file may fail with a permission error when Hedit tries to write to `/etc/hosts`. Follow the investigation in [issue #30](https://github.com/valtlfelipe/hedit/issues/30).

You can change the file's permissions to let your user edit it without sudo. I would recommend to do quick search before if you want to go that route, just do it knowing the tradeoffs.
