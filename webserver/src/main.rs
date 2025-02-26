use actix_files as fs;
use actix_web::{
    App, HttpResponse, HttpServer, Result as ActixResult, get, http::header::ContentType, post, web,
};
use maud::{DOCTYPE, Markup, html};
use serde::Deserialize;

fn make_header_html(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
        script src="/assets/scripts/htmx.min.js"{}
        link rel="stylesheet" href="/assets/styles/pico.css"{}
        link rel="stylesheet" href="/assets/styles/counter.css"{}
    }
}

fn make_counter_html(count: u8) -> Markup {
    html! {
        section id="counter" {
            h1 { "Counter: "(count) }
            button
                class="outline"
                hx-post=(format!("/count/{}", count.saturating_add(1)))
                hx-target="#counter"
                {"increment"}
        }
    }
}

#[derive(Deserialize)]
struct CounterQuery {
    count: Option<u8>,
}
#[get("/")]
async fn respond_index(counter: web::Query<CounterQuery>) -> ActixResult<Markup> {
    let count = counter.count.unwrap_or_default();

    Ok(html! {
        (make_header_html("noobi.fi - index"))
        main class="container" {
            (make_counter_html(count))
        }
    })
}

#[post("/count/{count}")]
async fn respond_count(path: web::Path<u8>) -> HttpResponse {
    let count = path.into_inner();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .insert_header(("HX-Push-Url", format!("/?count={}", count)))
        .body(make_counter_html(count))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(respond_index)
            .service(respond_count)
            .service(fs::Files::new("/assets", "./assets"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
