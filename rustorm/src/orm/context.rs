use crate::pg::connect::PgConnection;

pub struct DbContext {
    pub connection: PgConnection,
}

impl DbContext {
    pub fn new(conn: PgConnection) -> Self {
        DbContext { connection: conn }
    }

    pub fn from_url(url: &str) -> Self {
        DbContext { connection: PgConnection::connect(url).unwrap() }
    }
}