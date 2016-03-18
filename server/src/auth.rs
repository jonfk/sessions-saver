
use crypto::scrypt::{scrypt_simple, ScryptParams, scrypt_check};
use std::io;
use std::result::Result;

use jwt::{encode, decode, Header, Algorithm};
use jwt;

use postgres;

use errors;
use store;

pub fn hash_password(password_unhashed: String) -> io::Result<String> {
    let scrypt_params = ScryptParams::new(10, 8, 1);
    scrypt_simple(&password_unhashed, &scrypt_params)
}

pub fn verify_password(password_hashed: String, to_check: String) -> Result<bool, errors::Error> {
    match scrypt_check(&password_hashed, &to_check){
        Ok(res) => Ok(res),
        Err(err) => Err(errors::Error::RustCryptoErr(err)),
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub success: bool,
    pub token: String,
}

pub fn login(conn: &::PostgresPooledConnection, email: String, password_unchecked: String) -> Result<Login, errors::Error> {

    let user = try!(store::users::get_user_by_email(conn, email));

    if try!(verify_password(user.password, password_unchecked)) {
        let token = try!(encode_jwt(user.id));
        Ok(Login{success:true, token:token})
    } else {
        Ok(Login{success: false, token:"".to_string()})
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Claims {
    pub account_id: i64,
}

pub fn encode_jwt(account_id: i64) -> Result<String, jwt::errors::Error> {
    let my_claims = Claims { account_id : account_id };
    encode(Header::default(), &my_claims, "secret".as_ref())
}

pub fn decode_jwt(token: String) -> Result<Claims, jwt::errors::Error> {
    match decode::<Claims>(&token, "secret".as_ref(), Algorithm::HS256) {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => Err(err)
    }
}
