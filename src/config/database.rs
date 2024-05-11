use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;

pub fn establish_connection() -> Result<Pool<ConnectionManager<PgConnection>>, ()> {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            return Err(())
        }
    };
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    match Pool::builder()
    .build(manager) {
        Ok(pool) => Ok(pool.clone()),
        Err(_) => {
            println!("database is not set up");
            Err(())
        }
    }
}

#[macro_export]
macro_rules! init_connection_db {
    ($conn: ident, $connection: expr) => {
        #[allow(unused_mut)]
        let mut $conn = $connection.get().expect("something went wrong");
    };
}


#[macro_export]
macro_rules!  get_data{
    ($conn:expr, $struct: ident, $question:expr, $query:expr, $sql_type:ty, $name:expr) => {{
    use crate::config::helper;
    use diesel::{sql_query};
    use diesel::{RunQueryDsl};
    use crate::config::helper::ResponseJson;
use actix_web::HttpResponse;

    $question = match sql_query($query)
        .bind::<$sql_type,_>($name)
        .get_results::<$struct>(&mut $conn) {
            Ok(questions) => Some(questions),
            Err(err) => {
                println!("{:?}", err);
                return HttpResponse::InternalServerError().json(ResponseJson::<()> {
                    data: None,
                    message: String::from("Something went wrong"),
                    status: helper::Status::FAIL,
                    status_code: 500
                })
                            }
        };

    }};

    ($conn:expr, $struct: ident, $question:expr, $query:expr, $($sql_type:ty : $name: expr),* ) => {{
        $question = match sql_query($query)
            .$(bind::<$sql_type,_>($name)).*
            .get_results::<$struct>(&mut $conn) {
                Ok(questions) => Some(questions),
                Err(err) => {
                    println!("{:?}", err);
                    return HttpResponse::InternalServerError().json(ResponseJson::<()> {
                        data: None,
                        message: String::from("Something went wrong"),
                        status: crate::config::helper::Status::FAIL,
                        status_code: 500
                    })
                                }
            };

        }};
    ($conn:expr, $struct: ident, $question:expr, $query:expr, $($sql_type:ty : $name: expr),* ) => {{
        $question = match sql_query($query)
            .$(bind::<$sql_type,_>($name)).*
            .get_result::<$struct>(&mut $conn) {
                Ok(questions) => Some(questions),
                Err(err) => {
                    println!("{:?}", err);
                    return HttpResponse::InternalServerError().json(ResponseJson::<()> {
                        data: None,
                        message: String::from("Something went wrong"),
                        status: crate::config::helpers::Status::FAIL,
                        status_code: 500
                    })
                                }
            };

        }};
}
