use super::models::{NewUser, User};
use super::database::{PooledPg};
use super::result::*;
use diesel::prelude::*;
use scrypt::{ScryptParams, scrypt_simple, scrypt_check};

/// Create new user 
/// IMPORTANT: password is hashed as part of this function.
pub fn insert_user(create: NewUser, conn: PooledPg) -> Result<User> {
        debug!("Create user {:?}", &create.name);
        use super::schema::users;

        let params = ScryptParams::new(15, 8, 1).unwrap();

        let hashed_password = scrypt_simple(&create.password, &params).expect("OS RNG should not fail");

        let hashed_user = NewUser {name: create.name, password: hashed_password};

        diesel::insert_into(users::table)
                .values(&hashed_user)
                .get_result(&conn)
                .map_err(|e| ServerError::Database(e))
}