#[macro_use]
extern crate serde_derive;
extern crate fastrs;
extern crate speculate;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;

use fastrs::peer_config::*;
use speculate::speculate;

// #[derive(Deserialize, Debug)]
// struct IpConfig {
//     name: Option<String>,
//     ip: Option<String>,
//     port: Option<String>,
// }

// #[derive(Deserialize, Debug)]
// struct Conf {
//     global_string: Option<String>,
//     global_integer: Option<u64>,
//     ip_config: Option<Vec<IpConfig>>,
// }

speculate! {
    describe "config file verify" {

        before {
        let file_path = "peer.toml";
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

        }

        it "test global_string can get right value"{
        // let global_string = config.global_string;
        assert_eq!(config.global_string, Some(String::from("test")));
        }

        it "test global_integer can get right value"{
        // let global_string = config.global_string;
        assert_eq!(config.global_integer, Some(5));
        }
        it "test the lib fn from crate"{
              assert_eq!(fastrs::add(3, 2), 5);

        }
    }
}
