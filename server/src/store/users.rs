
use postgres::{Connection, Result};

use std::str::from_utf8;

use auth;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<i64>,
    pub email: String,
    pub password: String
}


pub fn create_user(conn: &::PostgresPooledConnection, email: String, password_unhashed: String) -> Result<u64> {
    let password_hashed = auth::hash_password(password_unhashed).unwrap();

    let stmt = conn.prepare("INSERT INTO users (email, password) VALUES ($1, $2)").unwrap();
    stmt.execute(&[&email, &password_hashed])
}

pub fn login(conn: &::PostgresPooledConnection, email: String, password_unchecked: String) -> bool {
    let stmt = conn.prepare("SELECT * FROM users WHERE email = $1").unwrap();
    let rows = stmt.query(&[&email]).unwrap();

    if rows.len() < 1 {
        false
    } else {
        let password_hashed: String = rows.get(0).get(2);
        //println!("{}", password_hashed);
        auth::verify_password(password_hashed, password_unchecked).unwrap()
    }

}
