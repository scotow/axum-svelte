use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use mime_guess::MimeGuess;
use rust_embed::Embed;
use tokio::net::TcpListener;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Assets::router()
        .merge(Router::new().route("/:alias", get(|Path(path): Path<String>| async { path })));
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Embed)]
#[folder = "ui/build"]
#[prefix = "/"]
struct Assets;

impl Assets {
    fn router() -> Router {
        let mut router = Router::new();
        for path in Self::iter() {
            let mime = MimeGuess::from_path(path.as_ref())
                .first_or_octet_stream()
                .as_ref()
                .to_owned();
            let content = Self::get(&path).unwrap().data;
            let handler =
                get(move || async { (StatusCode::OK, [("Content-Type", mime)], content) });
            router = router.route(&path, handler.clone());
            if path.ends_with("/index.html") {
                router = router.route(&path.strip_suffix("index.html").unwrap(), handler)
            }
        }
        router
    }
}
