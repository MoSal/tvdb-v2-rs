/*
    This file is a part of tvdb-v2-rs.

    Copyright (C) 2017 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/MoSal/tvdb-v2-rs

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

//! Rust interface for TVDB's JSON API v2
//!
//! This crate provides a native Rust interface for the JSON API v2 from [TheTVDB.com].
//!
//! [TheTVDB.com]: https://api.thetvdb.com/swagger

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate native_tls;
extern crate hyper_native_tls;

const USER_AGENT: &str = "tvdb-v2-rs/0.1";
const BASE_URL: &str = "https://api.thetvdb.com";

mod tvdb_errors;
mod tvdb_net;
mod tvdb_from;
pub mod tvdb_auth;
pub mod tvdb_api;
