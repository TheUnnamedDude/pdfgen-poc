#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

extern crate handlebars;
extern crate uuid;

use rocket::response::NamedFile;
use rocket_contrib::{Json, Value};

use handlebars::Handlebars;
use uuid::Uuid;

use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

#[post("/<pdf_type>", format = "application/json", data = "<data>")]
fn genpdf(pdf_type: String, data: Json<Value>) -> Option<NamedFile> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("legeerklaering", "templates/legeerklaering.hbs").unwrap();

    let uuid = Uuid::new_v4();
    let html_file = PathBuf::from("out").join(format!("{}.html", uuid));
    handlebars.render_to_write(pdf_type.as_str(), &data.0, &mut File::create(&html_file).unwrap());
    let out_file = PathBuf::from("out").join(format!("{}.pdf", uuid));

    let output = Command::new("wkhtmltopdf")
        .arg(&html_file)
        .arg(&out_file)
        .status()
        .expect("Failed to execute wkhtmltopdf");

    NamedFile::open(out_file).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![genpdf]).launch();
}
