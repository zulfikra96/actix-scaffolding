use std::fmt;

use actix_web::{body::BoxBody, HttpResponse, Responder};
use sailfish::TemplateOnce;
use serde::{Serialize, Serializer};

pub enum Status {
    SUCCESS,
    FAIL,
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::FAIL => write!(f, "FAIL"),
            Self::SUCCESS => write!(f, "SUCCESS"),
        }
    }
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::FAIL => serializer.serialize_str("FAIL"),
            Self::SUCCESS => serializer.serialize_str("SUCCESS"),
        }
    }
}


#[derive(Debug, Serialize)]
pub struct ResponseJson<T> {
    pub message: String,
    pub status_code: isize,
    pub status: Status,
    pub data: Option<T>,
}



impl<T: Serialize> Responder for ResponseJson<T> {
    type Body = BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        match self.status {
            Status::FAIL => {
                if self.status_code == 401 {
                    return HttpResponse::Unauthorized().json(ResponseJson {
                        message: self.message,
                        status: self.status,
                        status_code: self.status_code,
                        data: self.data,
                    });
                }
                return HttpResponse::BadRequest().json(ResponseJson {
                    message: self.message,
                    status: self.status,
                    status_code: self.status_code,
                    data: self.data,
                });
            }
            Status::SUCCESS => {
                return HttpResponse::Ok().json(ResponseJson {
                    message: self.message,
                    status: self.status,
                    status_code: self.status_code,
                    data: self.data,
                })
            }
        }
    }
}

#[derive(TemplateOnce)]
#[template( path = "error.html")]
pub struct ErrorView {
   pub  message: String
}

#[derive(TemplateOnce)]
#[template( path = "notfound.html")]
pub struct NotFoundView {}