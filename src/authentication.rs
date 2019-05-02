use super::database::PooledPg;
use super::models::{NewUser, User};
use super::result::*;
use diesel::prelude::*;
use scrypt::{scrypt_check, scrypt_simple, ScryptParams};

/// Create new user
/// IMPORTANT: password is hashed as part of this function.
pub fn insert_user(create: NewUser, conn: PooledPg) -> Result<User> {
        debug!("Create user {:?}", &create.name);
        use super::schema::users;

        let params = ScryptParams::new(15, 8, 1).unwrap();

        let hashed_password =
                scrypt_simple(&create.password, &params).expect("OS RNG should not fail");

        let hashed_user = NewUser {
                name: create.name,
                password: hashed_password,
        };

        diesel::insert_into(users::table)
                .values(&hashed_user)
                .get_result(&conn)
                .map_err(|e| ServerError::Database(e))
}

#[cfg(test)]
mod tests {
        use super::*;
        use crate::test_utils::connection;

        #[test]
        fn it_inserts_user_with_hashed_pass() {
                let new = NewUser {
                        name: "testName".to_string(),
                        password: "testPassword".to_string(),
                };
                let conn = connection();

                let stored_user = insert_user(new, conn).expect("insert_user should not fail");

                assert!(scrypt_check("testPassword", &stored_user.password).is_ok());
        }
}
