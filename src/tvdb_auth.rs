/*
    This file is a part of tvdb-v2-rs.

    Copyright (C) 2017 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/MoSal/tvdb-v2-rs

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

use serde_json;
use reqwest::Client;
use reqwest::header::{UserAgent, ContentType};

use std::io::Read;

use tvdb_errors::*;
use {BASE_URL, USER_AGENT};

pub fn auth_token<S: ToString>(api_key: S) -> Result<String> {
    // Only use for deserialisation
    #[derive(Deserialize)]
    struct TvdbAuthToken {
        token: String,
    }

    let api_key = api_key.to_string();

    let client = Client::builder().build()?;
    let url = String::from(BASE_URL) + "/login";

    let content_type = "application/json".parse().map_err(|_| "invalid mime")?;

    let post_body = String::from(r###"{"apikey":"API_KEY"}"###);
    let post_body = post_body.replace("API_KEY", &api_key);

    // Sending a POST request to get a JWT token
    let mut resp = client.post(&url)
        .header(UserAgent::new(USER_AGENT))
        .header(ContentType(content_type))
        .body(post_body)
        .send()?;

    // Read the Response.
    let mut bytes = Vec::with_capacity(8 * 1024);
    resp.read_to_end(&mut bytes)?;

    // Deserialize
    let auth_token : TvdbAuthToken  = serde_json::from_slice(&*bytes)?;
    Ok(auth_token.token)
}
