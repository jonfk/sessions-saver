#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde;
extern crate serde_json;

#[cfg(all(test, feature="it_test"))]
mod test {
    //! These are integration tests. The server should be running when they are run.
    //!


    use serde_json;
    use std::io::Read;

    use hyper;
    use hyper::Client;
    use hyper::header::{Headers, ContentType};
    use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct LoginSuccess {
        pub success: bool,
        pub token: String
    }

    #[test]
    fn test_create_account_and_login() {
        let mut client = Client::new();

        let account = r#"{"email":"test@example.com", "password":"12345678"}"#;

        // Creating an outgoing request.
        let mut headers = Headers::new();
        headers.set(
            ContentType(Mime(TopLevel::Application, SubLevel::Json,
                             vec![(Attr::Charset, Value::Utf8)]))
                );
        let mut res = client.post("http://localhost:3000/signup").headers(headers).body(account).send().unwrap();
        assert_eq!(res.status, hyper::Ok);

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        println!("{}",body);

        let mut res = client.post("http://localhost:3000/login").header(ContentType::json()).body(account).send().unwrap();
        assert_eq!(res.status, hyper::Ok);

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        println!("{}",body);

        let loginSuccess: LoginSuccess = serde_json::from_str(&body).unwrap();
        assert!(loginSuccess.success);
    }

    #[test]
    fn test_create_account_twice() {
        // handle error when account is created twice with same email. will receive unique violation
    }
}
