use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    constants::{methods, urls},
    IopClient,
};

#[derive(Serialize, Deserialize, Debug)]
struct ProductCountryGetCountryListResponse {
    #[serde(rename = "alibaba_icbu_product_country_getcountrylist_response")]
    response: ProductCountryGetCountryList,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProductCountryGetCountryList {
    request_id: Option<String>,
    _trace_id_: Option<String>,
    biz_success: bool,
    trace_id: Option<String>,
    data: ProductCountryDto,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductCountryDto {
    #[serde(rename = "continent_d_t_o")]
    pub items: Vec<ProductCountryItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductCountryItem {
    pub continent_name: String,
    pub continent_code: String,

    #[serde(rename = "country_list")]
    pub countries: CountryList,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountryList {
    #[serde(rename = "country_d_t_o")]
    pub data: Vec<CountryItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountryItem {
    pub country_code: String,
    pub country_name: String,
}

impl IopClient {
    /// 国际站获取商品国家列表
    ///
    /// [官方文档](https://open.alibaba.com/doc/api.htm?spm=a2o9m.11193494.0.0.22023a3a2ZhCGD#/api?cid=20966&path=alibaba.icbu.product.country.getcountrylist&methodType=GET/POST)
    ///
    /// Retrieves a list of product countries available from the Alibaba ICBU API.
    ///
    /// This function constructs the necessary request parameters, generates a signature,
    /// and sends a request to the Alibaba API to obtain a list of product countries.
    /// The request includes a language option, though it is currently unused.
    ///
    /// # Arguments
    ///
    /// * `_language` - An optional language parameter that can be provided for localization,
    ///   though it is not currently utilized in the request.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `PhotoAlbumGroup` models if successful, or an error
    /// if the process fails. The function logs the constructed parameters, the request URL,
    /// and the response result for debugging purposes.
    pub async fn list_product_countries(
        &self,
        _language: Option<String>,
    ) -> Result<ProductCountryDto, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("country_request".to_string(), "{}".to_string());
        map.insert(
            "method".to_string(),
            methods::ALIBABA_ICBU_PRODUCT_COUNTRY_GETCOUNTRYLIST.to_string(),
        );
        let params = self.build_request_params(map).await;

        let hash = self.generate_sign(None, params.clone());
        let url = self.generate_url(urls::BASE_SYNC_URL.to_string(), params.clone(), hash);
        info!("--------list_product_countries-------- url: {:#?}", url);

        let response = self.client.get(&url).send().await?;
        let result = response
            .json::<ProductCountryGetCountryListResponse>()
            .await?;

        Ok(result.response.data)
    }
}
