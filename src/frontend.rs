use axum::{
    http::header,
    response::{Html, IntoResponse},
};

mod assets;
mod page;

pub async fn index() -> Html<&'static str> {
    Html(page::INDEX_HTML)
}

pub async fn stylesheet() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        assets::STYLES_CSS,
    )
}

pub async fn script() -> impl IntoResponse {
    (
        [(
            header::CONTENT_TYPE,
            "application/javascript; charset=utf-8",
        )],
        assets::APP_JS,
    )
}
