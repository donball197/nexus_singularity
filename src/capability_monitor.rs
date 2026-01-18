use std::sync::atomic::{AtomicUsize, Ordering};

pub struct CapabilityMonitor {
    pub active_tokens: AtomicUsize,
    pub total_invocations: AtomicUsize,
}

impl CapabilityMonitor {
    pub const fn new() -> Self {
        Self {
            active_tokens: AtomicUsize::new(0),
            total_invocations: AtomicUsize::new(0),
        }
    }

    pub fn log_invocation(&self) {
        self.total_invocations.fetch_add(1, Ordering::SeqCst);
    }

    pub fn report_health(&self) -> String {
        format!(
            "🛡️ [Capability Monitor] Active Tokens: {} | Total Invocations: {}",
            self.active_tokens.load(Ordering::SeqCst),
            self.total_invocations.load(Ordering::SeqCst)
        )
    }
}
