use std::fmt;

use actix_web::{body::BoxBody, HttpResponse, Responder};
use sailfish::TemplateOnce;
use serde::{Serialize, Serializer};

#[allow(dead_code)]
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


#[macro_export]
macro_rules! response_json {
    (UNAUTHORIZED) => {{
       use crate::config::helper::ResponseJson;
        HttpResponse::Unauthorized().json(ResponseJson::<()> {
            status: Status::FAIL,
            message: "Unauthorized".to_string(),
            data: None,
            status_code: 401,
        })
    }};
    (OK, $data_type: ty, $data: expr) => {
        HttpResponse::Ok().json(ResponseJson::<$data_type> {
            status: Status::FAIL,
            message: "Success".to_string(),
            data: Some($data),
            status_code: 401,
        })
    };
     (OK) => {{
         use crate::config::helper::Status;
        use actix_web::HttpResponse;
        use crate::config::helper::ResponseJson;

        HttpResponse::Ok().json(ResponseJson::<()> {
            status: Status::FAIL,
            message: "Success".to_string(),
            data: None,
            status_code: 200,
        })
    }};
    (NOTFOUND, $msg: expr) => {{
        use crate::config::helper::Status;
        use actix_web::HttpResponse;
        use crate::config::helper::ResponseJson;

        HttpResponse::NotFound().json(ResponseJson::<()> {
            status: Status::FAIL,
            message: $msg.to_string(),
            data: None,
            status_code: 404,
        })
    }};
    (BADREQUEST, $msg: expr) => {{
        use crate::config::helper::Status;
        use crate::config::helper::ResponseJson;
        HttpResponse::BadRequest().json(ResponseJson::<()> {
            status: Status::FAIL,
            message: $msg.to_string(),
            data: None,
            status_code: 400,
        })
    }};

    (BADREQUEST, $msg: expr) => {{
        use crate::config::helper::Status;
        use crate::config::helper::ResponseJson;
        HttpResponse::BadRequest().json(ResponseJson::<()> {
            status: Status::FAIL,
            message: $msg.to_string(),
            data: None,
            status_code: 400,
        })
    }};
}
