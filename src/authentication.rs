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

        if create.name.is_empty() || create.password.is_empty() {
                return Err(ServerError::Input(
                        "name and password cannot be empty".to_string(),
                ));
        }

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
        use crate::test_utils::test_connection;
        use proptest::prelude::*;

        #[test]
        fn insert_user_returns_an_input_error_for_empty_strings() {
                //Tests empty user handled correctly

                let new = NewUser {
                        name: "".to_string(),
                        password: "".to_string(),
                };

                let conn = test_connection();

                assert_eq!(
                        insert_user(new, conn),
                        Err(ServerError::Input(
                                "name and password cannot be empty".to_string()
                        ))
                )
        }

        #[test]
        fn it_inserts_single_user_with_hashed_pass() {
                //Tests that password is properly hashed and inserted in DB

                let new = NewUser {
                        name: "testName".to_string(),
                        password: "testPassword".to_string(),
                };

                let conn = test_connection();

                let stored_user = insert_user(new, conn).expect("insert_user should not fail");

                assert!(scrypt_check("testPassword", &stored_user.password).is_ok());
        }

        proptest! {
        #[test] #[ignore] //too expensive property based version of inserts_single_user unit test, takes >1h to run
        fn it_inserts_user_with_hashed_pass(s1 in "\\w+", s2 in "\\w+") {
                let new = NewUser {
                        name: s1,
                        password: s2.clone(),
                };

                let conn = test_connection();

                let stored_user = insert_user(new, conn).expect("insert_user should not fail");

                assert!(scrypt_check(&s2, &stored_user.password).is_ok());
        }
        }
}
