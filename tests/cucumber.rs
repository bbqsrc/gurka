#[macro_use] extern crate cucumber_rust;
extern crate gurka;

struct MyWorld {}

impl cucumber_rust::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        MyWorld {}
    }
}

mod steps;
use steps::users;

cucumber! {
    features: "./features";
    world: MyWorld;
    steps: &[
        users::steps
    ]
}
