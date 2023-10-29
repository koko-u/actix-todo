use std::time::Duration;

use actix_identity::IdentityMiddleware;

pub fn build_identity() -> IdentityMiddleware {
    IdentityMiddleware::builder()
        .login_deadline(Duration::from_secs(24 * 60 * 60).into())
        .build()
}
