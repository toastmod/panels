// pub fn world_to_pixel(screen_size_px: (u32,u32),  )

use std::time::Duration;

pub fn fps_to_dur(fps: f64) -> Duration {
    let spf = 1.0f64/fps;
    Duration::from_secs_f64(spf)
}