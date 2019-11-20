/*
    This file is a part of tvdb-v2-rs.

    Copyright (C) 2017 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/MoSal/tvdb-v2-rs

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

use serde_json;
use reqwest;

use std::{fmt, result};

pub type Result<T> = result::Result<T, TvdbError>;

#[derive(Debug)]
pub enum TvdbError {
    StdIO(::std::io::Error),
    SystemTime(::std::time::SystemTimeError),
    SerdeJson(serde_json::Error),
    Reqwest(reqwest::Error),
    Other(String),
}

impl fmt::Display for TvdbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TvdbError::StdIO(ref e) => write!(f, "IO Error: {}", e),
            TvdbError::SystemTime(ref e) => write!(f, "System Time Error: {}", e),
            TvdbError::SerdeJson(ref e) => write!(f, "Deserialization Error: {}", e),
            TvdbError::Reqwest(ref e) => write!(f, "Reqwest Error: {}", e),
            TvdbError::Other(ref e) => write!(f, "Error: {}", e),
        }
    }
}

impl From<::std::io::Error> for TvdbError {
    fn from(e: ::std::io::Error) -> Self {
        TvdbError::StdIO(e)
    }
}

impl From<::std::time::SystemTimeError> for TvdbError {
    fn from(e: ::std::time::SystemTimeError) -> Self {
        TvdbError::SystemTime(e)
    }
}

impl From<serde_json::Error> for TvdbError {
    fn from(e: serde_json::Error) -> Self {
        TvdbError::SerdeJson(e)
    }
}

impl From<reqwest::Error> for TvdbError {
    fn from(e: reqwest::Error) -> Self {
        TvdbError::Reqwest(e)
    }
}

impl<'a> From<&'a str> for TvdbError {
    fn from(e: &str) -> Self {
        TvdbError::Other(e.into())
    }
}

impl From<String> for TvdbError {
    fn from(e: String) -> Self {
        TvdbError::Other(e)
    }
}
