
fn main() {
    println!("Hello, world!");
    let mut conducter = Box::new(panels::appmgmt::PanelsApp{
        focused_panel: 0
    });
    panels::start(conducter);
}
