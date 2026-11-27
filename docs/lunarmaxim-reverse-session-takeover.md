# Windows reverse desktop handover

This opt-in setting is for a reciprocal pair with a single Tailscale IPv4 address on each side and a RustDesk firewall rule restricted to the other peer on the Tailscale interface. Existing password or click approval, IP and ID whitelists, and other RustDesk permissions still apply. It is not a setting for Internet-facing RustDesk or shared networks.

On each **updated** Windows installation, set `reverse-session-takeover-peer` to the other computer’s Tailscale IPv4 address. The service requires the existing RustDesk IP whitelist to be exactly that same address. Change neither the firewall nor the authentication mode to use this feature. For these two computers, configure the following from an elevated PowerShell:

On S-Game:

```powershell
& "$env:ProgramFiles\RustDesk\rustdesk.exe" --option reverse-session-takeover-peer 100.106.149.62
```

On LMTerminal:

```powershell
& "$env:ProgramFiles\RustDesk\rustdesk.exe" --option reverse-session-takeover-peer 100.127.220.16
```

Leave the option empty to disable handover, which is the upstream-compatible default. Reconnect any desktop connection that was authorized before enabling the feature so it can be tracked.

When a user starts a Windows desktop session by the configured literal IP (not a RustDesk ID or DNS name), the controller takes a local snapshot of the single already-authorized incoming desktop session from that peer before dialing. After login, the host declares handover support only for a direct desktop session from its explicitly configured single IP whitelist entry. The controller requires that declaration, keyboard control and a decoded desktop frame in its current connection round before it can close the exact snapshotted reverse session. A failed or pending login, an original-version peer, a stale snapshot, a superseded connection round, view-only mode, and file, camera or terminal sessions do not trigger a handover. Simultaneous attempts without an existing reverse session do not displace each other. A snapshotted session closes without automatically reconnecting or locking the screen. The declaration is a capability check, not a replacement for the Tailscale transport and firewall restrictions.

## Windows runtime acceptance

1. Start S-Game → LMTerminal. In the opposite direction, try one failed or unanswered approval: the original desktop must remain open.
2. Successfully start LMTerminal → S-Game. After decoding the new desktop, only the original S-Game → LMTerminal connection must end. Try the same test in the opposite direction.
3. Repeat with the other host's handover setting disabled or with a stock client. The original desktop must remain open. Repeat after replacing an existing connection round: delayed frames from the previous round must have no effect.
4. Keep a file-transfer session or a different peer connected during the test; it must not be closed. Verify existing password/click approval and tray status remain intact.
