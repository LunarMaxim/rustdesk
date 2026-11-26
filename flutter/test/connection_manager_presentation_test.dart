import 'package:flutter_hbb/desktop/widgets/connection_manager_presentation.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('authorized Windows sessions do not open the connection manager', () {
    expect(
      shouldPresentConnectionManager(
        isWindows: true,
        hiddenByPolicy: false,
        clients: const [
          (authorized: true, disconnected: false),
          (authorized: false, disconnected: true),
        ],
      ),
      isFalse,
    );
  });

  test('Windows still opens the manager when a peer needs approval', () {
    expect(
      shouldPresentConnectionManager(
        isWindows: true,
        hiddenByPolicy: false,
        clients: const [
          (authorized: true, disconnected: false),
          (authorized: false, disconnected: false),
        ],
      ),
      isTrue,
    );
  });

  test('other desktops retain the authorized-session popup', () {
    expect(
      shouldPresentConnectionManager(
        isWindows: false,
        hiddenByPolicy: false,
        clients: const [(authorized: true, disconnected: false)],
      ),
      isTrue,
    );
  });
}
