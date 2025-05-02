pub trait Entity {
    fn table_name() -> &'static str;
    fn primary_key() -> &'static str;
}