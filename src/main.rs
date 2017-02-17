/* man-browsed - Server for viewing HTML man pages.
 * Copyright ©2017 Dmitri Kourennyi
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.

 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.

 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

extern crate clap;
extern crate hyper;

mod landing;

use std::net::IpAddr;
use std::str::FromStr;

use hyper::header;
use hyper::server::{Server, Request, Response};

enum ResponseType {
    Landing,
    ManPage,
    ErrPage,
    OpenSearch,
}

struct ManResponse {
    res_type: ResponseType,
    body: String,
}

fn manhandle(uri: hyper::uri::RequestUri, addr: String, port: u16) -> ManResponse {
    use hyper::uri::RequestUri::*;
    match uri {
        AbsolutePath(s) => {
            let path = s.trim_left_matches("/");
            if path == "os.xml" {
                return ManResponse {
                    res_type: ResponseType::OpenSearch,
                    body: landing::OSEARCH.replace("$addr", &addr)
                        .replace("$port", &port.to_string()),
                };
            }
            let term = path.trim_left_matches("?q=");
            if term == path || term == "" {
                ManResponse {
                    res_type: ResponseType::Landing,
                    body: landing::HTML.to_string(),
                }
            } else {
                ManResponse {
                    res_type: ResponseType::ManPage,
                    body: gen_man_html(&term),
                }
            }
        }
        _ => {
            ManResponse {
                res_type: ResponseType::ErrPage,
                body: "Error: Could not understand request".to_string(),
            }
        }
    }
}

fn gen_man_html(page: &str) -> String {
    use std::process::Command;
    let words: Vec<&str> = page.split('+').collect();
    let html = Command::new("man")
        .arg("-Thtml")
        .args(&words)
        .output()
        .expect("failed to execute process");

    String::from_utf8_lossy(&html.stdout).into_owned()
}

fn main() {
    use clap::{App, Arg};
    let args = App::new("man-browsed")
        .version("0.1.0")
        .about("Daemon for serving HTML man pages ")
        .author("Dmitri Kourennyi")
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

    let addr = args.value_of("addr").unwrap_or("127.0.0.1").to_string();
    let port = args.value_of("port").unwrap_or("").parse::<u16>().unwrap_or(53805);

    let serve = Server::http((IpAddr::from_str(&addr).unwrap(), port)).unwrap();
    let addr = addr;
    serve.handle(move |req: Request, mut res: Response| {
            let resp = manhandle(req.uri, addr.clone(), port);

            match resp.res_type {
                ResponseType::OpenSearch => {
                    let hdr = res.headers_mut();
                    hdr.set(header::ContentType("application/opensearchdescription+xml"
                        .parse()
                        .unwrap()));
                }
                _ => {
                    let hdr = res.headers_mut();
                    hdr.set(header::ContentType("text/html".parse().unwrap()));
                }
            }
            res.send(&resp.body.into_bytes()).unwrap()
        })
        .unwrap();
}
