use askama::Template;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ServerErrorTemplate {
    pub error_message: String,
}
