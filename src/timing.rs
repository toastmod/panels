use std::time::{Duration, Instant};

/// Defines the timing for when a function should be called next.
pub enum Timing {
    ASAP,
    Framerate{last_rendered_at: Instant, desired_framerate: f64},
    SpecificTime{last_rendered_at: Instant, desired_wait_time: Duration },
    Never,
}

pub enum CallStatus {
    /// Ready to call function when the timing constraints are met.
    Awaiting(Timing),

    /// This function will not be called.
    Inactive,

    /// This function was just called.
    /// The applet can react accordingly, and optionally choose `Awaiting` to queue for calling.
    JustCalled(Instant),

}
