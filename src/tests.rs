use super::authentication;
use super::models::*;
use scrypt::scrypt_check;
use super::test_utils::connection;

mod authentication_tests {
use super::*;

#[test]
fn it_inserts_user_with_hashed_pass() {
    let new = NewUser { name: "testName".to_string(), password: "testPassword".to_string()};
    let conn = connection();

    let stored_user = authentication::insert_user(new, conn).expect("insert_user should not fail");

    assert!(scrypt_check("testPassword", &stored_user.password).is_ok());

}
}