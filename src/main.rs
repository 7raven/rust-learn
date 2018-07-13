extern crate crypto;
extern crate reqwest;
extern crate json;
extern crate rand;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;


use json::JsonValue;
use std::fs::File;
use std::io::Read;
use crypto::md5::Md5;
use crypto::digest::Digest;
use rand::random;

#[derive(Deserialize)]
struct Cfg{
    app_id: String,
    app_key: String,
    uri: String,
    from: String,
    to: String,
}

impl Cfg{
    fn build_url(self, q: String, sign: String, salt: u8) -> String{
        format!(
            "{}?appKey={}&q={}&from={}&to={}&salt={}&sign={}",
            self.uri, self.app_id, q, self.from,
            self.to, salt, sign
        )
    }
}

fn main() {
    let query = "回报";
    let cfg = get_cfg("api_cfg.toml");
    let salt = random::<u8>();
    let org_sign_str = format!("{}{}{}{}", cfg.app_id, query, salt, cfg.app_key);
    let sign= string_sign(org_sign_str);

    let uri = cfg.build_url(String::from(query), sign, salt);
    println!("url: {}", uri);
    let body = reqwest::get(uri.as_str())
        .and_then(|mut req|{
            req.text()
        }).unwrap();

    println!("body ={}", body);

    let json_data = json::parse(body.as_str()).unwrap();

    println!("{}", json_data["web"][0]["value"][0]);
}

fn get_cfg(filename: &str) -> Cfg{
    let mut file_handle = File::open(filename).unwrap();
    let mut contents = String::new();
    file_handle.read_to_string(&mut contents).expect("Can't open file");
    toml::from_str(&contents).expect("Can't to toml")
}

fn string_sign(org_str: String) -> String{
    let mut sign_md5 = Md5::new();
    sign_md5.input_str(&org_str.as_str());
    sign_md5.result_str().to_uppercase()
}
