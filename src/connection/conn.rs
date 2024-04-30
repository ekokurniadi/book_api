use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

trait IDatabaseConnection<'a> {
    fn get() -> &'a mut DatabaseConnection;
}

struct DatabaseConnection;

impl<'a> IDatabaseConnection<'a> for DatabaseConnection {
    fn get() -> &'a mut DatabaseConnection {
        todo!()
    }
}

pub fn config_db() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL MUST BE SET");
    let connection =
        PgConnection::establish(&database_url).unwrap_or_else(|c| panic!("Error connection {}", c));
    connection
}
