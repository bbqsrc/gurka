use gurka::graphql::GurkaMutator;
use gurka::models::{NewStep, Step};

use ::MyWorld;

steps! {
    world: MyWorld;

    given regex r#"a feature in "(.*)" with slug "(.*)" created by (.*)"# |world, matches, _| {
        let feature_slug = &matches[2];

        let feature = {
            let project = world.current_project().unwrap();
            let user = world.current_user().unwrap();
            world.mutator.create_feature(feature_slug.to_string(), "My Feature".to_string(), &project, &user).unwrap()
        };

        world.current_feature = Some(feature.model);
    };

    given "I have a series of steps:" |world, step| {
        let table = step.table().expect("table to exist");

        let feature = world.current_feature().expect("current feature");
        let user = world.current_user().expect("current user");

        let new_steps = table.rows.iter().map(|r| {
            NewStep {
                feature: feature,
                creator: user,
                step_type: r[0].to_string(),
                value: r[1].to_string(),
                position: Some(r[3].parse::<i32>().unwrap())
            }
        }).collect::<Vec<NewStep>>();

        for (n, new_step) in new_steps.into_iter().enumerate() {
            world.mutator.create_step(new_step).expect(&format!("{}: expected new step to succeed", n));
        }
    };

    when "each step is moved according to the following:" |world, step| {
        let table = step.table().expect("table to exist");
        let steps = {
            let feature = world.current_feature().expect("current feature");
            let pairs = table.rows.iter()
                .map(|r| (r[0].parse::<usize>().unwrap() - 1, r[1].parse::<usize>().unwrap() - 1))
                .collect::<Vec<(usize, usize)>>();

            assert_eq!(pairs.len(), 5);

            let mut last_steps = vec![];

            for (from, to) in pairs {
                let steps = world.query.feature_steps(feature.id).unwrap();
                world.mutator.reorder_step_before(steps[from].model.clone(), &steps[to].model).unwrap();

                let mut steps = world.query.feature_steps(feature.id).unwrap()
                    .into_iter().map(|x| x.model)
                    .collect::<Vec<Step>>();

                // steps.sort_unstable_by(|a, b| a.id.cmp(&b.id));

                last_steps.push(steps);
            }

            last_steps
        };
        world.last_steps = steps;
    };

    then "the expected order of each step after each move is:" |world, step| {
        let table = step.table().expect("table to exist");
        let last_steps = &world.last_steps;

        eprintln!("{:?}", last_steps.iter().map(|steps| steps.iter().map(|x| x.position).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>());

        for (n, row) in table.rows.iter().enumerate() {
            let mut steps = last_steps[n].clone();
            steps.sort_unstable_by(|a, b| a.position.cmp(&b.position));
            
            let positions = row.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

            eprintln!("-----");
            eprintln!("Iteration: {}", n + 1);
            eprintln!("Current: {:?}", steps.iter().map(|x| x.id).collect::<Vec<i32>>());
            eprintln!("Expected: {:?}", &positions);
            
            for (m, expected) in positions.into_iter().enumerate() {
                let step = &steps[m];
                assert_eq!(step.id, expected);
            }
        }
    };

    when regex r#"^a feature with slug "(.*)" is submitted$"# |world, matches, _| {
        let user = world.current_user().unwrap();

        let name = &matches[1];
        world.mutator.create_feature(
            name.to_string(),
            name.to_string(),
            world.current_project().unwrap(),
            user).unwrap();
    };

    then regex r#"^a feature with slug "(.*)" is created in the project with slug "(.*)"$"# |world, matches, _| {
        let feature_slug = &matches[1];
        let project_slug = &matches[2];
        let feature = world.query.feature(project_slug, feature_slug).unwrap().unwrap();
        assert_eq!(&feature.model.slug, feature_slug);
    };
}