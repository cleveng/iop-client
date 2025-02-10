use crate::{
    constants::{keys, methods, urls},
    IopClient,
};
use deadpool_redis::redis::cmd;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use urlencoding::encode;

#[derive(Serialize, Deserialize, Debug)]
pub struct CountryUserInfo {
    #[serde(rename = "aliId")]
    pub union_id: String,

    #[serde(rename = "loginId")]
    pub username: String,

    #[serde(rename = "user_id")]
    pub open_id: String,
    pub seller_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    pub access_token: String,
    pub refresh_token: String,
    pub refresh_expires_in: i32,
    pub expires_in: i32,
    code: String,
    request_id: Option<String>,
    _trace_id_: Option<String>,
    pub account_platform: String,
    pub country: String,

    #[serde(rename = "country_user_info")]
    pub user_info: CountryUserInfo,

    #[serde(rename = "account")]
    pub email: String,
}

impl IopClient {
    /// Constructs and returns a redirect URL for the OAuth authorization process.
    ///
    /// # Arguments
    ///
    /// * `redirect_uri` - The URI to which the response should be sent after authorization.
    /// * `state` - An optional parameter to maintain state between the request and callback.
    ///
    /// # Returns
    ///
    /// A `String` representing the complete redirect URL, including query parameters for
    /// response type, client ID, redirect URI, and state.
    pub fn get_redirect_url(&self, redirect_uri: String, state: Option<String>) -> String {
        let redirect_url = encode(&redirect_uri).to_string();

        format!(
            "{}?response_type=code&force_auth=true&redirect_uri={}&client_id={}&state={}",
            urls::OAUTH_AUTHORIZE_URL,
            redirect_url,
            self.appid,
            state.unwrap_or("".to_string())
        )
    }

    /// Generates an access token using the provided authorization code.
    /// [GenerateAccessToken]<https://open.alibaba.com/doc/api.htm?spm=a2o9m.11193494.0.0.24813a3aBw28pU#/api?cid=2&path=/auth/token/create&methodType=GET/POST>
    /// # Arguments
    ///
    /// * `code` - The authorization code received after user authorization.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `AccessToken` model if successful, or an error if the process fails.
    ///
    /// This function constructs the necessary request parameters, generates a signature,
    /// and sends a request to the Alibaba API to obtain an access token. The token is
    /// then stored in Redis for future access.
    pub async fn generate_access_token(
        &self,
        code: String,
    ) -> Result<AccessToken, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("code".to_string(), code);

        let params = self.build_request_params(map).await;

        let hash = self.generate_sign(Some(methods::AUTH_TOKEN_CREATE.to_string()), params.clone());
        let url = self.generate_url(
            urls::AUTH_TOKEN_CREATE_URL.to_string(),
            params.clone(),
            hash,
        );
        info!("--------generate_access_token-------- url: {:#?}", url);

        let response = match self.client.get(&url).send().await {
            Ok(response) => response,
            Err(err) => {
                error!("Failed to get access token response, {err}");
                return Err(err.into());
            }
        };

        let at = match response.json::<AccessToken>().await {
            Ok(at) => at,
            Err(err) => {
                error!("Failed to get access token json, {err}");
                return Err(err.into());
            }
        };

        let key = format!("{}:{}", keys::ACCESS_TOKEN, self.appid);
        if let Ok(mut conn) = self.pool.get().await {
            let _: () = cmd("SET")
                .arg(&key)
                .arg(serde_json::to_string(&at).unwrap())
                .query_async::<()>(&mut conn)
                .await
                .unwrap();
        }

        Ok(at)
    }

    /// Retrieves the access token associated with the client from Redis.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `AccessToken` model if the token exists, or an error if the token
    /// is not found in Redis.
    pub async fn get_access_token(&self) -> Result<AccessToken, Box<dyn std::error::Error>> {
        let key = format!("{}:{}", keys::ACCESS_TOKEN, self.appid);
        if let Ok(mut conn) = self.pool.get().await {
            let at: Option<String> = cmd("GET").arg(&key).query_async(&mut conn).await.unwrap();
            if let Some(at) = at {
                return Ok(serde_json::from_str(&at).unwrap());
            }
        }

        Err("Access token not found".into())
    }

    /// Refreshes the access token using the stored refresh token.
    ///
    /// This function retrieves the current access token and uses its refresh token to
    /// request a new access token from the Alibaba API. The newly obtained access token
    /// is then stored in Redis for future access.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `AccessToken` model if successful, or an error if the process fails.
    ///
    /// # Errors
    ///
    /// Returns an error if retrieving the current access token fails, or if the request
    /// to refresh the token is unsuccessful.
    pub async fn refresh_access_token(&self) -> Result<AccessToken, Box<dyn std::error::Error>> {
        let token = match self.get_access_token().await {
            Ok(token) => token,
            Err(err) => {
                error!("Failed to get access token, {err}");
                return Err(err.into());
            }
        };

        let mut map = HashMap::new();
        map.insert("refresh_token".to_string(), token.refresh_token.clone());
        map.insert(
            "method".to_string(),
            methods::AUTH_TOKEN_REFRESH.to_string(),
        );
        let params = self.build_request_params(map).await;

        let hash = self.generate_sign(None, params.clone());
        let url = self.generate_url(urls::BASE_SYNC_URL.to_string(), params.clone(), hash);
        info!("--------refresh_access_token-------- url: {:#?}", url);

        let response = match self.client.get(&url).send().await {
            Ok(response) => response,
            Err(err) => {
                error!("Failed to refresh access token, {err}");
                return Err(err.into());
            }
        };
        let at = match response.json::<AccessToken>().await {
            Ok(at) => at,
            Err(err) => {
                error!("Failed to refresh access token, {err}");
                return Err(err.into());
            }
        };

        let key = format!("{}:{}", keys::ACCESS_TOKEN, self.appid);
        if let Ok(mut conn) = self.pool.get().await {
            let _: () = cmd("SET")
                .arg(&key)
                .arg(serde_json::to_string(&at).unwrap())
                .query_async::<()>(&mut conn)
                .await
                .unwrap();
        }

        // TODO: return at
        Err("Failed to refresh access token".into())
    }
}
