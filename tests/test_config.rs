#[allow(dead_code)]
#[macro_use]
extern crate serde_derive;
extern crate fastrs;
extern crate speculate;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;

use fastrs::fastrs_config::*;

use speculate::speculate;

speculate! {
    describe "config file verify" {

        before {
        // let file_path = "fastrs.toml";
        // let mut file = match File::open(file_path) {
        //     Ok(f) => f,
        //     Err(e) => panic!("no such file {} exception:{}", file_path, e),
        // };
        // let mut str_val = String::new();
        // match file.read_to_string(&mut str_val) {
        //     Ok(s) => s,
        //     Err(e) => panic!("Error Reading file: {}", e),
        // };
        // let config: Conf = toml::from_str(&str_val).unwrap();
        // let  ip_confs:Vec<IpConfig>=config.ip_config.unwrap();
        // let ip_conf=ip_confs[0].clone();
        // let  name=ip_conf.name;
        // let  ip=ip_conf.ip;
        // let   port =ip_conf.port;
           let config =load_config("").unwrap();

           let  ip_confs:Vec<IpConfig>=config.ip_config.unwrap();
           let ip_conf=ip_confs[0].clone();
           let  name=ip_conf.name;
           let  ip=ip_conf.ip;
           let   port =ip_conf.port;

    }

        it "test global_string can get right value"{
        // let global_string = config.global_string;
        assert_eq!(config.fastrs_url, Some(String::from("127.0.0.1")));
        }

        it "test global_integer can get right value"{
        // let global_string = config.global_string;
        assert_eq!(config.fastrs_port, Some(9098));
        }
         it "test ipconf name  can get right value"{
        // let global_string = config.global_string;
        assert_eq!(name, Some(String::from("CN")));
        }
         it "test ipconf ip  can get right value"{
        // let global_string = config.global_string;
        assert_eq!(ip, Some(String::from("127.0.0.1")));
        }
        it "test ipconf port  can get right value"{
        // let global_string = config.global_string;
        assert_eq!(port, Some(String::from("80")));
        }

    }
}
