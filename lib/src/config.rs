extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};

pub struct ServerConfig {
    pub user_cap:         u64,
    pub thread_count:     u64,
    pub thread_max_count: u64,
    pub mgr_user:         String,
    pub mgr_pass:         String,
    pub mgr_ssl_only:     bool,
    pub mssql_ip:         String,
    pub mssql_port:       u16,
    pub mssql_user:       String,
    pub mssql_pass:       String,
    pub mssql_database:   String,
    pub http_ip:          String,
    pub http_port:        u16,
    pub https_ip:         String,
    pub https_port:       u16,
    pub ws_ip:            String,
    pub ws_port:          u16,
    pub wss_ip:           String,
    pub wss_port:         u16,
}

pub fn env_to_config_path(env: String) -> String {
    let mut exe_path: String =     format!("{}", std::env::current_exe().unwrap().to_string_lossy());
    exe_path = exe_path.chars().into_iter().take(exe_path.len() - String::from("/server").len()).collect();;
    return format!("{}/../../../config/{}.yaml", exe_path, env);
}

pub fn read(env: String) -> ServerConfig  {
    let config_path = env_to_config_path(env);

    let docs = YamlLoader::load_from_str(&std::fs::read_to_string(config_path).unwrap()).unwrap();
    let doc = &docs[0];

    let output = ServerConfig {
        user_cap:         doc["user_cap"].as_i64().unwrap() as u64,
        thread_max_count: doc["thread_max_count"].as_i64().unwrap() as u64,
        thread_count:     doc["thread_count"].as_i64().unwrap() as u64,
        mgr_user:         String::from(doc["mgr_user"].as_str().unwrap()),
        mgr_pass:         String::from(doc["mgr_pass"].as_str().unwrap()),
        mgr_ssl_only:     doc["mgr_ssl_only"].as_bool().unwrap(),
        mssql_ip:         String::from(doc["mssql_ip"].as_str().unwrap()),
        mssql_port:       doc["mssql_port"].as_i64().unwrap() as u16,
        mssql_user:       String::from(doc["mssql_user"].as_str().unwrap()),
        mssql_pass:       String::from(doc["mssql_pass"].as_str().unwrap()),
        mssql_database:   String::from(doc["mssql_database"].as_str().unwrap()),
        http_ip:          String::from(doc["http_ip"].as_str().unwrap()),
        http_port:        doc["http_port"].as_i64().unwrap() as u16,
        https_ip:         String::from(doc["https_ip"].as_str().unwrap()),
        https_port:       doc["https_port"].as_i64().unwrap() as u16,
        ws_ip:            String::from(doc["ws_ip"].as_str().unwrap()),
        ws_port:          doc["ws_port"].as_i64().unwrap() as u16,
        wss_ip:           String::from(doc["wss_ip"].as_str().unwrap()),
        wss_port:         doc["wss_port"].as_i64().unwrap() as u16,
    };

    return output;
}
