#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

#[get("/")]
async fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("index.html").await
}

#[post("/", data = "<n>")]
fn generate(n: Form<i32>) -> Template {
    Template::render(
        "response",
        context! {
            n: n.into_inner(),
            output: vec![1],
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, generate])
        .attach(Template::fairing())
}
