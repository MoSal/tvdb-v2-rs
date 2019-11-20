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

use reqwest::Client;
use pipeliner::Pipeline;

use std::io::Read;

use tvdb_errors::*;

pub(crate) trait TvdbFromMulti: TvdbFrom + Send + 'static {
    type Element : Clone;

    fn num_pages(&self) -> usize;

    fn get_data(&self) -> &[Self::Element];
    fn get_data_mut(&mut self) -> &mut Vec<Self::Element>;

    fn from_id_multi<S: ToString>(id: S, auth_token: &str) -> Result<Self> {
        let url = Self::url_from_id(&*id.to_string());

        let mut initial_self = <Self as TvdbFrom>::from_id(id, auth_token)?;
        let pages = initial_self.num_pages();

        if pages > 1 {
            let auth_token = String::from(auth_token);
            // with_threads() is provided by the pipeliner::Pipeline trait
            // TODO: Use inclusive ranges when they are stable
            let more_pages = (2..pages+1)
                .map(move |page| (url.clone() + &format!("?page={}", page), auth_token.clone()))
                .with_threads(8)
                .map(|(page_url, auth_token)| Self::from_url(&page_url, &auth_token));

            for page_res in more_pages {
                let page = page_res?;
                initial_self.get_data_mut().extend_from_slice(page.get_data());
            }
        }

        Ok(initial_self)
    }
}

pub(crate) trait TvdbFrom: Sized + DeserializeOwned {
    // url=id by default
    fn url_from_id(id: &str) -> String {
        String::from(id)
    }

    fn bytes_from_url(url: &str, auth_token: &str) -> Result<Vec<u8>> {
        //tvdb_net::bytes_from_url(url, auth_token)
        let client = Client::builder().build()?;

        // Creating an outgoing request.
        let mut resp = client.get(url)
            .header("User-Agent", super::USER_AGENT)
            .header("Accept", super::ACCEPT_API_VERSION)
            .bearer_auth(auth_token)
            .send()?;

        // Read the Response.
        let mut buf = Vec::with_capacity(256 * 1024);
        resp.read_to_end(&mut buf)?;
        Ok(buf)
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
