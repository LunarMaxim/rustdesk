use std::net::Ipv4Addr;

#[cfg(windows)]
pub(crate) const PEER_CAPABILITY: &str = "reverse_desktop_takeover";

pub(crate) fn pinned_tailscale_peer(configured: &str, whitelist: &str) -> Option<Ipv4Addr> {
    let peer: Ipv4Addr = configured.trim().parse().ok()?;
    let [first, second, _, _] = peer.octets();
    (first == 100
        && (64..=127).contains(&second)
        && whitelist.trim().parse::<Ipv4Addr>().ok() == Some(peer))
    .then_some(peer)
}

pub(crate) struct IncomingDesktop {
    pub peer: Ipv4Addr,
    pub token: String,
}

pub(crate) fn only_reverse_session<'a>(
    peer: Ipv4Addr,
    incoming: impl Iterator<Item = &'a IncomingDesktop>,
) -> Option<&'a IncomingDesktop> {
    let mut matching = incoming.filter(|incoming| incoming.peer == peer);
    let first = matching.next()?;
    matching.next().is_none().then_some(first)
}

/// Accepts a handover only for the exact already-authorized desktop snapped before dialing.
pub(crate) fn matches_snapshot(incoming: &IncomingDesktop, peer: Ipv4Addr, token: &str) -> bool {
    incoming.peer == peer && incoming.token == token
}

#[cfg(windows)]
pub(crate) fn policy_peer() -> Option<Ipv4Addr> {
    use base::config::keys;
    use hbb_common::config::Config;

    pinned_tailscale_peer(
        &Config::get_option(keys::OPTION_REVERSE_SESSION_TAKEOVER_PEER),
        &Config::get_option(keys::OPTION_WHITELIST),
    )
}

#[cfg(windows)]
pub(crate) async fn snapshot_before_dial(id: &str) -> Option<IncomingDesktop> {
    // Only a literal IP is eligible; connecting to an ID, relay, or DNS name
    // must not be mistaken for a connection to the opted-in Tailscale peer.
    let peer: Ipv4Addr = id.parse().ok()?;
    if policy_peer() != Some(peer) {
        return None;
    }
    let mut ipc = crate::ipc::connect(700, "").await.ok()?;
    ipc.send(&crate::ipc::Data::ReverseDesktopSnapshot(peer, None))
        .await
        .ok()?;
    match ipc.next_timeout(700).await.ok()? {
        Some(crate::ipc::Data::ReverseDesktopSnapshot(reply_peer, Some(token)))
            if reply_peer == peer && !token.is_empty() =>
        {
            Some(IncomingDesktop { peer, token })
        }
        _ => None,
    }
}

#[cfg(windows)]
pub(crate) async fn take_over(
    incoming: IncomingDesktop,
    state: &std::sync::Mutex<crate::ui_session_interface::ConnectionRoundState>,
    round: u32,
) {
    use std::{
        future::{poll_fn, Future},
        pin::pin,
        task::Poll,
    };

    let result = async {
        let mut ipc = crate::ipc::connect(700, "").await?;
        let request = crate::ipc::Data::ReplaceReverseDesktop(incoming.peer, incoming.token);
        let mut send = pin!(ipc.send(&request));
        // Reconnect shares this lock. Check every write poll rather than only
        // before awaiting the pipe, and never hold the lock across a yield.
        poll_fn(|cx| {
            let Ok(state) = state.lock() else {
                return Poll::Ready(Ok(()));
            };
            if !state.is_current_connected_round(round) {
                return Poll::Ready(Ok(()));
            }
            send.as_mut().poll(cx)
        })
        .await
    }
    .await;
    if let Err(error) = result {
        hbb_common::log::debug!("Reverse desktop was not replaced: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn takeover_is_off_unless_the_one_tailscale_peer_is_explicitly_whitelisted() {
        let peer = Ipv4Addr::new(100, 106, 149, 62);
        assert_eq!(
            pinned_tailscale_peer("100.106.149.62", "100.106.149.62"),
            Some(peer)
        );
        assert_eq!(pinned_tailscale_peer("", "100.106.149.62"), None);
        assert_eq!(
            pinned_tailscale_peer("100.106.149.62", "100.106.149.63"),
            None
        );
        assert_eq!(
            pinned_tailscale_peer("100.106.149.62", "100.106.149.62,100.106.149.63"),
            None
        );
        assert_eq!(pinned_tailscale_peer("192.168.10.1", "192.168.10.1"), None);
    }

    #[test]
    fn snapshots_one_existing_peer_not_other_peers_or_ambiguous_sessions() {
        let peer: Ipv4Addr = "100.106.149.62".parse().unwrap();
        let old = IncomingDesktop {
            peer,
            token: "old".into(),
        };
        let other = IncomingDesktop {
            peer: Ipv4Addr::new(100, 127, 220, 16),
            token: "other".into(),
        };
        assert_eq!(
            only_reverse_session(peer, [&other, &old].into_iter()).map(|s| s.token.as_str()),
            Some("old")
        );
        assert!(only_reverse_session(peer, [&other].into_iter()).is_none());
        assert!(only_reverse_session(peer, [&old, &old].into_iter()).is_none());
    }

    #[cfg(windows)]
    #[test]
    fn a_superseded_round_is_ineligible_even_if_it_had_connected() {
        let mut state = crate::ui_session_interface::ConnectionRoundState::default();
        let first = state.new_round();
        state.set_connected();
        assert!(state.is_current_connected_round(first));

        let next = state.new_round();
        assert!(!state.is_current_connected_round(first));
        assert!(!state.is_current_connected_round(next));
        state.set_connected();
        assert!(!state.is_current_connected_round(first));
        assert!(state.is_current_connected_round(next));
        state.set_disconnected(next);
        assert!(!state.is_current_connected_round(next));
    }

    #[test]
    fn a_late_candidate_does_not_close_a_reconnected_or_different_peer() {
        let peer: Ipv4Addr = "100.106.149.62".parse().unwrap();
        let old = IncomingDesktop {
            peer,
            token: "old".into(),
        };
        let replacement = IncomingDesktop {
            peer,
            token: "new".into(),
        };
        assert!(matches_snapshot(&old, peer, "old"));
        assert!(!matches_snapshot(&replacement, peer, "old"));
        assert!(!matches_snapshot(
            &old,
            Ipv4Addr::new(100, 127, 220, 16),
            "old"
        ));
    }
}
