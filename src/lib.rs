/*  artifact: the requirements tracking tool made for developers
 * Copyright (C) 2017  Garrett Berg <@vitiral, vitiral@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the Lesser GNU General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the Lesser GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 * */

//! artifact library
// need for clippy lints
#![allow(unknown_lints)]
#![allow(zero_ptr)]
#![recursion_limit = "1024"]

// # general crates
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate fern;
extern crate itertools;

// # core crates
extern crate regex;
extern crate strfmt;
extern crate time;
extern crate rustc_serialize;
extern crate difference;
extern crate unicode_segmentation;
extern crate unicode_width;

// # cmdline crates
extern crate clap;
extern crate ansi_term;
extern crate tabwriter;
extern crate tar;

// # server crates
#[cfg(feature="server")]
#[macro_use]
extern crate nickel;
#[cfg(feature="server")]
extern crate jsonrpc_core;
#[cfg(feature="server")]
extern crate tempdir;

// # test tracking crates
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

// serialization
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate toml;

// "core" modules
pub mod dev_prefix;
pub mod types;
pub mod user;
pub mod logging;
pub mod export;
pub mod utils;
pub mod security;

// command line modules
pub mod ui;
pub mod cmd;

#[cfg(test)]
pub mod test_data;

// server modules
#[cfg(feature="server")]
pub mod api;
// test tracking modules
#[cfg(feature="server")]
pub mod schema;
#[cfg(feature="server")]
pub mod models;

pub use types::*;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", database_url))
}

use models::*;
pub fn store_test<'a, 'b>(conn: &'a PgConnection, name: &'b str) -> Result<serde_json::Value> {
	use schema::test_name;
	
	let new_test_name = NewTestName {
		name: name,
	};
	
	Ok(diesel::insert(&new_test_name).into(test_name::table)
		.get_result(conn)
		.expect("Error saving new test"))
		
}
