use rustorm::prelude::*;

// Simulated entity model
#[derive(Entity)]
#[table = "users"]
struct User {
    id: i32,
    name: String,
}

fn main() -> Result<(), RustormError> {
    env_logger::init(); // optional, for logging
    // server=localhost;port=5431;database=fusion;username=postgres;password=postgres
    let mut conn = PgConnection::connect("server=localhost;port=5432;database=RustTestDb;username=postgres;password=postgres")?;
    
    let sql = "SELECT id, name FROM users"; // <--- clean here
    println!("DEBUG SQL: {:?}", sql); // <--- clean here
    let rows = conn.query(sql).unwrap();

    for row in rows {
        println!("id: {}, name: {}", row[0], row[1]);
    }

    Ok(())
}
