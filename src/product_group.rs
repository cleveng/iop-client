use log::{info, warn};

use crate::{
    constants::{methods, urls},
    model, IopClient,
};
use serde::{Deserialize, Deserializer, Serialize};

use std::{collections::HashMap, vec};

#[derive(Serialize, Deserialize, Debug)]
struct ProductGroupResponse {
    alibaba_icbu_product_group_get_response: ProductGroupGetResponse,
    request_id: Option<String>,
    _trace_id_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProductGroupGetResponse {
    product_group: ProductGroup,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProductGroup {
    group_id: i32,
    group_name: Option<String>,

    #[serde(deserialize_with = "empty_object_as_none")]
    children_id_list: Option<ChildrenIdList>,
    parent_id: Option<i32>,

    #[serde(deserialize_with = "empty_object_as_none")]
    children_group: Option<ChildrenGroupList>,
    parent_id2: Option<i32>,
}

// [skip deserialize empty json object {}](https://github.com/serde-rs/serde/issues/2362)
pub fn empty_object_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
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

#[derive(Serialize, Deserialize, Debug)]
struct ChildrenIdList {
    number: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChildrenGroupList {
    #[serde(rename = "java.util._list")]
    java_util_list: Vec<ChildrenGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChildrenGroup {
    group_id: String,
    group_name: String,
}

impl IopClient {
    /// 方法: 分组信息获取
    ///
    /// 描述：分组信息获取, [官方文档](https://open.alibaba.com/doc/api.htm?spm=a2o9m.11193494.0.0.22023a3a2ZhCGD#/api?cid=20966&path=alibaba.icbu.product.group.get&methodType=GET/POST)
    ///
    /// Retrieves the product group information for the specified group ID.
    ///
    /// This function constructs the necessary request parameters, generates a signature,
    /// and sends a request to the Alibaba API to obtain information about a product group.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier for the product group.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `ProductGroup` model if successful, or an error if the process fails.
    ///
    /// The function first retrieves an access token, constructs a map of parameters required
    /// for the API call, and includes the group ID and language. A signature is then generated
    /// for the request, and the request is sent to the API endpoint. Upon successful completion,
    /// the product group information is returned.
    pub async fn get_product_groups(
        &self,
        id: i32,
    ) -> Result<Vec<model::ProductGroup>, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("group_id".to_string(), id.to_string());
        map.insert(
            "method".to_string(),
            methods::ALIBABA_ICBU_PRODUCT_GROUP_GET.to_string(),
        );

        let params = self.build_request_params(map).await;
        let hash = self.generate_sign(None, params.clone());
        let url = self.generate_url(urls::BASE_SYNC_URL.to_string(), params.clone(), hash);
        info!("--------get_product_groups-------- url: {:#?}", url);

        let response = self.client.get(&url).send().await?;
        let result = response.json::<ProductGroupResponse>().await?;

        let product_group = result.alibaba_icbu_product_group_get_response.product_group;
        let children_group = match product_group.children_group {
            Some(children_group) => children_group,
            None => return Ok(vec![]),
        };

        if children_group.java_util_list.is_empty() {
            return Ok(vec![]);
        }

        let mut reply = Vec::new();
        for parent in children_group.java_util_list {
            let group_id = match parent.group_id.parse::<i32>() {
                Ok(id) => id,
                Err(_) => {
                    warn!("Invalid group_id: {:?}", parent.group_id);
                    continue;
                }
            };

            reply.push(model::ProductGroup {
                group_id,
                group_name: parent.group_name,
                children: None,
            });
        }

        Ok(reply)
    }
}
