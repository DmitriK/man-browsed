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
extern crate iron;
#[macro_use]
extern crate mime;

mod landing;

use iron::prelude::*;
use iron::status;

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

fn manhandle(url: &iron::Url, addr: String, port: u16) -> ManResponse {
    if url.clone().into_generic_url().as_ref() == "os.xml" {
        return ManResponse {
            res_type: ResponseType::OpenSearch,
            body: landing::OSEARCH.replace("$addr", &addr)
                .replace("$port", &port.to_string()),
        };
    }

    match url.query() {
        Some(q) => {
            let term = q.trim_left_matches("q=");
            ManResponse {
                res_type: ResponseType::ManPage,
                body: gen_man_html(&term),
            }
        }
        None => {
            ManResponse {
                res_type: ResponseType::Landing,
                body: landing::HTML.to_string(),
            }
        }
    }
}

fn gen_man_html(page: &str) -> String {
    use std::io::{Read, Write};
    use std::process::{Command, Stdio};
    let words: Vec<&str> = page.split('+').collect();

    /*let mandoc = Command::new("mandoc")
        .arg("-Thtml")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();

    match mandoc {
        Ok(mandoc) => {
            let manout = Command::new("man").args(&words).stdout(Stdio::piped()).output().unwrap();

            mandoc.stdin.unwrap().write_all(&manout.stdout).unwrap();

            let mut html: String = "".to_owned();
            mandoc.stdout.unwrap().read_to_string(&mut html).unwrap();

            html
        }
        Err(_) => {*/
    let html = Command::new("man")
        .arg("-Thtml")
        .args(&words)
        .output()
        .expect("failed to execute process");

    String::from_utf8_lossy(&html.stdout).into_owned()
    /*}
    }*/
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

    let addr2 = addr.clone();

    Iron::new(move |req: &mut Request| {
            let resp = manhandle(&req.url, addr.clone(), port);

            match resp.res_type {
                ResponseType::OpenSearch => {
                    let ct = "application/opensearchdescription+xml".parse::<mime::Mime>().unwrap();
                    Ok(Response::with((ct, status::Ok, resp.body)))
                }
                _ => {
                    let ct = mime!(Text / Html);
                    Ok(Response::with((ct, status::Ok, resp.body)))
                }
            }
        })
        .http((&*addr2, port))
        .unwrap();
}
