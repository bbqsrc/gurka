use diesel::result::QueryResult;
use diesel::prelude::*;
use diesel::insert_into;
use ::PgConnection;
use ::schema;
use ::schema::projects;

#[derive(Queryable, Debug)]
pub struct Project {
    pub id: i32,
    pub slug: String,
    pub name: String,
    pub owner_id: i32
}

impl Project {
    pub fn new(db: &PgConnection, new_project: NewProject) -> QueryResult<Project> {
        insert_into(schema::projects::table)
            .values(new_project)
            .get_result(db)
    }

    pub fn find_by_slug(db: &PgConnection, slug: &str) -> QueryResult<Option<Project>> {
        use schema::projects::dsl as projects;

        schema::projects::table
            .filter(projects::slug.eq(slug))
            .get_result(db)
            .optional()
    }
}

#[derive(Insertable)]
#[table_name="projects"]
pub struct NewProject {
    pub slug: String,
    pub name: String,
    pub owner_id: i32
}