use postgres::{types::ToSql, Row};

pub trait Model {
    fn table_name() -> &'static str;
    fn fields() -> &'static [&'static str];
    fn to_refs(&self) -> Vec<&(dyn ToSql + Sync)>;
}

pub trait FromRow : Sized {
    fn from_row(row: &Row) -> Self;
}