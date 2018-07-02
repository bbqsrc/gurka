use gurka;
use gurka::GurkaMutator;

use ::MyWorld;

steps! {
    world: MyWorld;

    when "John creates an account" |world, _| {
        let db = world.pool().get().unwrap();
        let new_user = gurka::models::NewUser::create(&*db, "john", "abc123".to_owned()).unwrap();
        assert_eq!(new_user.username, "john");
    };

    then "an account named John is created" |world, _| {
        let db = world.pool().get().unwrap();
        let user = gurka::models::User::find_by_username(&*db, "john").unwrap().unwrap();
        assert_eq!(user.username, "john");
    };

    given "a logged in user named John" |world, step| {
        let db = world.pool().get().unwrap();
        let new_user = gurka::models::NewUser::create(&*db, "john", "abc123".to_owned()).unwrap();
        let model = world.mutator.log_in("john", "abc123").unwrap();
        world.token = Some(model.token());
        world.current_user = Some(model.user.model);
    };

    given "the feature below:" |world, step| {
        let feature_file_string = step.docstring().unwrap();
        unimplemented!();
    };

    given regex "^a person named (.*)$" |world, matches, step| {
        let name = &matches[1];
        assert_eq!("John", name);
        unimplemented!();
    };
}