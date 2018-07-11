use diesel::prelude::*;
use diesel::result::QueryResult;
use diesel::{insert_into, update};
use diesel::result::Error as DieselError;
use std::error;
use std::fmt;
use heck::KebabCase;

use ::PgConnection;
use schema;
use schema::steps;
use super::{Feature, User};

#[derive(Identifiable, Queryable, Associations, Debug, Clone)]
#[belongs_to(Feature)]
pub struct Step {
    pub id: i32,
    pub slug: String,
    pub feature_id: i32,
    pub creator_id: i32,
    pub step_type: String,
    pub value: String,
    pub position: i32
}

#[derive(Debug)]
enum StepError {
    InvalidPosition(i32)
}

impl error::Error for StepError {}
impl fmt::Display for StepError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StepError::InvalidPosition(p) => write!(f, "Invalid position for step: {}, must be > 0.", p)
        }
    }
}

pub struct NewStep<'a> {
    pub feature: &'a Feature,
    pub creator: &'a User,
    pub step_type: String,
    pub value: String,
    pub position: Option<i32>
}

#[derive(Insertable)]
#[table_name="steps"]
struct NewStepInner {
    pub slug: String,
    pub feature_id: i32,
    pub creator_id: i32,
    pub step_type: String,
    pub value: String,
    pub position: i32
}

impl Step {
    fn new_inner<'a>(db: &PgConnection, new_step: NewStep<'a>) -> QueryResult<Step> {
        let position = match new_step.position {
            Some(v) => v,
            None => Step::next_position(db, new_step.feature)?
        };

        if position <= 0 {
            return Err(DieselError::QueryBuilderError(
                Box::new(StepError::InvalidPosition(position))));
        }

        let new_step_inner = NewStepInner {
            slug: format!("{} {}", &new_step.step_type, &new_step.value).to_kebab_case(),
            feature_id: new_step.feature.id,
            creator_id: new_step.creator.id,
            step_type: new_step.step_type,
            value: new_step.value,
            position: position
        };

        Ok(insert_into(schema::steps::table)
            .values(new_step_inner)
            .get_result(db).unwrap())
    }

    pub fn new<'a>(db: &PgConnection, new_step: NewStep<'a>) -> QueryResult<Step> {
        db.transaction::<Step, DieselError, _>(|| Step::new_inner(db, new_step))
    }

    pub fn move_to(db: &PgConnection, step: Step, position: i32) -> QueryResult<Vec<Step>> {
        if position <= 0 {
            return Err(DieselError::QueryBuilderError(
                Box::new(StepError::InvalidPosition(position))));
        }
        
        db.build_transaction().deferrable().run::<Vec<Step>, DieselError, _>(|| {
            use std::cmp::Ordering;
            use schema::steps::dsl as steps;

            let maybe_feature = Feature::find_by_id(db, step.feature_id)?;
            let feature = match maybe_feature {
                Some(v) => v,
                None => return Ok(vec![])
            };

            // Check which direction the filthy thing did a move
            let c = step.position;
            let t = position;
            let direction = c.cmp(&t);

            match direction {
                Ordering::Less => {
                    let next_steps: Vec<Step> = Step::belonging_to(&feature)
                        .order(steps::position.asc())
                        .filter(steps::position.gt(step.position).and(steps::position.le(position)))
                        .get_results(db)?;

                    for next_step in next_steps.into_iter() {
                        update(&next_step).set(steps::position.eq(&next_step.position - 1)).execute(db)?;
                    }
                },
                Ordering::Greater => {
                    let prev_steps: Vec<Step> = Step::belonging_to(&feature)
                        .order(steps::position.asc())
                        .filter(steps::position.lt(step.position).and(steps::position.ge(position)))
                        .get_results(db)?;

                    for prev_step in prev_steps.into_iter() {
                        update(&prev_step).set(steps::position.eq(&prev_step.position + 1)).execute(db)?;
                    }
                },
                Ordering::Equal => {
                    // Run away!
                    return Step::belonging_to(&feature)
                        .order(steps::position.asc())
                        .get_results(db)
                }
            };

            // Update the current step
            update(&step).set(steps::position.eq(position)).execute(db)?;

            // Get all steps and return
            Step::belonging_to(&feature)
                .order(steps::position.asc())
                .get_results(db)
        })
    }

    pub fn all_by_feature(db: &PgConnection, feature: &Feature) -> QueryResult<Vec<Step>> {
        use schema::steps::dsl as steps;

        Step::belonging_to(feature)
            .order(steps::position.asc())
            .get_results(db)
    }

    fn next_position(db: &PgConnection, feature: &Feature) -> QueryResult<i32> {
        use schema::steps::dsl as steps;

        let maybe_record: Option<Step> = schema::steps::table
            .filter(steps::feature_id.eq(feature.id))
            .order(steps::position.desc())
            .get_result(db)
            .optional()?;

        match maybe_record {
            Some(record) => Ok(record.position + 1),
            None => Ok(1)
        }
    }
}