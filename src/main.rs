use axum::response::Html;
use axum::Router;
use axum::routing::{get, get_service};
use maud::{html, DOCTYPE};
use tokio::net::TcpListener;
use tower_http::body::Full;
use tower_http::compression::CompressionLayer;
use tower_http::services::fs::ServeDir;
use tower_http::services::Redirect;
use tower_http::trace::TraceLayer;
use tracing::Level;

#[tokio::main]
async fn main() {
    // Set up tracing
    let subscriber= tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // Set up routes
    let app = Router::new()
        .route("/", get(|| async { Html(render_hero()) }))
        .route("/boop", get(|| async { ":3".to_owned() }))
        .route("/discord", get_service(Redirect::<Full>::temporary("https://discord.gg/tampafurryclub".parse().unwrap())))
        .route("/telegram", get_service(Redirect::<Full>::temporary("https://t.me/+8ID4Z2VpbadlZmUx".parse().unwrap())))
        .fallback(get_service(ServeDir::new("static")))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    // Launch
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn render_hero() -> String {
    wrapper(inner()).into_string()
}

fn wrapper(content: maud::PreEscaped<String>) -> maud::PreEscaped<String> {
    html! {
        (DOCTYPE)
        html style="visibility: hidden;" {
            head {
                meta charset="utf-8";
                meta content="width=device-width, initial-scale=1" name="viewport";
                meta name="description" content="Looking for furries in Tampa, FL? Here we are.";
                link rel="stylesheet" href="/style/main.css";
                title { "Tampa Furry Club" }
            }
            body style="background-color: rgb(28,27,34);" {
                #dots {
                    (content)
                }
            }
        }
    }
}

fn inner() -> maud::PreEscaped<String> {
    html! {
        #hero_box {
            img #hero_image src="img/logo.png";
            #hero_content_box {
                #text_box {
                    h1 #hero_title { "Looking for Furries in Tampa?" }
                    h2 #hero_subtitle { "Here we are." }
                }
                #button_box {
                    ({button("discord", "Discord Server", "/discord")})
                    ({button("telegram", "Telegram Feed", "/telegram")})
                }
            }
        }
    }
}

fn button(id: &str, text: &str, href: &str) -> maud::PreEscaped<String> {
    html! {
        a href=(href) target="_blank" {
            #(id) .button {
                (text)
            }
        }
    }
}