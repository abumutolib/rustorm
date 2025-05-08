use rustorm::{error::OrmResult, *};

#[derive(Debug)]
struct User {
    id: i32,
    name: String
}

derive_model!(User, "users", [id, name]);

fn main() -> OrmResult<()> {
    let client = establish_connection("host=localhost user=postgres password=StarLab0101 dbname=RustTestDb")?;
    let mut ctx = DbContext::new(client);

    if let Some(found) = ctx.find_by_id::<User>(4)? {
        let user = User { id: found.id + 1, name: "Abu Tolib".to_string() };
        ctx.insert(&user)?;
        println!("Found user: {:?}", found);
    } else {
        println!("User not found");
    }

    Ok(())
}