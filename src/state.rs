#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectionPhase {
    Disconnected,
    Connecting,
    Connected,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectionEvent {
    ProbeStarted,
    ProbeSucceeded,
    ProbeFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DesktopState {
    phase: ConnectionPhase,
    endpoint: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transition {
    pub state: DesktopState,
    pub accepted: bool,
}

impl DesktopState {
    #[must_use]
    pub fn initial(endpoint: impl Into<String>) -> Self {
        Self {
            phase: ConnectionPhase::Disconnected,
            endpoint: endpoint.into(),
        }
    }

    #[must_use]
    pub const fn phase(&self) -> ConnectionPhase {
        self.phase
    }

    #[must_use]
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Pure and total over the closed phase/event vocabulary.
    #[must_use]
    pub fn transition(&self, event: ConnectionEvent) -> Transition {
        use ConnectionEvent::{ProbeFailed, ProbeStarted, ProbeSucceeded};
        use ConnectionPhase::{Connected, Connecting, Disconnected, Failed};

        let (phase, accepted) = match (self.phase, event) {
            (Disconnected, ProbeStarted) => (Connecting, true),
            (Disconnected, ProbeSucceeded | ProbeFailed) => (Failed, false),
            (Connecting, ProbeStarted) => (Connecting, false),
            (Connecting, ProbeSucceeded) => (Connected, true),
            (Connecting, ProbeFailed) => (Failed, true),
            (Connected, ProbeStarted) => (Connecting, true),
            (Connected, ProbeSucceeded) => (Connected, false),
            (Connected, ProbeFailed) => (Failed, true),
            (Failed, ProbeStarted | ProbeSucceeded | ProbeFailed) => (Failed, false),
        };

        Transition {
            state: Self {
                phase,
                endpoint: self.endpoint.clone(),
            },
            accepted,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PHASES: [ConnectionPhase; 4] = [
        ConnectionPhase::Disconnected,
        ConnectionPhase::Connecting,
        ConnectionPhase::Connected,
        ConnectionPhase::Failed,
    ];
    const EVENTS: [ConnectionEvent; 3] = [
        ConnectionEvent::ProbeStarted,
        ConnectionEvent::ProbeSucceeded,
        ConnectionEvent::ProbeFailed,
    ];

    fn at(phase: ConnectionPhase) -> DesktopState {
        DesktopState {
            phase,
            endpoint: "https://flags.example".into(),
        }
    }

    #[test]
    fn every_phase_event_pair_has_a_controlled_result() {
        let mut transitions = 0;
        for phase in PHASES {
            for event in EVENTS {
                let outcome = at(phase).transition(event);
                assert_eq!(outcome.state.endpoint(), "https://flags.example");
                assert!(PHASES.contains(&outcome.state.phase()));
                transitions += 1;
            }
        }
        assert_eq!(transitions, PHASES.len() * EVENTS.len());
    }

    #[test]
    fn connected_requires_a_successful_in_flight_probe() {
        for phase in PHASES {
            for event in EVENTS {
                let outcome = at(phase).transition(event);
                if outcome.state.phase() == ConnectionPhase::Connected && outcome.accepted {
                    assert_eq!(
                        (phase, event),
                        (ConnectionPhase::Connecting, ConnectionEvent::ProbeSucceeded)
                    );
                }
            }
        }
    }

    #[test]
    fn failure_is_absorbing() {
        for event in EVENTS {
            let outcome = at(ConnectionPhase::Failed).transition(event);
            assert_eq!(outcome.state.phase(), ConnectionPhase::Failed);
            assert!(!outcome.accepted);
        }
    }
}
