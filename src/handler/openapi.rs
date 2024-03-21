use crate::dto::response::MessageResponse;
use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
#[derive(OpenApi)]
#[openapi(
    info(
        version = "v0.1.0",
        title = "Merchant REST Api"
    ),
    paths(
        //server Api
        crate::handler::server::health_check,
    ),
    components(
        schemas(
            MessageResponse
        )
    ),
    tags(
        (name = "crate::handler::server", description = "server endpoints.")
    ),
    modifiers()
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "jwt",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        )
    }
}
