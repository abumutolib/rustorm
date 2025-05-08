use postgres::Client;
use crate::error::{OrmResult};
use crate::model::{Model, FromRow};

pub struct DbContext {
    pub client: Client,
}

impl DbContext {
    pub fn new(client: Client) -> Self {
        DbContext { client }
    }

    pub fn insert<T: Model>(&mut self, item: &T) -> OrmResult<u64> {
        let fields = T::fields().join(", ");
        let placeholders: Vec<String> = (1..=T::fields().len()).map(|i| format!("${}", i)).collect();

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            T::table_name(),
            fields,
            placeholders.join(", ")
        );

        let refs = item.to_refs();
        let params: Vec<&(dyn postgres::types::ToSql + Sync)> = refs.into_iter().collect();
        let rows = self.client.execute(&sql, &params)?;
        Ok(rows)
    }

    pub fn find_by_id<T: Model + FromRow>(&mut self, id: i32) -> OrmResult<Option<T>> {
        let sql = format!("SELECT {} FROM {} WHERE id = $1", T::fields().join(", "), T::table_name());
        let rows = self.client.query(&sql, &[&id])?;
        if let Some(row) = rows.get(0) {
            Ok(Some(T::from_row(row)))
        } else {
            Ok(None)
        }
    }
}