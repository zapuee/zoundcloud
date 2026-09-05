use crate::common::accessor::{Accessor, Setter, Getter};

mod app;
mod input;
mod common;

fn main() {
    let mut my_accessor = Accessor::new(1 as u8);
    my_accessor.set(255);
    println!("Hello, world!");
    println!("{}", my_accessor.get());
    // let myApp = app::init_app();
}
