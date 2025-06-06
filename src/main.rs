use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use yew::prelude::*;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("W-wequest numbew: {counter} OwO") // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    yew::Renderer::<App2>::new().render();

    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[function_component(App2)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

/*
fn main() {
}
*/
