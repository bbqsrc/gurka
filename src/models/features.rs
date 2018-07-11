use diesel::prelude::*;
use diesel::result::QueryResult;
use diesel::insert_into;

use ::PgConnection;
use schema;
use schema::features;
use super::Project;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(Project)]
pub struct Feature {
    pub id: i32,
    pub project_id: i32,
    pub creator_id: i32,
    pub slug: String,
    pub name: String
}

impl Feature {
    pub fn new(db: &PgConnection, new_feature: NewFeature) -> QueryResult<Feature> {
        insert_into(schema::features::table)
            .values(new_feature)
            .get_result(db)
    }

    pub fn find_one(db: &PgConnection, project_slug: &str, feature_slug: &str) -> QueryResult<Option<Feature>> {
        use schema::features::dsl as features;
        
        let maybe_project = Project::find_by_slug(db, project_slug)?;
        let project = match maybe_project {
            Some(v) => v,
            None => return Ok(None)
        };

        Feature::belonging_to(&project)
            .filter(features::slug.eq(feature_slug))
            .get_result(db)
            .optional()
    }

    pub fn find_by_id(db: &PgConnection, id: i32) -> QueryResult<Option<Feature>> {
        use schema::features::dsl as features;

        schema::features::table
            .filter(features::id.eq(id))
            .get_result(db)
            .optional()
    }

    pub fn all_by_project(db: &PgConnection, project: &Project) -> QueryResult<Vec<Feature>> {
        Feature::belonging_to(project)
            .get_results(db)
    }
}

#[derive(Insertable)]
#[table_name="features"]
pub struct NewFeature {
    pub project_id: i32,
    pub creator_id: i32,
    pub slug: String,
    pub name: String
}