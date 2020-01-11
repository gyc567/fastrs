extern crate toml;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct IpConfig {
    pub name: Option<String>,
    pub ip: Option<String>,
    pub port: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub fastrs_url: Option<String>,
    pub fastrs_port: Option<u64>,
    pub ip_config: Option<Vec<IpConfig>>,
}
pub fn load_config(path: &str) -> Result<(Conf), &str> {
    let file_path = "fastrs.toml";
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", file_path, e),
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e),
    };
    let config: Conf = toml::from_str(&str_val).unwrap();
    // let ip_confs: Vec<IpConfig> = config.ip_config.unwrap();

    Ok((config))
}
pub fn addr(a: i32, b: i32) -> i32 {
    a + b
}
