#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
extern crate chrono;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

extern crate handlebars;
extern crate uuid;

mod wkhtmltopdf;

use chrono::prelude::*;
use chrono::{Date, DateTime};

use rocket::request::FromForm;
use rocket::State;
use rocket::response::NamedFile;
use rocket_contrib::{Json, Value};

use handlebars::{Handlebars, Helper, RenderContext, RenderError};
use uuid::Uuid;

use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use wkhtmltopdf::HtmlConverter;

fn iso_to_nor_date(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    if let Some(date_string) = h.param(0).unwrap().value().as_str() {
        let chrono_date = date_string.parse::<DateTime<Local>>().unwrap();
        rc.writer.write(chrono_date.format("%d.%m.%Y").to_string().as_bytes())?;
    }
    Ok(())
}

#[derive(FromForm)]
struct GenerationParams {
    header: Option<String>
}

#[get("/is_alive")]
fn liveness() -> &'static str {
    "I'm alive"
}

#[get("/is_ready")]
fn readyness() -> &'static str {
    "I'm readu"
}

#[post("/<pdf_type>?<params>", format = "application/json", data = "<data>")]
fn genpdf(pdf_type: String, params: GenerationParams, data: Json<Value>, html_converter: State<HtmlConverter>) -> Option<NamedFile> {
    let start_time = Instant::now();
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("iso_to_nor_date", Box::new(iso_to_nor_date));
    handlebars.register_template_file("fagmelding_header", "templates/fagmelding_header.hbs").unwrap();
    handlebars.register_template_file("fagmelding", "templates/fagmelding.hbs").unwrap();

    let uuid = Uuid::new_v4();
    let html_file = PathBuf::from("out").join(format!("{}.html", uuid));

    let header_file = if let Some(header) = params.header {
        let header_file = PathBuf::from("out").join(format!("{}_{}.html", uuid, header));
        handlebars.render_to_write(header.as_str(), &*data, &mut File::create(&header_file).unwrap());
        Some(header_file)
    } else {
        None
    };

    handlebars.render_to_write(pdf_type.as_str(), &*data, &mut File::create(&html_file).unwrap());
    let out_file = PathBuf::from("out").join(format!("{}.pdf", uuid));

    let mut args = vec![];
    if let Some(header_file) = header_file {
        args.push("--header-html".to_owned());
        args.push(header_file.to_str().unwrap().to_owned());
    }
    args.push(html_file.to_str().unwrap().to_owned());
    args.push(out_file.to_str().unwrap().to_owned());
    html_converter.run_command(args);
    println!("{}", start_time.elapsed().subsec_millis());

    NamedFile::open(out_file).ok()
}

fn main() {
    rocket::ignite().manage(HtmlConverter::new()).mount("/", routes![genpdf, liveness, readyness]).launch();
}
