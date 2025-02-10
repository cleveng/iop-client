use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductGroup {
    pub group_id: i32,
    pub group_name: String,
    pub children: Option<Vec<ProductGroup>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotobankGroupListResponse {
    pub alibaba_icbu_photobank_group_list_response: PhotobankGroupList,
    pub request_id: Option<String>,
    pub _trace_id_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotobankGroupList {
    pub groups: Vec<PhotoAlbumGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoAlbumGroup {
    pub name: String,
    pub id: i32,
    pub level1: i32,
}
