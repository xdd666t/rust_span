#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod web;
mod data;
mod database;

fn main() {
    web::main();
}