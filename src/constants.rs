pub mod urls {
    pub const BASE_SYNC_URL: &str = "https://open-api.alibaba.com/sync";
    pub const AUTH_TOKEN_CREATE_URL: &str = "https://open-api.alibaba.com/rest/auth/token/create";
    // pub const AUTH_TOKEN_REFRESH_URL: &str = "https://open-api.alibaba.com/rest/auth/token/refresh";
    pub const OAUTH_AUTHORIZE_URL: &str = "https://open-api.alibaba.com/oauth/authorize";
}

pub mod methods {
    pub const AUTH_TOKEN_CREATE: &str = "/auth/token/create";
    pub const AUTH_TOKEN_REFRESH: &str = "/auth/token/refresh";
    pub const ALIBABA_ICBU_PRODUCT_GROUP_GET: &str = "alibaba.icbu.product.group.get";
    pub const ALIBABA_ICBU_PHOTOBANK_GROUP_LIST: &str = "alibaba.icbu.photobank.group.list";
    pub const ALIBABA_ICBU_CATEGORY_GET_NEW: &str = "alibaba.icbu.category.get.new";
    pub const ALIBABA_ICBU_CATEGORY_ATTRIBUTE_GET: &str = "alibaba.icbu.category.attribute.get";
    pub const ALIBABA_ICBU_PRODUCT_COUNTRY_GETCOUNTRYLIST: &str =
        "alibaba.icbu.product.country.getcountrylist";
}

/// caches
///
/// seconds
pub mod caches {
    pub const FIVE_MINUTE_IN_SECONDS: u64 = 300;
    // pub const ONE_HOUR_IN_SECONDS: u64 = 3600;
    // pub const HALF_DAY_IN_SECONDS: u64 = 43200;
    // pub const ONE_DAY_IN_SECONDS: u64 = 86400;
}

pub mod keys {
    pub const ACCESS_TOKEN: &str = "iop:client:access_token";
}
