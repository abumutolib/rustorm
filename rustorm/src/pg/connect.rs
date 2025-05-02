use crate::tcp::stream::PgStream;
use crate::tcp::packet::build_startup_packet;
use crate::error::RustormError;

pub struct PgConnection {
    pub stream: PgStream,
}

impl PgConnection {
    pub fn connect(url: &str) -> Result<Self, RustormError> {
        let config = crate::config::DbConfig::from_url(url).map_err(|e| RustormError::Connection(e))?;
        let mut stream = PgStream::connect(&config.server, config.port)?;
        let startup = build_startup_packet(&config.username, &config.database);
        stream.send(&startup)?;
        Ok(PgConnection { stream })
    }
}