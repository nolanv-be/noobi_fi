use maud::{DOCTYPE, Markup, html};

pub fn make_header_html(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
        script src="/assets/scripts/htmx.min.js"{}
        link rel="stylesheet" href="/assets/styles/pico.css"{}
        link rel="stylesheet" href="/assets/styles/counter.css"{}
    }
}
