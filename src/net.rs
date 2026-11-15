#![forbid(unsafe_code)]

use crate::state::{ConnectionEvent, ConnectionPhase, DesktopState};

/// Result of a single, externally supplied native transport probe.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProbeOutcome {
    Reachable,
    Unavailable,
}

/// Run one injected probe through the closed desktop lifecycle state machine.
///
/// This adapter does not perform HTTP or assert remote authentication.
/// The caller must inject a real, bounded native transport when available.
pub fn probe_with<F>(endpoint: &str, check: F) -> DesktopState
where
    F: FnOnce(&str) -> ProbeOutcome,
{
    let starting = DesktopState::initial(endpoint).transition(ConnectionEvent::ProbeStarted);
    debug_assert_eq!(starting.state.phase(), ConnectionPhase::Connecting);
    let result = match check(endpoint) {
        ProbeOutcome::Reachable => ConnectionEvent::ProbeSucceeded,
        ProbeOutcome::Unavailable => ConnectionEvent::ProbeFailed,
    };
    starting.state.transition(result).state
}

/// Fail closed until a real native transport is bound at the application edge.
pub fn probe(endpoint: &str) -> DesktopState {
    probe_with(endpoint, |_| ProbeOutcome::Unavailable)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn native_probe_is_injected_once_and_preserves_exact_endpoint() {
        let calls = Cell::new(0);
        let endpoint = "https://flags.example/v1/health";
        let state = probe_with(endpoint, |actual| {
            calls.set(calls.get() + 1);
            assert_eq!(actual, endpoint);
            ProbeOutcome::Reachable
        });
        assert_eq!(calls.get(), 1);
        assert_eq!(state.endpoint(), endpoint);
        assert_eq!(state.phase(), ConnectionPhase::Connected);
    }

    #[test]
    fn rejected_probe_does_not_advance_to_connected() {
        let state = probe_with("https://flags.example", |_| ProbeOutcome::Unavailable);
        assert_eq!(state.phase(), ConnectionPhase::Failed);
    }

    #[test]
    fn default_probe_remains_fail_closed_without_transport() {
        let state = probe("https://flags.example");
        assert_eq!(state.phase(), ConnectionPhase::Failed);
    }
}
