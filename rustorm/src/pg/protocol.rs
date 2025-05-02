use crate::error::RustormError;

pub enum ServerMessage {
    ReadyForQuery,
    CommandComplete(String),
    DataRow(Vec<String>),
    RowDescription(Vec<String>),
    Error(String),
    Unknown,
}

pub fn parse_message(buf: &[u8]) -> Result<ServerMessage, RustormError> {
    match buf[0] as char {
        'R' => Ok(ServerMessage::ReadyForQuery),
        'T' => Ok(ServerMessage::RowDescription(vec![])), // simplify
        'D' => {
            let len = u16::from_be_bytes([buf[5], buf[6]]) as usize;
            let mut result = vec![];
            let mut i = 7;
            for _ in 0..len {
                let str_len = u16::from_be_bytes([buf[i], buf[i + 1]]) as usize;
                i += 2;
                let s = String::from_utf8(buf[i..i + str_len].to_vec())?;
                result.push(s);
                i += str_len;
            }
            Ok(ServerMessage::DataRow(result))
        }
        'C' => Ok(ServerMessage::CommandComplete("CommandComplete".into())),
        'E' => Ok(ServerMessage::Error("ErrorMessage".into())),
        _ => Ok(ServerMessage::Unknown),
    }
}