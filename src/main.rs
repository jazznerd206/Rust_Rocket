#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::Request;
use rocket::response::content::Json;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/")]
fn index() -> Template {
    #[derive(Serialize)]
    struct Context {
    }
    let context = Context {
    };
    Template::render("home", context)
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json("{
        'status': 'success',
        'message': 'Hello API!'
    }")
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    print!("{}", req);
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/", routes![index, hello])
        .attach(Template::fairing())
        .launch()
    ;
}