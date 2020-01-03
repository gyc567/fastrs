#[derive(Deserialize, Debug)]
pub struct IpConfig {
    pub name: Option<String>,
    pub ip: Option<String>,
    pub port: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub global_string: Option<String>,
    pub global_integer: Option<u64>,
    pub ip_config: Option<Vec<IpConfig>>,
}
