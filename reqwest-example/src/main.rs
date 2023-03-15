use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde()]
pub struct OpenIdConfigration {
    pub issuer: String,
    //#[serde(rename = "authorization_endpoint")]
    pub authorization_endpoint: String,
    //#[serde(rename = "token_endpoint")]
    pub token_endpoint: String,
    //#[serde(rename = "jwks_uri")]
    pub jwks_uri: String,
    pub userinfo_endpoint: String,
    pub device_authorization_endpoint: String,
    pub grant_types_supported: Vec<String>,
    pub response_types_supported: Vec<String>,
    pub code_challenge_methods_supported: Vec<String>,
    pub scopes_supported: Vec<String>,
    pub token_endpoint_auth_methods_supported: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde()]
pub struct Jwk {
    #[serde(rename = "use")]
    pub usage: String,
    pub kty: String,
    pub kid: String,
    pub alg: String,
    pub n: String,
    pub e: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde()]
pub struct Jwks {
    pub keys: Vec<Jwk>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = "https://oauth2.sigstore.dev/auth/.well-known/openid-configuration";
    let response = reqwest::get(&*request_url).await?;
    let value: OpenIdConfigration = response.json().await?;

    let jwks_uri = value.jwks_uri;
    let response = reqwest::get(&*jwks_uri).await?;
    let jwks: Jwks = response.json().await?;
    for jwk in jwks.keys {
        println!("kid: {:?}, n: {:?}", jwk.kid, jwk.n);
    }
    Ok(())
}
