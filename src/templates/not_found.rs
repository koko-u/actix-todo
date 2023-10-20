use askama::Template;

#[derive(Template)]
#[template(path = "notfound.html")]
pub struct NotFoundTemplate;