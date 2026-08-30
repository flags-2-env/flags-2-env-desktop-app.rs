#![forbid(unsafe_code)]

use crate::state::{ConnectionEvent, DesktopState};

pub fn probe(endpoint: &str) -> DesktopState {
    let starting = DesktopState::initial(endpoint).transition(ConnectionEvent::ProbeStarted);
    starting
        .state
        .transition(ConnectionEvent::ProbeFailed)
        .state
}
