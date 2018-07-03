use gurka::graphql::GurkaMutator;

use ::MyWorld;

steps! {
    world: MyWorld;

    when regex "^a project with slug \"(.*)\" is submitted$" |world, matches, _| {
        let user = world.current_user().unwrap();
        let slug = (&matches[1]).to_string();
        let name = "Any Name".to_string();
        let _project = world.mutator.create_project(slug, name, user).unwrap();
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

    given regex "^a project with slug \"(.*)\" owned by (.*)$" |world, matches, _| {
        let user = world.query.user_by_username(&matches[2]).unwrap().unwrap();
        let project = world.mutator
            .create_project(matches[1].clone(), "Any Name".to_string(), &user.model).unwrap();
        world.current_project = Some(project.model);
    };

    when regex "^a project with slug \"(.*)\" is selected$" |world, matches, _| {
        let slug = &matches[1];
        let project = world.query.project_by_slug(slug).unwrap().unwrap();
        world.current_project = Some(project.model);
    };

    when regex "^a project with slug \"(.*)\" is selected for deletion$" |world, matches, _| {
        let slug = &matches[1];
        let project = world.query.project_by_slug(slug).unwrap().unwrap();
        let _ = world.mutator.delete_project(project.model).unwrap();
    };

    then regex "^a project with slug \"(.*)\" no longer exists$" |world, matches, _| {
        let slug = &matches[1];
        assert!(world.query.project_by_slug(slug).unwrap().is_none());
    };

    then regex "^a project with slug \"(.*)\" is shown$" |world, matches, _| {
        let slug = &matches[1];
        assert!(world.query.project_by_slug(slug).unwrap().is_some());
    };

    when regex "^a project with slug \"(.*)\" is renamed to \"(.*)\"$" |world, matches, _| {
        let old_slug = &matches[1];
        let new_slug = &matches[2];

        let old_project = world.query.project_by_slug(old_slug).unwrap().unwrap();
        let _new_project = world.mutator.rename_project_slug(old_project.model, new_slug).unwrap();
    };
}