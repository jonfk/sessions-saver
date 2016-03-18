
use postgres;
use postgres::Connection;

use std::str::from_utf8;
use std::result::Result;

use auth;
use errors;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<i64>,
    pub email: String,
    pub password: String
}


pub fn create_user(conn: &::PostgresPooledConnection, email: String, password_unhashed: String) -> Result<u64, postgres::error::Error> {
    let password_hashed = auth::hash_password(password_unhashed).unwrap();

    match conn.prepare("INSERT INTO users (email, password) VALUES ($1, $2)") {
        Ok(stmt) => stmt.execute(&[&email, &password_hashed]),
        Err(err) => Err(err),
    }
}

pub fn get_user_by_email(conn: &::PostgresPooledConnection, email: String) -> Result<User, errors::Error> {
    let stmt = try!(conn.prepare("SELECT * FROM users WHERE email = $1"));
    let rows = try!(stmt.query(&[&email]));

    if rows.len() < 1 {
        Err(errors::Error::NotFoundErr(format!("Cannot find account with email {}", email)))
    } else {
        let id: i64 = rows.get(0).get(0);
        let email: String = rows.get(0).get(1);
        let password_hashed: String = rows.get(0).get(2);
        Ok(User{
            id: Some(id),
            email: email,
            password: password_hashed,
        })
    }
}
