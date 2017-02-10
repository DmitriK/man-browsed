extern crate hyper;

use hyper::server::{Server, Request, Response};


fn manhandle(req: Request, res: Response) {
    match req.uri {
        hyper::uri::RequestUri::AbsolutePath(mut s) => {
            let offset = s.find('?').unwrap_or(s.len() - 1) + 1;

            // Remove the range up until the β from the string
            let term: String = s.drain(offset..).collect();

            res.send(&gen_man_html(&term).into_bytes()).unwrap();
        },
        _ => {
            res.send(b"Error: Could not understand request").unwrap();
        }
    }
}

fn gen_man_html(page: &str) -> String {
    let html = std::process::Command::new("man")
        .arg("-Thtml")
        .arg(page)
        .output()
        .expect("failed to execute process");

    String::from_utf8_lossy(&html.stdout).into_owned()
}

fn main() {
    Server::http("0.0.0.0:3000").unwrap().handle(manhandle).unwrap();
}
