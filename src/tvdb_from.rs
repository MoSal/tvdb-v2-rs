/*
    This file is a part of tvdb-v2-rs.

    Copyright (C) 2017 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/MoSal/tvdb-v2-rs

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

use pipeliner::Pipeline;

use serde_json;
use serde::de::DeserializeOwned;

use tvdb_net;
use tvdb_errors::*;

pub(crate) trait TvdbFrom: Sized + DeserializeOwned {
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
