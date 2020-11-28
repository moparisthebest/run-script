#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::io::{Cursor, Error};
use rocket::response::Response;
use rocket::http::{Status, ContentType};
use rocket::response::status::{BadRequest};

use std::process::{Command, Output};
use std::path::PathBuf;
use std::str::Utf8Error;

#[get("/run/<script>/<arg>")]
fn run(script: String, arg: String) -> Result<String, BadRequest<String>> {
    println!("running: {} '{}'", script, arg);

    match std::env::current_dir() {
        Ok(pwd) => match Command::new(pwd.join(script))
            .arg(arg)
            .output() {
            Ok(output) => {
                match std::str::from_utf8(&output.stdout) {
                    Ok(stdout) => {
                        let stdout = stdout.to_string();
                        if output.status.success() {
                            Ok(stdout)
                        } else {
                            Err(BadRequest(Some(stdout)))
                        }
                    }
                    Err(e) => Err(BadRequest(Some(format!("{}", e)))),
                }
            }
            Err(e) => Err(BadRequest(Some(format!("{}", e)))),
        },
        Err(e) => Err(BadRequest(Some(format!("{}", e)))),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![run]).launch();
}
