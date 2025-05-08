#[macro_export]
macro_rules! derive_model {
    ($name:ident, $table:literal, [$($field:ident),*]) => {
        impl Model for $name {
            fn table_name() -> &'static str {
                $table
            }
            fn fields() -> &'static [&'static str] {
                &[$(stringify!($field)),*]
            }
            fn to_refs(&self) -> Vec<&(dyn postgres::types::ToSql + Sync)> {
                vec![ $( &self.$field as &(dyn postgres::types::ToSql + Sync) ),* ]
            }
        }

        impl FromRow for $name {
            fn from_row(row: &postgres::Row) -> Self {
                Self {
                    $(
                        $field: row.get(stringify!($field)),
                    )*
                }
            }
        }
    }
}