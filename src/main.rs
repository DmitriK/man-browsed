extern crate clap;
extern crate hyper;

mod landing;

use hyper::server::{Server, Request, Response};

fn manhandle(req: Request, res: Response) {
    use hyper::uri::RequestUri::*;
    match req.uri {
        AbsolutePath(mut s) => {
            let offset = s.rfind("?p=").unwrap_or(s.len() - 1) + 1;
            let term: String = s.drain(offset..).collect();
            println!("{:?}", term);
            match term.as_ref() {
                "" => {res.send(landing::HTML).unwrap();}
                _ => {res.send(&gen_man_html(&term).into_bytes()).unwrap();}
            }
        }
        _ => {
            res.send(b"Error: Could not understand request").unwrap();
        }
    }
}

fn gen_man_html(page: &str) -> String {
    use std::process::Command;
    let html = Command::new("man")
        .arg("-Thtml")
        .arg(page)
        .output()
        .expect("failed to execute process");

    String::from_utf8_lossy(&html.stdout).into_owned()
}

fn main() {
    use clap::{App, Arg};
    let args = App::new("man-browsed")
        .version("0.1.0")
        .about("Daemon for serving HTML man pages ")
        .arg(Arg::with_name("address")
            .short("a")
            .long("addr")
            .value_name("address")
            .help("The address to listen for connections")
            .default_value("127.0.0.1")
            .takes_value(true))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("The port to listen for connections")
            .default_value("53805")
            .takes_value(true))
        .get_matches();

    let addr = args.value_of("addr").unwrap_or("127.0.0.1");
    let port = args.value_of("port").unwrap_or("").parse::<u16>().unwrap_or(53805);

    Server::http((addr, port)).unwrap().handle(manhandle).unwrap();
}
