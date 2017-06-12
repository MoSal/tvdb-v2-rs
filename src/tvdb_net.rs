use hyper_native_tls::NativeTlsClient;
use hyper::client::{Client, RedirectPolicy};
use hyper::net::HttpsConnector;
use hyper::header::{UserAgent, Authorization, Bearer};

use std::io::Read;

use tvdb_errors::*;
use USER_AGENT;

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

pub fn bytes_from_url(url: &str, auth_token: &str) -> Result<Vec<u8>> {
    let client = mk_client()?;

    let bearer = Bearer {
        token: auth_token.to_owned(),
    };

    // Creating an outgoing request.
    let mut resp = client.get(url)
        .header(UserAgent(USER_AGENT.into()))
        .header(Authorization(bearer))
        .send()?;

    // Read the Response.
    let mut buf = Vec::with_capacity(64 * 1024);
    resp.read_to_end(&mut buf)?;
    Ok(buf)
}
