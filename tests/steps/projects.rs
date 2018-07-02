use gurka;
use gurka::GurkaMutator;

use ::MyWorld;

steps! {
    world: MyWorld;

    when regex "^a project with slug \"(.*)\" is submitted$" |world, matches, _| {
        let user = world.current_user().unwrap();
        let slug = (&matches[1]).to_string();
        let name = "Any Name".to_string();
        let project = world.mutator.create_project(slug, name, user).unwrap();
    };

    then regex "^a project with slug \"(.*)\" is created$" |world, matches, _| {
        let slug = &matches[1];
        let project = world.query.project_by_slug(slug).unwrap();
        assert!(project.is_some());
    };

    then regex "^a project with slug \"(.*)\" is owned by (.*)$" |world, matches, _| {
        let slug = &matches[1];
        let username = &matches[2];
        let user = world.query.user_by_username(username).unwrap().unwrap();
        let project = world.query.project_by_slug(slug).unwrap().unwrap();
        assert_eq!(project.model.owner_id, user.model.id);
    };
}