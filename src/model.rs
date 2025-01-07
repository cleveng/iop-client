use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CountryUserInfo {
    #[serde(rename = "aliId")]
    pub ali_id: String,
    #[serde(rename = "loginId")]
    pub login_id: String,
    pub user_id: String,
    pub seller_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    access_token: String,
    refresh_token: String,
    refresh_expires_in: i32,
    expires_in: i32,
    code: String,
    request_id: String,
    _trace_id_: String,
    pub country: String,
    pub account_platform: String,
    pub country_user_info: CountryUserInfo,
    pub account: String,
}
