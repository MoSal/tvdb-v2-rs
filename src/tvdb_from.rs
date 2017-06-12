/*
    This file is a part of tvdb-v2-rs.

    Copyright (C) 2017 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/MoSal/tvdb-v2-rs

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

use serde_json;
use serde::de::DeserializeOwned;

use tvdb_net;
use tvdb_errors::*;

pub trait TvdbFrom: Sized + DeserializeOwned {
    // url=id by default
    fn url_from_id(id: &str) -> String {
        String::from(id)
    }

    fn bytes_from_url(url: &str, auth_token: &str) -> Result<Vec<u8>> {
        tvdb_net::bytes_from_url(url, auth_token)
    }

    fn bytes_from_id(id: &str, auth_token: &str) -> Result<Vec<u8>> {
        Self::bytes_from_url(&Self::url_from_id(id), auth_token)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        // Deserialize
        let info = serde_json::from_slice(bytes)?;
        Ok(info)
    }

    fn from_url(url: &str, auth_token: &str) -> Result<Self> {
        let bytes = Self::bytes_from_url(url, auth_token)?;
        Self::from_bytes(&*bytes)
    }

    fn from_id<S: ToString>(id: S, auth_token: &str) -> Result<Self> {
        let id = id.to_string();
        Self::from_url(&Self::url_from_id(&id), auth_token)
    }
}
