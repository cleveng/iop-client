use chrono::Utc;
use deadpool_redis::{redis::cmd, Runtime};
use hmac::{Hmac, Mac};
use log::info;
use reqwest::Client;
use sha2::Sha256;
use std::collections::HashMap;
use urlencoding::encode;

mod model;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct IopClient {
    appid: String,
    app_secret: String,
    pool: deadpool_redis::Pool,
}

impl IopClient {
    pub async fn new(
        appid: String,
        app_secret: String,
        redis_addr: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let cfg = deadpool_redis::Config::from_url(redis_addr);
        let pool = match cfg.create_pool(Some(Runtime::Tokio1)) {
            Ok(pool) => pool,
            Err(err) => {
                panic!("Failed to create redis pool: {err}")
            }
        };

        match pool.get().await {
            Ok(mut conn) => {
                match cmd("SETEX")
                    .arg("PING")
                    .arg(5)
                    .arg("pong")
                    .query_async::<()>(&mut conn)
                    .await
                {
                    Ok(_) => {
                        info!("Redis connected");
                    }
                    Err(err) => {
                        panic!("Failed to connect to redis: {err}")
                    }
                }
            }
            Err(err) => {
                panic!("Failed to connect to redis: {err}")
            }
        };

        Ok(IopClient {
            appid,
            app_secret,
            pool,
        })
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
    ) -> Result<model::AccessToken, Box<dyn std::error::Error>> {
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

        let client = Client::new();
        let response = client.get(&url).send().await?;
        let at: model::AccessToken = response.json::<model::AccessToken>().await?;

        let key = format!("iop:client:{}:token", self.appid);
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
