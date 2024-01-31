use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    Pool::builder()
        .build(manager)
        .expect("Could not build connection pool")
    
}