use std::fmt::Display;

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use color_eyre::Report;
use derive_more::Error;
use log::error;
use serde_json::json;

#[derive(Debug, Error)]
pub enum LolEsportsApiError {
    InternalError {
        message: Option<String>,
        e: color_eyre::Report,
    },
    PageNotFound {
        path: String,
    },
    NotFound {
        message: String,
    },
}

impl LolEsportsApiError {
    pub fn not_found<T: Into<String>>(message: T) -> Self {
        Self::NotFound {
            message: message.into(),
        }
    }

    pub fn page_not_found<T: Into<String>>(path: T) -> Self {
        Self::PageNotFound { path: path.into() }
    }

    pub fn internal_error(err: Report) -> Self {
        Self::InternalError {
            message: None,
            e: err,
        }
    }
}

impl From<color_eyre::Report> for LolEsportsApiError {
    fn from(value: color_eyre::Report) -> Self {
        Self::internal_error(value)
    }
}

impl Display for LolEsportsApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::NotFound { message } => json!({ "message": message, "status": 404 }),
            Self::InternalError { .. } => json!({
                "message": "Internal Server Error",
                "status": 500
            }),
            Self::PageNotFound { path } => json!({
                "status": 404,
                "path": path,
                "message": "There is no resource on this path"
            }),
        };
        f.write_str(s.to_string().as_str())
    }
}

impl ResponseError for LolEsportsApiError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        if let Self::InternalError { e, .. } = self {
            error!("Hit Error {:?}", e);
        }
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            LolEsportsApiError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            LolEsportsApiError::PageNotFound { .. } | LolEsportsApiError::NotFound { .. } => {
                StatusCode::NOT_FOUND
            }
        }
    }
}
