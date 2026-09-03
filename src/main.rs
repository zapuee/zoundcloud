mod ui;
mod input;

fn main() {
    println!("Hello, world!");
    let mut app = ui::init_app();
    input::start(&mut app);
}
