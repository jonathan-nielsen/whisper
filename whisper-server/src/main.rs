use warp::Filter;
use serde::Deserialize;

#[derive(Deserialize)]
struct Body {
    first_name: String,
    last_name: String,
}

#[tokio::main]
async fn main() {
    let hello_get = warp::get()
        .and(warp::path!("hello" / String))
        .map(|name: String| {
            let decoded = percent_encoding::percent_decode(name.as_bytes()).decode_utf8_lossy();
            format!("GET Hello, {}!", decoded)
        });

    let hello_delete = warp::delete()
        .and(warp::path!("hello" / String))
        .map(|name: String| {
            let decoded = percent_encoding::percent_decode(name.as_bytes()).decode_utf8_lossy();
            format!("DELETE Hello, {}!", decoded)
        });

    let hello_post = warp::post()
        .and(warp::path!("hello"))
        .and(warp::body::json())
        .map(|body: Body| format!("POST JSON Hello, {} {}!", body.first_name, body.last_name));

    let hello_post_form = warp::post()
        .and(warp::path!("hello-form"))
        .and(warp::body::form())
        .map(|body: Body| format!("POST FORM Hello, {} {}!", body.first_name, body.last_name));

    let routes = hello_get.or(hello_delete).or(hello_post).or(hello_post_form);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8000))
        .await;
}