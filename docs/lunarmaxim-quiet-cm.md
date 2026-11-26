# LunarMaxim Windows connection-manager behavior

This fork starts the Windows connection manager hidden. An already authorized incoming session does not show or focus it. An incoming request that still needs the local user's approval opens it normally. The change does not authorize a session, change authentication, disable LAN discovery, or remove the service tray indicator and its session count. Non-Windows behavior is unchanged.

## Updating the fork

The Windows presentation changes live on `lunar/quiet-connection-manager`, initially based on upstream tag `1.5.3`. Leave the fork's `master` free of local customization. When upgrading, fetch upstream tags and replay the small presentation commit(s) onto the chosen stable tag in a clean checkout, then rebuild the Windows client and run the regression test. To avoid overwriting the published branch's history, prefer a new local branch for a new base; a merge is not needed just to update the patch's base.

```sh
git fetch upstream --tags
git switch -c lunar/quiet-cm-next <upstream-stable-tag>
git cherry-pick <presentation-commit>
cd flutter
flutter test test/connection_manager_presentation_test.dart
```

Before installing an updated build, test both an already authenticated session and a connection that still requires local approval. The former must not raise or flash a window; the latter must visibly request approval and must not grant desktop control before approval. Check that the tray icon still reports an active session and can stop the service. Use the same test after each upstream update that touches the connection manager.
