use std::time::{Duration, Instant};
use winit::event_loop::EventLoopProxy;
use crate::proxyevents::ProxyEvent;

/// Defines the timing for when a function should be called next.
pub enum Timing {

    /// Call immediately at the surface framerate.
    /// * If you wish to overuse the CPU, SpecificTime may be better.
    ASAP,

    /// Called at a certain framerate, in sync with the Surface framerate.
    Framerate{last_rendered_at: Instant, desired_framerate: f64},

    /// Uses a separate timer thread to trigger this call.
    /// * As opposed to `Framerate`, the EventLoop will recieve a proxy event so that the call will be made immediately at the desired time.
    SpecificTime{last_rendered_at: Instant, desired_wait_time: Duration },

    /// This will not call the function.
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



pub struct TimingMgmt {
    proxy: EventLoopProxy<ProxyEvent>,
    timers: Vec<CallStatus>
}

impl TimingMgmt {
    pub fn start_manager(proxy: EventLoopProxy<ProxyEvent>) {
        std::thread::spawn(move ||{
            let mut t_mgmt = TimingMgmt {
                proxy,
                timers: vec![]
            };

            loop {
                // t_mgmt.proxy.send_event()
            }

        });
    }
}

