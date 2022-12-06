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
fn generate(n: Form<usize>) -> Template {
    let n = n.into_inner();
    let mut out: Vec<Vec<usize>> = vec![];
    let a = (1..=n).collect();
    gen(n, a, &mut out);
    Template::render(
        "response",
        context! {
            n: n,
            output: out,
        },
    )
}

fn gen(k: usize, a: Vec<usize>, out: &mut Vec<Vec<usize>>) {
    if k == 1 {
        out.push(a.clone());
        return;
    }
    gen(k - 1, a.clone(), out);
    for i in 0..(k - 1) {
        let b = swap(i, k - 1, a.clone());
        gen(k - 1, b, out);
    }
}

fn swap(i: usize, j: usize, mut b: Vec<usize>) -> Vec<usize> {
    let t = b[i];
    b[i] = b[j];
    b[j] = t;
    b
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, generate])
        .attach(Template::fairing())
}
