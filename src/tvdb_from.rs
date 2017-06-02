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

use hyper_native_tls::NativeTlsClient;
use hyper::client::{Client, RedirectPolicy};
use hyper::net::HttpsConnector;
use hyper::header::{UserAgent, ContentType, Authorization, Bearer};

use std::io::Read;

use tvdb_errors::*;

pub const BASE_URL: &str = "https://api.thetvdb.com";

pub fn mk_client() -> Result<Client> {
    // Create a client.
    let tls_client = NativeTlsClient::new()?;
    let https_connector = HttpsConnector::new(tls_client);
    let mut hyper_client = Client::with_connector(https_connector);

    // client opts
    hyper_client.set_redirect_policy(RedirectPolicy::FollowAll);

    // ret
    Ok(hyper_client)
}

#[derive(Deserialize, Debug, Clone)]
pub struct TvdbAuthToken {
    #[serde(rename = "token")]
    auth_token: String,
}

impl TvdbAuthToken {
    pub fn from_key(api_key: &str) -> Result<Self> {
        let client = mk_client()?;

        let url = String::from(BASE_URL) + "/login";
        let content_type = "application/json".parse().map_err(|_| "invalid mime")?;

        let post_body = String::from(r###"{"apikey":"API_KEY"}"###);
        let post_body = post_body.replace("API_KEY", api_key);

        // Sending a POST request to get a JWT token
        let mut resp = client.post(&url)
            .header(UserAgent("tvdb-v2-rs-rs/0.1".into()))
            .header(ContentType(content_type))
            .body(post_body.as_bytes())
            .send()?;

        // Read the Response.
        let mut bytes = Vec::with_capacity(8 * 1024);
        resp.read_to_end(&mut bytes)?;

        // Deserialize
        let auth_token  = serde_json::from_slice(&*bytes)?;
        Ok(auth_token)
    }

    pub fn get_auth_token(&self) -> &str {
        &self.auth_token
    }
}

pub trait TvdbFrom: Sized + DeserializeOwned {
    // url=id by default
    fn url_from_id(id: &str) -> String {
        String::from(id)
    }

    fn bytes_from_url(url: &str, auth_token: &str) -> Result<Vec<u8>> {
        let client = mk_client()?;

        let bearer = Bearer {
            token: auth_token.to_owned(),
        };

        // Creating an outgoing request.
        let mut resp = client.get(url)
            .header(UserAgent("tvdb-v2-rs-rs/0.1".into()))
            .header(Authorization(bearer))
            .send()?;

        // Read the Response.
        let mut buf = Vec::with_capacity(64 * 1024);
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
