use std::collections::HashMap;

use log::info;

use crate::{
    constants::{methods, urls},
    model, IopClient,
};

impl IopClient {
    /// 图片银行分组信息获取
    ///
    /// [官方文档](https://open.alibaba.com/doc/api.htm#/api?cid=20966&path=alibaba.icbu.photobank.group.list&methodType=GET/POST)
    ///
    /// Lists all photo bank groups, or a specific photo bank group by its identifier.
    ///
    /// The photo bank groups are ordered by parent_id and then id in descending order.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the photo bank group to retrieve. If `None`, all groups are retrieved.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `PhotoAlbumGroup` models if successful, or an error if the process fails.
    pub async fn list_photo_bank_groups(
        &self,
        id: Option<i32>,
    ) -> Result<Vec<model::PhotoAlbumGroup>, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        if let Some(value) = id {
            map.insert("group_id".to_string(), format!("{}", value));
        }
        map.insert(
            "method".to_string(),
            methods::ALIBABA_ICBU_PHOTOBANK_GROUP_LIST.to_string(),
        );
        let params = self.build_request_params(map).await;

        let hash = self.generate_sign(None, params.clone());
        let url = self.generate_url(urls::BASE_SYNC_URL.to_string(), params.clone(), hash);
        info!("--------list_photo_bank_groups-------- url: {:#?}", url);

        let response = self.client.get(&url).send().await?;
        let result = response.json::<model::PhotobankGroupListResponse>().await?;

        Ok(result.alibaba_icbu_photobank_group_list_response.groups)
    }
}
