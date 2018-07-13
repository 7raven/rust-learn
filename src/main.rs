extern crate crypto;
extern crate reqwest;
extern crate json;
extern crate rand;


use crypto::md5::Md5;
use crypto::digest::Digest;

use rand::random;

struct Cfg{
    app_id: String,
    app_key: String,
    uri: String,
    from: String,
    to: String,
    salt: u8,
}

impl Cfg{
    fn build_url(self, q: String, sign: String) -> String{
        format!(
            "{}?appKey={}&q={}&from={}&to={}&salt={}&sign={}",
            self.uri, self.app_id, q, self.from,
            self.to, self.salt,sign
        )
    }
}

fn main() {

    let cfg = Cfg{
        app_id: String::from("2fbbf4ff8d5d7eb8"),
        app_key: String::from("9MBvQ6MJqbpEhlUpsKcBx0nTkvTxNcuR"),
        uri: String::from("http://openapi.youdao.com/api"),
        from: String::from("auto"),
        to: String::from("auto"),
        salt: random::<u8>()
    };

    let org_sign_str = format!("{}{}{}{}", cfg.app_id, "expected", cfg.salt, cfg.app_key);
    let sign= string_sign(org_sign_str);

    let uri = cfg.build_url(String::from("expected"), sign);
    println!("url: {}", uri);
    let body = reqwest::get(uri.as_str())
        .and_then(|mut req|{
            req.text()
        }).unwrap();

    println!("body ={}", body);
}

fn string_sign(org_str: String) -> String{
    let mut sign_md5 = Md5::new();
    sign_md5.input_str(&org_str.as_str());
    sign_md5.result_str().to_uppercase()
}