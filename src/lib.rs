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

const USER_AGENT: &str = "tvdb-v2-rs/0.1";
const BASE_URL: &str = "https://api.thetvdb.com";
const ACCEPT_API_VERSION: &str = "application/vnd.thetvdb.v3";
const API_KEY: &str = "0629B785CE550C8D";

mod tvdb_errors;
mod tvdb_from;
pub mod tvdb_api;
