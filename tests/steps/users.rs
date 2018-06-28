extern crate rocket;

use gurka;

steps! {
    given "the feature below:" |world, step| {
        let feature_file_string = step.docstring().unwrap();
        let server = gurka::make_server();
        let client = rocket::local::Client::new(server).expect("rocket to the moon");
        let mut response = client.get("/graphql?query={\"test\":true}").dispatch();
        unimplemented!()
    };

    given regex "^a person named (.*)$" |world, matches, step| {
        let name = &matches[1];
        assert_eq!("John", name);
        unimplemented!()
    };
}