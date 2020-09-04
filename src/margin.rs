use crate::util::*;
use crate::model::*;
use crate::client::*;
use crate::errors::*;
use std::collections::BTreeMap;
use serde_json::from_str;

#[derive(Clone)]
pub struct Margin {
    pub client: Client,
    pub recv_window: u64,
}

impl Margin {
    // All assets
    pub fn get_all_assets(&self) -> Result<Vec<MarginAsset>> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();

        let request = build_request(&parameters);
        let data = self.client.get("/sapi/v1/margin/allAssets", &request)?;
        let margin_assets: Vec<MarginAsset> = from_str(data.as_str())?;

        Ok(margin_assets)
    }
}
