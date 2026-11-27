# LunarMaxim Windows connection-manager behavior

This fork starts the Windows connection manager hidden. An already authorized incoming session does not show or focus it. An incoming request that still needs the local user's approval opens it normally. The change does not authorize a session, change authentication, disable LAN discovery, or remove the service tray indicator and its session count. Non-Windows behavior is unchanged.

## Updating the fork

The published patch lives on the fork's `lunar/quiet-cm-master` branch. The local `lunar/quiet-connection-manager` branch applies the same patch to upstream tag `1.5.3`, matching the installed release. The fork's `master` stays unmodified. When upgrading, fetch upstream tags and cherry-pick the presentation commit onto the chosen stable tag, then rebuild the Windows client and run the regression test. Use a new branch for a new base instead of rewriting the published branch's history.

```sh
git fetch upstream --tags
git switch -c lunar/quiet-cm-next <upstream-stable-tag>
git cherry-pick <presentation-commit>
cd flutter
flutter test test/connection_manager_presentation_test.dart
```

Before installing an updated build, test both an already authenticated session and a connection that still requires local approval. The former must not raise or flash a window; the latter must visibly request approval and must not grant desktop control before approval. Check that the tray icon still reports an active session and can stop the service. Use the same test after each upstream update that touches the connection manager.

## Windows x64 builds

Keep the `master` branch unmodified. Carry `.github/workflows/lunarmaxim-windows-x64.yml` on the maintenance branch beside the presentation and [reverse handover](lunarmaxim-reverse-session-takeover.md) patches. Start a build by explicitly pushing a **new** tag matching `lunarmaxim-win-x64-*` for the tested maintenance commit:

```sh
git tag lunarmaxim-win-x64-1.5.3-01 <maintenance-commit>
git push origin refs/tags/lunarmaxim-win-x64-1.5.3-01
```

The workflow generates the Dart/Rust bridge from that same commit, builds Windows x64, then runs the focused presentation and reverse-handover tests. Download `rustdesk-lunarmaxim-windows-x64` from the successful Actions run. Check `source-commit.txt` and `SHA256SUMS` before using its MSI or unpacked application payload. The artifact is an unsigned fork build, not an upstream release. Updating installed PCs remains a separate, manual deployment with a rollback copy of the original app and verification of the existing firewall scope, authentication, and both directions of remote control.
