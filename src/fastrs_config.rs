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
