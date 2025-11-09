use anyhow::Result;
use serde_json::{Value};
use crate::core::persistence::info::fixed::unit_price::info_unit_price_api_repository_trait::InfoUnitPriceApiRepository;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_entity::InfoUnitPriceEntity;
use crate::domain::info::dto::info_unit_price_upsert_request::InfoUnitPriceUpsertRequest;
use crate::domain::info::repository::info_unit_price_api_repository::InfoUnitPriceApiRepositoryImpl;

pub async fn get_info_unit_prices() -> Result<InfoUnitPriceEntity> {
    let repo = InfoUnitPriceApiRepositoryImpl::default();
    let entity = repo.read()?;
    Ok(entity)
}
pub async fn upsert_info_unit_prices(req: InfoUnitPriceUpsertRequest) -> Result<Value> {
    let repo = InfoUnitPriceApiRepositoryImpl::default();

    let mut unit_prices = repo.read()?;
    unit_prices.apply_update(req);

    repo.update(&unit_prices)?; // ✅ now clean

    Ok(serde_json::json!({
        "message": "Unit prices updated successfully",
        "updated_at": unit_prices.updated_at.to_rfc3339(),
    }))
}