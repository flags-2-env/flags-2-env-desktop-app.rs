#![forbid(unsafe_code)]

use crate::state::{ConnectionPhase, DesktopState};

/// Native UI surface. Not a webview and not React.
pub fn render(state: &DesktopState) -> String {
    let connection = match state.phase() {
        ConnectionPhase::Disconnected => "disconnected",
        ConnectionPhase::Connecting => "connecting",
        ConnectionPhase::Connected => "connected",
        ConnectionPhase::Failed => "failed",
    };
    format!(
        "flags-2-env desktop\nendpoint={}\nconnection={}\n",
        state.endpoint(),
        connection
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ConnectionEvent;

    #[test]
    fn render_exhaustively_names_each_connection_phase() {
        let disconnected = DesktopState::initial("https://flags.example");
        let connecting = disconnected.transition(ConnectionEvent::ProbeStarted).state;
        let connected = connecting.transition(ConnectionEvent::ProbeSucceeded).state;
        let failed = connecting.transition(ConnectionEvent::ProbeFailed).state;

        assert!(render(&disconnected).contains("connection=disconnected"));
        assert!(render(&connecting).contains("connection=connecting"));
        assert!(render(&connected).contains("connection=connected"));
        assert!(render(&failed).contains("connection=failed"));
    }
}
