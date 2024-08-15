use rocket::{get, launch, routes};
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/test/<name>")]
fn test(name: &str) -> Template {
    Template::render("test", context! {name: name})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Template::fairing())
    .mount("/", routes![index, test])
}
