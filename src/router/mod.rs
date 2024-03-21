
use crate::{handler::openapi::ApiDoc, server::state::AppState};
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod server;

pub fn create_router_app(state: AppState) -> Router {
    let router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let router = server::add_routers(router);
    router.with_state(state)

}