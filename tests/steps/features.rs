use ::MyWorld;
use gurka::GurkaMutator;

steps! {
    world: MyWorld;

    when regex r#"^a feature with slug "(.*)" is submitted$"# |world, matches, _| {
        let name = &matches[1];
        world.mutator.create_feature(
            name.to_string(),
            name.to_string(),
            world.current_project().unwrap()).unwrap();
    };

    then regex r#"^a feature with slug "(.*)" is created in the project with slug "(.*)"$"# |world, matches, _| {
        let feature_slug = &matches[1];
        let project_slug = &matches[2];
        let feature = world.query.feature(project_slug, feature_slug).unwrap().unwrap();
        assert_eq!(&feature.model.slug, feature_slug);
    };
}