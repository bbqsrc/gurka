use schema;
use schema::{users, user_sessions};
use diesel;
use diesel::{PgConnection, insert_into};
use diesel::prelude::*;

use ring::{pbkdf2, digest};
use ring::rand::{SystemRandom, SecureRandom};
use uuid::Uuid;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub iterations: i32,
    pub salt: Vec<u8>,
    pub credential: Vec<u8>
}

impl User {
    pub fn verify_password(&self, password: &str) -> bool {
        match pbkdf2::verify(DIGEST_ALG, self.iterations as u32, &self.salt, &password.as_bytes(), &self.credential) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn find_by_username(db: &PgConnection, username: &str) -> diesel::result::QueryResult<Option<User>> {
        use schema::users::dsl as users;

        schema::users::table
            .filter(users::username.eq(&username.to_lowercase()))
            .get_result(db)
            .optional()
    }
}

#[derive(Queryable, Insertable, Debug)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: i32
}

impl UserSession {
    pub fn create(db: &PgConnection, user_id: i32) -> diesel::result::QueryResult<UserSession> {
        let record = UserSession {
            id: Uuid::new_v4(),
            user_id: user_id
        };

        insert_into(schema::user_sessions::table)
            .values(&record)
            .execute(db)?;
        
        Ok(record)
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub iterations: i32,
    pub salt: Vec<u8>,
    pub credential: Vec<u8>
}

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

impl NewUser {
    pub fn new(username: String, password: String, iterations: u32, salt_len: usize) -> NewUser {
        let rand = SystemRandom::new();
        let mut salt = vec![0; salt_len];
        rand.fill(&mut salt).unwrap();
        
        let mut out: Credential = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(DIGEST_ALG, iterations, &salt, password.as_bytes(), &mut out);
        
        NewUser {
            username: username.to_lowercase(),
            iterations: iterations as i32,
            salt: salt,
            credential: out.to_vec()
        }
    }

    pub fn create(db: &PgConnection, username: &str, password: String) -> diesel::result::QueryResult<User> {
        use schema::users::dsl as users;
        let username = username.to_lowercase();

        let new_user = NewUser::new(username, password, 10000, 16);

        insert_into(schema::users::table)
            .values(&new_user)
            .execute(db)?;

        let record: User = schema::users::table
            .filter(users::username.eq(&new_user.username))
            .get_result(db)?;

        Ok(record)
    }
}