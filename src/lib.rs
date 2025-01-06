use chrono::Utc;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;
use std::collections::HashMap;
use urlencoding::encode;

type HmacSha256 = Hmac<Sha256>;

pub struct IopClient {
    appid: String,
    app_secret: String,
}

impl IopClient {
    pub fn new(appid: String, app_secret: String) -> Self {
        IopClient { appid, app_secret }
    }

    // 获取重定向url
    pub fn get_redirect_url(&self, redirect_uri: String, state: Option<String>) -> String {
        let target_url = "https://open-api.alibaba.com/oauth/authorize".to_string();
        let redirect_url = encode(&redirect_uri).to_string();

        format!(
            "{}?response_type=code&force_auth=true&redirect_uri={}&client_id={}&state={}",
            target_url,
            redirect_url,
            self.appid,
            state.unwrap_or("".to_string())
        )
    }

    // [GenerateAccessToken]<https://open.alibaba.com/doc/api.htm?spm=a2o9m.11193494.0.0.24813a3aBw28pU#/api?cid=2&path=/auth/token/create&methodType=GET/POST>
    pub async fn generate_access_token(
        &self,
        code: String,
    ) -> Result<AccessToken, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        let now = Utc::now().timestamp_millis().to_string();
        map.insert("app_key".to_string(), self.appid.to_string());
        map.insert("timestamp".to_string(), now);
        map.insert("sign_method".to_string(), "sha256".to_string());
        map.insert("simplify".to_string(), "true".to_string());
        map.insert("code".to_string(), code);

        let hash = self.generate_sign(Some("/auth/token/create".to_string()), map.clone());
        let url = self.generate_url(
            "https://open-api.alibaba.com/rest/auth/token/create".to_string(),
            map.clone(),
            hash,
        );

        let data = match reqwest::get(url).await?.json().await {
            Ok(res) => res,
            Err(e) => {
                return Err(Box::new(e));
            }
        };

        Ok(data)
    }

    fn generate_url(
        &self,
        base_url: String,
        params: HashMap<String, String>,
        sign: String,
    ) -> String {
        let mut url = String::from(base_url);
        let mut first = true;

        for (key, value) in params {
            if first {
                url.push_str(&format!("?{}={}", key, value));
                first = false;
            } else {
                url.push_str(&format!("&{}={}", key, value));
            }
        }

        url.push_str(&format!("&sign={}", sign));

        url
    }

    // [拼接参数名与参数值]<https://open.alibaba.com/doc/doc.htm?spm=a2o9m.11193535.0.0.55fb2f04MHBYoD&docId=107343&docType=1#/?docId=134>
    fn generate_sign(&self, method: Option<String>, payload: HashMap<String, String>) -> String {
        let mut sorted_vec: Vec<(&String, &String)> = payload.iter().collect();
        sorted_vec.sort_by(|a, b| a.0.cmp(b.0));

        let mut concatenated = match method {
            Some(m) => String::from(m),
            None => String::new(),
        };
        for (key, value) in sorted_vec {
            concatenated.push_str(key);
            concatenated.push_str(value);
        }

        self.generate_hmac_sha256(concatenated.as_bytes())
    }

    fn generate_hmac_sha256(&self, data: &[u8]) -> String {
        let app_secret = self.app_secret.clone();
        let mut mac = HmacSha256::new_from_slice(app_secret.as_bytes())
            .expect("HMAC can take key of any size");

        mac.update(data);
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        format!("{:X}", code_bytes)
    }
}

#[derive(Deserialize, Debug)]
pub struct CountryUserInfo {
    #[serde(rename = "aliId")]
    pub ali_id: String,
    #[serde(rename = "loginId")]
    pub login_id: String,
    pub user_id: String,
    pub seller_id: String,
}

#[derive(Deserialize, Debug)]
pub struct AccessToken {
    pub access_token: String,
    pub country: String,
    pub refresh_token: String,
    pub account_platform: String,
    pub refresh_expires_in: i32,
    pub country_user_info: CountryUserInfo,
    pub expires_in: i32,
    pub account: String,
    pub code: String,
    pub request_id: String,
    pub _trace_id_: String,
}
