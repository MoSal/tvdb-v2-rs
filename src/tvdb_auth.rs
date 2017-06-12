use serde_json;
use hyper::header::{UserAgent, ContentType};

use std::io::Read;

use tvdb_net;
use tvdb_errors::*;
use {BASE_URL, USER_AGENT};

pub fn auth_token(api_key: &str) -> Result<String> {
    // Only use for deserialisation
    #[derive(Deserialize)]
    struct TvdbAuthToken {
        token: String,
    }

    let client = tvdb_net::mk_client()?;
    let url = String::from(BASE_URL) + "/login";

    let content_type = "application/json".parse().map_err(|_| "invalid mime")?;

    let post_body = String::from(r###"{"apikey":"API_KEY"}"###);
    let post_body = post_body.replace("API_KEY", api_key);

    // Sending a POST request to get a JWT token
    let mut resp = client.post(&url)
        .header(UserAgent(USER_AGENT.into()))
        .header(ContentType(content_type))
        .body(post_body.as_bytes())
        .send()?;

    // Read the Response.
    let mut bytes = Vec::with_capacity(8 * 1024);
    resp.read_to_end(&mut bytes)?;

    // Deserialize
    let auth_token : TvdbAuthToken  = serde_json::from_slice(&*bytes)?;
    Ok(auth_token.token)
}