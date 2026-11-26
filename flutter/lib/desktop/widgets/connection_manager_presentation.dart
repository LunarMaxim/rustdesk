/// Determines whether an incoming connection should automatically open the
/// connection manager. It does not grant or change connection authorization.
bool shouldPresentConnectionManager({
  required bool isWindows,
  required bool hiddenByPolicy,
  required Iterable<({bool authorized, bool disconnected})> clients,
}) {
  if (hiddenByPolicy) return false;
  if (!isWindows) return clients.isNotEmpty;
  return clients.any((client) => !client.authorized && !client.disconnected);
}
