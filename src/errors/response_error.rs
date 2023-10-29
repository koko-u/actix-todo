use actix_web::body;
use actix_web::http;
use actix_web::HttpResponse;
use askama_actix::TemplateToResponse;

use crate::templates::ServerErrorTemplate;

#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum AppResponseError {
    #[display("Validation Error")]
    ValidationError,
    #[display("Authentication Error")]
    AuthenticationError,
    #[display("Server Error")]
    ServerError,
}

impl actix_web::ResponseError for AppResponseError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            Self::AuthenticationError => http::StatusCode::UNAUTHORIZED,
            Self::ValidationError => http::StatusCode::BAD_REQUEST,
            Self::ServerError => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        let server_error = ServerErrorTemplate {
            error_message: format!("{self}"),
            ..Default::default()
        };
        server_error.to_response()
    }
}

impl<C> From<error_stack::Report<C>> for AppResponseError
where
    C: error_stack::Context,
{
    fn from(report: error_stack::Report<C>) -> Self {
        log::error!("{report:#?}");
        Self::ServerError
    }
}
