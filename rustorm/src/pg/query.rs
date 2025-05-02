use crate::error::RustormError;
use crate::pg::connect::PgConnection;
use crate::pg::protocol::{parse_message, ServerMessage};

impl PgConnection {
    pub fn query(&mut self, sql: &str) -> Result<Vec<Vec<String>>, RustormError> {
        let mut msg = vec!['Q' as u8];
        let mut body = sql.as_bytes().to_vec();
        body.push(0);
        let len = (body.len() + 4) as u32;
        msg.extend_from_slice(&len.to_be_bytes());
        msg.extend(body);
        self.stream.send(&msg)?;

        let mut results = vec![];
        loop {
            let mut buf = [0u8; 2048];
            let size = self.stream.receive(&mut buf)?;
            let message = parse_message(&buf[..size])?;

            match message {
                ServerMessage::DataRow(row) => results.push(row),
                ServerMessage::ReadyForQuery => break,
                ServerMessage::CommandComplete(_) => continue,
                ServerMessage::RowDescription(_) => continue,
                ServerMessage::Error(msg) => return Err(RustormError::Query(msg)),
                ServerMessage::Unknown => continue,
            }
        }

        Ok(results)
    }
}
