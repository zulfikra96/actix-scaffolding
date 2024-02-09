use std::fmt;

use actix_session::{Session, SessionExt};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::header,
    HttpResponse, ResponseError,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};

use crate::models::users::{Role, UserToken};

pub fn encrypt(plain: String) -> String {
    let private_key = std::env::var("PRIVATE_KEY").expect("Private key is not defined");
    let mc = new_magic_crypt!(private_key, 256);
    let base64 = mc.encrypt_str_to_base64(plain);
    base64
}

pub fn decrypt(chipper: &String) -> String {
    let private_key = std::env::var("PRIVATE_KEY").expect("Private key is not defined");
    let mc = new_magic_crypt!(private_key, 256);
    let plain_text = mc
        .decrypt_base64_to_string(chipper)
        .expect("something went wrong");
    plain_text
}

pub fn generate_token<T: Serialize + for<'a> Deserialize<'a>>(claims: T) -> String {
    let private_key = std::env::var("PRIVATE_KEY").expect("Something went wrong");
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(private_key.as_ref()),
    )
    .expect("Something went wrong encoding");

    token
}

pub enum ResponseType {
    PAGE,
    JSON,
}

#[derive(TemplateOnce)]
#[template(path = "notfound.html")]
struct NotFound {}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseErr {
    pub message: String,
    pub status_code: isize,
}

impl fmt::Display for ResponseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.message)
    }
}

impl ResponseError for ResponseErr {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        if self.status_code == 401 {
            HttpResponse::Unauthorized().json(ResponseErr {
                message: "Unauthorized".to_string(),
                status_code: self.status_code,
            })
        } else {
            HttpResponse::BadRequest().json(ResponseErr {
                message: "Something went wrong".to_string(),
                status_code: 405,
            })
        }
    }
}

fn error_filter(
    response_type: ResponseType,
    res: ServiceRequest,
    message: String,
) -> ServiceResponse {
    match response_type {
        ResponseType::JSON => {
            return res.error_response(ResponseErr {
                message,
                status_code: 401,
            })
        }
        ResponseType::PAGE => {
            let ctx = NotFound {};
            return res.into_response(HttpResponse::Ok().body(ctx.render_once().unwrap()));
        }
    }
}

pub fn auth(
    req: ServiceRequest,
    _role: Vec<Role>,
    response_type: ResponseType,
) -> Result<ServiceRequest, ServiceResponse> {
    let private_key = std::env::var("PRIVATE_KEY").unwrap();
    let payload = req.get_session();

    let session = Session::from(payload);
    // println!("{}")
    let bearer_token = req.headers().get(header::AUTHORIZATION);
    let token = match session.get::<String>("token") {
        Ok(res) => {
            if res.is_none() && bearer_token.is_none() {
                return Err(error_filter(response_type, req, "unauthorized".to_string()));
            } else if res.is_some() {
                res.unwrap()
            } else {
                let token: Vec<&str> = bearer_token.unwrap().to_str().unwrap().split(" ").collect();
                // println!("call error {:?}", token[0] );

                token[1].to_string()
            }
        }
        Err(_) => {
            let token = bearer_token.unwrap().to_str().unwrap().to_string();
            print!("token {}", token);
            return Err(error_filter(response_type, req, "unauthorized".to_string()));
        }
    };
    let claim = match decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(private_key.as_ref()),
        &Validation::default(),
    ) {
        Ok(res) => res,
        Err(err) => {
            println!("error :{:?}", err);
            return Err(error_filter(response_type, req, "unauthorized".to_string()));
        }
    };
    if !_role.contains(&claim.claims.role) {
        return Err(error_filter(response_type, req, "Unauthorized".to_string()));
    }
    // let token = match token {
    //     None => match response_type {
    //         ResponseType::JSON =>
    //     }
    // }

    Ok(req)
}
