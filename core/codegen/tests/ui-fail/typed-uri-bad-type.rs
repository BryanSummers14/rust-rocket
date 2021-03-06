#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use rocket::request::FromParam;

struct S;

impl<'a> FromParam<'a> for S {
    type Error = ();
    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> { Ok(S) }
}

#[post("/<id>")]
fn simple(id: i32) {  }

#[post("/<id>/<name>")]
fn not_uri_display(id: i32, name: S) {  }

#[post("/<id>/<name>")]
fn not_uri_display_but_unused(id: i32, name: S) {  }

#[post("/<id>/<name>")]
fn optionals(id: Option<i32>, name: Result<String, &RawStr>) {  }

use rocket::request::{Query, FromQuery};

impl<'q> FromQuery<'q> for S {
    type Error = ();
    fn from_query(query: Query<'q>) -> Result<Self, Self::Error> { Ok(S) }
}

#[post("/?<id>")]
fn simple_q(id: i32) {  }

#[post("/?<id>&<rest..>")]
fn other_q(id: usize, rest: S) {  }

#[post("/?<id>&<name>")]
fn optionals_q(id: Option<i32>, name: Result<String, &RawStr>) {  }

fn main() {
    uri!(simple: id = "hi");
    //~^ ERROR i32: rocket::http::uri::FromUriParam<rocket::http::uri::Path, &str>

    uri!(simple: "hello");
    //~^ ERROR i32: rocket::http::uri::FromUriParam<rocket::http::uri::Path, &str>

    uri!(simple: id = 239239i64);
    //~^ ERROR i32: rocket::http::uri::FromUriParam<rocket::http::uri::Path, i64>

    uri!(not_uri_display: 10, S);
    //~^ ERROR S: rocket::http::uri::FromUriParam<rocket::http::uri::Path, _>

    // This one is okay. In paths, a value _must_ be supplied.
    uri!(optionals: id = 10, name = "bob".to_string());

    uri!(optionals: id = Some(10), name = Ok("bob".into()));
    //~^ ERROR i32: rocket::http::uri::FromUriParam<rocket::http::uri::Path, std::option::Option<{integer}>>
    //~^^ ERROR String: rocket::http::uri::FromUriParam<rocket::http::uri::Path, std::result::Result<_, _>>

    uri!(simple_q: "hi");
    //~^ ERROR i32: rocket::http::uri::FromUriParam<rocket::http::uri::Query, &str>

    uri!(simple_q: id = "hi");
    //~^ ERROR i32: rocket::http::uri::FromUriParam<rocket::http::uri::Query, &str>

    uri!(other_q: 100, S);
    //~^ ERROR S: rocket::http::uri::FromUriParam<rocket::http::uri::Query, _>

    uri!(other_q: rest = S, id = 100);
    //~^ ERROR S: rocket::http::uri::FromUriParam<rocket::http::uri::Query, _>

    // This one is okay.
    uri!(optionals_q: None, Err("foo".into()));

    // For queries, we need to know the exact variant.
    uri!(optionals_q: id = 10, name = "Bob".to_string());
    //~^ ERROR Option<i32>: rocket::http::uri::FromUriParam<rocket::http::uri::Query, {integer}>
    //~^^ ERROR: Result<std::string::String, &rocket::http::RawStr>: rocket::http::uri::FromUriParam<rocket::http::uri::Query, std::string::String>
}
