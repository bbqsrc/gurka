use gurka;
use gurka::graphql::GurkaMutator;

use ::MyWorld;

steps!(MyWorld => {
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

    given "an existing user named John" |world, _| {
        let db = world.pool().get().unwrap();
        let _ = gurka::models::NewUser::create(&*db, "john", "abc123".to_owned()).unwrap(); 
    };

    when "John uses an invalid password" |world, _| {
        match world.mutator.log_in("john", "wrong password") {
            Ok(_) => panic!("Got Ok, expected Err!"),
            Err(e) => {
                world.last_field_error = Some(e);
            }
        }
    };

    then "a login failure occurs" |world, _| {
        match &world.last_field_error {
            None => panic!("Expected a field error"),
            Some(err) => {
                assert_eq!(err.message(), "No user found or invalid password");
            }
        }
    };

    given "a logged in user named John" |world, _| {
        let db = world.pool().get().unwrap();
        let _new_user = gurka::models::NewUser::create(&*db, "john", "abc123".to_owned()).unwrap();
        let model = world.mutator.log_in("john", "abc123").unwrap();
        world.token = Some(model.token());
        world.current_user = Some(model.user.model);
    };

    given "the feature below:" |_world, step| {
        let _feature_file_string = step.docstring().unwrap();
        unimplemented!();
    };

    given regex "^a person named (.*)$" (String) |_world, name, _| {
        assert_eq!("John", name);
        unimplemented!();
    };

    when "a request to list all projects for the current user is received" |world, _| {
        let db = world.pool().get().unwrap();
        let projects = {
            let user = world.current_user().unwrap();
            gurka::models::Project::all_for_user(&*db, &user).unwrap()
        };
        world.current_user_projects = Some(projects);
    };

    then "a list of all projects owned by the current user are shown" |world, _|  {
        assert!(world.current_user_projects.is_some());
    };
});