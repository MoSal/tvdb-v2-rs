/*
    This file is a part of tvdb-v2-rs.

    Copyright (C) 2017 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/MoSal/tvdb-v2-rs

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate native_tls;
extern crate hyper_native_tls;

mod tvdb_errors;
pub mod tvdb_from;
pub mod tvdb_api;
