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
