extern crate iron;

use iron::prelude::*;

fn gen_man_html(page: &str) -> String {
    println!("{:?}", page);

    let html = std::process::Command::new("man")
        .arg("-Thtml")
        .arg(page)
        .output()
        .expect("failed to execute process");

    String::from_utf8_lossy(&html.stdout).into_owned()
}

fn hello_world(req: &mut Request) -> IronResult<Response> {
    let term = req.url.query().unwrap();
    let html = gen_man_html(term);
    let mime: iron::mime::Mime = "text/html".parse().unwrap();
    Ok(Response::with((iron::status::Ok, mime, html)))
}

fn main() {
    let chain = Chain::new(hello_world);
    Iron::new(chain).http("localhost:3000").unwrap();
}
