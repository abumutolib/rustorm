use postgres::{Client, NoTls};
use crate::error::OrmResult;

pub fn establish_connection(conn_str: &str) -> OrmResult<Client> {
    let client = Client::connect(conn_str, NoTls)?;

    Ok(client)
}