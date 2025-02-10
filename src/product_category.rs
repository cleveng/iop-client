use crate::constants::{methods, urls};

use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use log::info;

use crate::IopClient;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCategoryResponse {
    pub alibaba_icbu_category_get_new_response: NewCategoryGroup,
    pub request_id: Option<String>,
    pub _trace_id_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCategoryGroup {
    pub category: NewCategory,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCategory {
    pub leaf_category: bool,
    pub cn_name: Option<String>,
    pub category_id: i32,
    pub level: i32,
    pub name: String,

    #[serde(default, deserialize_with = "empty_object_as_none")]
    pub child_ids: Option<NewCategoryChildId>,

    #[serde(default, deserialize_with = "empty_object_as_none")]
    pub parent_ids: Option<NewCategoryChildId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCategoryChildId {
    pub number: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CategoryAttributeGetResponse {
    alibaba_icbu_category_attribute_get_response: CategoryAttributeGroup,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryAttributeGroup {
    pub attributes: CategoryAttributes,
    request_id: Option<String>,
    _trace_id_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryAttributes {
    pub attribute: Vec<CategoryAttribute>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryAttribute {
    pub sku_attribute: bool,
    pub show_type: String,
    pub customize_image: bool,
    pub car_model: bool,
    pub value_type: String,
    pub customize_value: bool,

    #[serde(deserialize_with = "empty_object_as_none")]
    pub attribute_values: Option<AttributeValues>,
    pub input_type: String,
    pub en_name: String,
    pub required: bool,
    pub attr_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttributeValues {
    pub attribute_value: Vec<AttributeValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttributeValue {
    pub sku_value: bool,
    pub attr_value_id: i32,
    pub en_name: String,
}

// [skip deserialize empty json object {}](https://github.com/serde-rs/serde/issues/2362)
fn empty_object_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(
        untagged,
        deny_unknown_fields,
        expecting = "object, empty object or null"
    )]
    enum Helper<T> {
        Empty {},
        Data(T),
        Null,
    }
    match Helper::deserialize(deserializer) {
        Ok(Helper::Data(data)) => Ok(Some(data)),
        Ok(_) => Ok(None),
        Err(e) => Err(e),
    }
}

impl IopClient {
    /// (新)ICBU类目树获取接口
    ///
    /// [官方文档](https://open.alibaba.com/doc/api.htm#/api?cid=20966&path=alibaba.icbu.category.get.new&methodType=GET/POST)
    ///
    ///  Get the specified category by cat_id.
    ///
    /// # Arguments
    ///
    /// * `cat_id` - The category ID.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `NewCategory` model if successful, or an error if the process fails.
    ///
    /// The function first retrieves an access token, constructs a map of parameters required
    /// for the API call, and includes the category ID and language. A signature is then generated
    /// for the request, and the request is sent to the API endpoint. Upon successful completion,
    /// the category information is returned.
    pub async fn list_product_categories(
        &self,
        cat_id: i32,
    ) -> Result<NewCategory, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("cat_id".to_string(), format!("{}", cat_id));
        map.insert(
            "method".to_string(),
            methods::ALIBABA_ICBU_CATEGORY_GET_NEW.to_string(),
        );

        let params = self.build_request_params(map).await;

        let hash = self.generate_sign(None, params.clone());
        let url = self.generate_url(urls::BASE_SYNC_URL.to_string(), params.clone(), hash);
        info!("--------list_product_categories-------- url: {:#?}", url);

        let response = self.client.get(&url).send().await?;
        let result = response.json::<NewCategoryResponse>().await?;

        Ok(result.alibaba_icbu_category_get_new_response.category)
    }

    /// 类目属性获取
    ///
    /// [官方文档](https://open.alibaba.com/doc/api.htm?spm=a2o9m.11193494.0.0.22023a3a2ZhCGD#/api?cid=20966&path=alibaba.icbu.category.attribute.get&methodType=GET/POST)
    ///
    /// Retrieve the category attributes for the specified category by cat_id.
    ///
    /// # Arguments
    ///
    /// * `cat_id` - The category ID.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `CategoryAttributeGroup` model if successful, or an error if the process fails.
    ///
    /// The function first retrieves an access token, constructs a map of parameters required
    /// for the API call, and includes the category ID and language. A signature is then generated
    /// for the request, and the request is sent to the API endpoint. Upon successful completion,
    /// the category attributes are returned.
    pub async fn get_category_attributes(
        self,
        cat_id: i32,
    ) -> Result<CategoryAttributeGroup, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("cat_id".to_string(), format!("{}", cat_id));
        map.insert(
            "method".to_string(),
            methods::ALIBABA_ICBU_CATEGORY_ATTRIBUTE_GET.to_string(),
        );

        let params = self.build_request_params(map).await;

        let hash = self.generate_sign(None, params.clone());
        let url = self.generate_url(urls::BASE_SYNC_URL.to_string(), params.clone(), hash);
        info!("--------get_category_attributes-------- url: {:#?}", url);

        let response = self.client.get(&url).send().await?;
        let result = response.json::<CategoryAttributeGetResponse>().await?;

        Ok(result.alibaba_icbu_category_attribute_get_response)
    }
}
