use crate::conductor::PanelsApp;

mod panel;
mod conductor;
fn main() {
    println!("Hello, world!");
    let mut conductor = Box::new(PanelsApp{
        window_focused: false,
        focused_panel: 0
    });
    panels::start(conductor);
}
