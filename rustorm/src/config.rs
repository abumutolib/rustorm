// pub struct DbConfig {
//     pub user: String,
//     pub password: String,
//     pub host: String,
//     pub port: u16,
//     pub dbname: String,
// }
// // server=localhost;port=5432;database=fusion;username=postgres;password=postgres
// impl DbConfig {
//     pub fn from_url(url: &str) -> Result<Self, String> {
//         // Very naive parser, not production-ready
//         let url = url.trim_start_matches("postgres://");
//         let parts: Vec<&str> = url.split(['@', '/', ':']).collect();
//         if parts.len() < 5 {
//             return Err("Invalid URL".to_string());
//         }
//         Ok(DbConfig {
//             user: parts[0].to_string(),
//             password: parts[1].to_string(),
//             host: parts[2].to_string(),
//             port: parts[3].parse().unwrap_or(5432),
//             dbname: parts[4].to_string(),
//         })
//     }
// }

//server=localhost;port=5432;database=fusion;username=postgres;password=postgres
pub struct DbConfig {
    pub port: u16,
    pub server: String,
    pub database: String,
    pub username: String,
    pub password: String
}

impl DbConfig {
    pub fn from_url(connection: &str)  -> Result<Self, String> {
        let mut server: Option<String> = None;
        let mut port: Option<u16> = None;
        let mut database: Option<String> = None;
        let mut username: Option<String> = None;
        let mut password: Option<String> = None;

        for part in connection.split(';') {
            let mut iter = part.splitn(2, '=');
            let key = iter.next().ok_or("Invalid key=value pair")?.trim().to_lowercase();
            let value = iter.next().ok_or(format!("Missing value for key '{}'", key))?.trim().to_string();

            match key.as_str() {
                "server" | "host" => server = Some(value),
                "port" => {
                    let parsed_port = value.parse::<u16>().map_err(|_| format!("Invalid port: '{}'", value))?;
                    port = Some(parsed_port);
                }
                "database" => database = Some(value),
                "username" | "user" => username = Some(value),
                "password" => password = Some(value),
                _ => return Err(format!("Unknown key: '{}'", key)),
            }
        }

        Ok(Self {
            server: server.ok_or("Missing 'server'")?,
            port: port.ok_or("Missing or invalid 'port'")?,
            database: database.ok_or("Missing 'database'")?,
            username: username.ok_or("Missing 'username'")?,
            password: password.ok_or("Missing 'password'")?,
        })
    }
}