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
extern crate regex;
extern crate router;

mod landing;

use iron::{Iron, Request, Response, IronResult};
use iron::status;
use regex::Regex;
use router::Router;

fn manhandle(req: &mut Request) -> IronResult<Response> {
    let ct = mime!(Text / Html);
    match req.url.query() {
        Some(q) => {
            let term = q.trim_left_matches("q=");
            Ok(Response::with((ct, status::Ok, gen_man_html(&term))))
        }
        None => Ok(Response::with((ct, status::Ok, landing::HTML.to_string()))),
    }
}

fn gen_man_html(page: &str) -> String {
    use std::process::{Command, Stdio};
    let words: Vec<&str> = page.split('+').collect();

    let mandoc =
        Command::new("mandoc").arg("-V").stdout(Stdio::null()).stderr(Stdio::null()).status();

    match mandoc {
        Ok(_) => {
            let manout = Command::new("man")
                .arg("-w")
                .args(&words)
                .output()
                .unwrap()
                .stdout;

            let html = Command::new("mandoc")
                .arg("-Thtml")
                .arg("-O")
                .arg("man=/?q=%S+%N")
                .arg(String::from_utf8_lossy(&manout).into_owned().trim())
                .stderr(Stdio::inherit())
                .output()
                .unwrap();

            let html = String::from_utf8_lossy(&html.stdout);

            let link_finder = Regex::new(r"<b>([^ ]+)</b>\(\d\)").unwrap();
            let html = link_finder.replace_all(&html, "<a href=\"/?$2+$1\">$0</a>");

            html.into_owned()
        }
        Err(_) => {
            let html = Command::new("man")
                .arg("-Thtml")
                .args(&words)
                .output()
                .expect("failed to execute process")
                .stdout;

            let html = String::from_utf8_lossy(&html);

            let link_finder = Regex::new(r"<b>([^ ]+)</b>\(\d\)").unwrap();
            let html = link_finder.replace_all(&html, "<a href=\"/?$2+$1\">$0</a>");

            let link_finder = Regex::new(r"file://[^\s<>]+").unwrap();
            let html = link_finder.replace_all(&html, "<a href=\"$0\">$0</a>");

            html.into_owned()
        }
    }
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

    let addr = args.value_of("addr").unwrap_or("127.0.0.1");
    let port = args.value_of("port").unwrap_or("").parse::<u16>().unwrap_or(53805);

    let addr_lt = addr.to_string();

    let mut router = Router::new();
    router.get("/os.xml",
               move |_: &mut Request| {
        let ct = "application/opensearchdescription+xml".parse::<mime::Mime>().unwrap();
        Ok(Response::with((ct,
                           status::Ok,
                           landing::OSEARCH.replace("$addr", &addr_lt)
                               .replace("$port", &port.to_string()))))
    },
               "handler");

    router.get("/", manhandle, "query");

    match Iron::new(router).http((addr, port)) {
        Err(iron::error::HttpError::Io(e)) => {
            if e.kind() == std::io::ErrorKind::AddrInUse {
                println!("Something is already listening on {:}:{:}; \
                          Perhaps the service is already running?",
                         addr,
                         port);
                std::process::exit(1);
            } else {
                panic!(e);
            }
        }
        Err(e) => panic!(e),
        Ok(_) => {}
    }
}
