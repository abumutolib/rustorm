pub enum PgType {
    Int4,
    Text,
    Unknown,
}

pub fn from_oid(oid: u32) -> PgType {
    match oid {
        23 => PgType::Int4,
        25 => PgType::Text,
        _ => PgType::Unknown,
    }
}