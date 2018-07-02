#![feature(fnbox)]
#[macro_use] extern crate cucumber_rust;
extern crate gurka;
extern crate rocket;
extern crate diesel;

fn reset() {
    std::process::Command::new("diesel")
            .args(&["migration", "redo"])
            .output()
            .expect("success");
}

pub struct MyWorld {
    pool: gurka::PgPool,
    client: rocket::local::Client,
    mutator: gurka::DatabaseMutator,
    token: Option<String>
}

impl MyWorld {
    pub fn client(&self) -> &rocket::local::Client {
        &self.client
    }

    pub fn pool(&self) -> &gurka::PgPool {
        &self.pool
    }

    pub fn token(&self) -> Option<&str> {
        match &self.token {
            Some(v) => Some(&v),
            None => None
        }
    }
}

impl cucumber_rust::World for MyWorld {
}

impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        reset();
        let server = gurka::make_server();

        MyWorld {
            client: rocket::local::Client::new(server)
                .expect("rocket to the moon"),
            pool: gurka::establish_pool(),
            mutator: gurka::DatabaseMutator::new(gurka::establish_pool()),
            token: None
        }
    }
}

mod steps;
use steps::users;

cucumber! {
    features: "./features";
    world: MyWorld;
    steps: &[
        users::steps
    ];
    before: || {
        reset();
    }
}
