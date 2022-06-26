/*
    This file is a part of tvdb-v2-rs.

    Copyright (C) 2017 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/MoSal/tvdb-v2-rs

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

use serde_json;
use serde::Deserialize;
use serde::de::DeserializeOwned;

use isahc::{HttpClient, HttpClientBuilder};
use isahc::config::{Configurable, RedirectPolicy};
use isahc::Request;
use isahc::http::header;
use once_cell::sync::OnceCell;
use futures_lite::AsyncReadExt;

use crate::tvdb_errors::*;

fn static_client() -> Result<&'static HttpClient> {
    static CELL: OnceCell<HttpClient> = OnceCell::new();
    let mk_client = || {
        HttpClientBuilder::new()
            .redirect_policy(RedirectPolicy::Follow)
            .auto_referer()
            .automatic_decompression(true)
            .build()
    };
    Ok(CELL.get_or_try_init(mk_client)?)
}

fn static_auth_token() -> Result<&'static str> {
    static CELL: OnceCell<String> = OnceCell::new();
    let client = static_client()?;
    let init_fn = || -> Result<String> {
        // Only use for deserialisation
        #[derive(Deserialize)]
        struct TvdbAuthToken {
            token: String,
        }

        let url = String::from(crate::BASE_URL) + "/login";
        let post_body = String::from(r###"{"apikey":"API_KEY"}"###);
        let post_body = post_body.replace("API_KEY", crate::API_KEY);

        // Sending a POST request to get a JWT token
        let req = Request::post(&url)
            .header(header::USER_AGENT, crate::USER_AGENT)
            .header(header::CONTENT_TYPE, "application/json")
            .body(post_body)?;
        let mut resp = client.send(req)?;


        // check status
        let http_status = resp.status();
        if http_status.is_client_error() || http_status.is_server_error() {
            Err(format!("response error: {} ({})", http_status.as_u16(), http_status.as_str()))?;
        }

        // Read the Response.
        let mut bytes = Vec::with_capacity(8 * 1024);
        std::io::Read::read_to_end(resp.body_mut(), &mut bytes)?;

        // Deserialize
        let auth_token : TvdbAuthToken  = serde_json::from_slice(&*bytes)?;
        Ok(auth_token.token)
    };
    Ok(CELL.get_or_try_init(init_fn)?)
}


#[async_trait::async_trait]
pub(crate) trait TvdbFromMulti: TvdbFrom + Send + 'static {
    type Element : Clone;

    fn num_pages(&self) -> usize;

    fn get_data(&self) -> &[Self::Element];
    fn get_data_mut(&mut self) -> &mut Vec<Self::Element>;

    async fn from_id_multi<S: ToString + Send>(id: S) -> Result<Self> {
        let url = Self::url_from_id(&*id.to_string());

        let mut initial_self = <Self as TvdbFrom>::from_id(id).await?;
        let pages = initial_self.num_pages();

        if pages > 1 {
            // with_threads() is provided by the pipeliner::Pipeline trait
            // TODO: Use inclusive ranges when they are stable
            let more_pages = (2..pages+1)
                .map(move |page| url.clone() + &format!("?page={}", page))
                .map(move |page_url| Self::from_owned_url(page_url));

            for page_res in more_pages {
                let page = page_res.await?;
                initial_self.get_data_mut().extend_from_slice(page.get_data());
            }
        }

        Ok(initial_self)
    }
}

#[async_trait::async_trait]
pub(crate) trait TvdbFrom: Sized + DeserializeOwned {
    // url=id by default
    fn url_from_id(id: &str) -> String {
        String::from(id)
    }

    async fn bytes_from_url(url: &str) -> Result<Vec<u8>> {
        //tvdb_net::bytes_from_url(url, auth_token)
        let client = static_client()?;
        let auth_token = static_auth_token()?;

        // Creating an outgoing request.
        let req = Request::get(url)
            .header(header::USER_AGENT, crate::USER_AGENT)
            .header(header::ACCEPT, crate::ACCEPT_API_VERSION)
            .header(header::AUTHORIZATION, format!("Bearer {}", auth_token))
            .body(())?;
        let mut resp = client.send_async(req).await?;

        // check status
        let http_status = resp.status();
        if http_status.is_client_error() || http_status.is_server_error() {
            Err(format!("response error: {} ({})", http_status.as_u16(), http_status.as_str()))?;
        }

        // Read the Response.
        let mut buf = Vec::with_capacity(256 * 1024);
        resp.body_mut().read_to_end(&mut buf).await?;
        Ok(buf)
    }

    async fn bytes_from_id(id: &str) -> Result<Vec<u8>> {
        Self::bytes_from_url(&Self::url_from_id(id)).await
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        // Deserialize
        //eprintln!("========\n{}\n==========", String::from_utf8_lossy(bytes));
        let info = serde_json::from_slice(bytes)?;
        Ok(info)
    }

    async fn from_url(url: &str) -> Result<Self> {
        let bytes = Self::bytes_from_url(url).await?;
        Self::from_bytes(&*bytes)
    }

    // for multi
    async fn from_owned_url(url: String) -> Result<Self> {
        Self::from_url(&url).await
    }

    async fn from_id<S: ToString + Send>(id: S) -> Result<Self> {
        let id = id.to_string();
        Self::from_url(&Self::url_from_id(&id)).await
    }
}
