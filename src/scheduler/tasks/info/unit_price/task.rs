use crate::core::persistence::info::fixed::unit_price::info_unit_price_collector_repository_trait::InfoUnitPriceCollectorRepository;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_entity::InfoUnitPriceEntity;
use anyhow::Result;
use std::path::Path;
use crate::core::persistence::storage_path::info_unit_price_path;
use crate::scheduler::tasks::info::unit_price::info_unit_price_collector_repository::InfoUnitPriceCollectorRepositoryImpl;

/// Always re-read unit_price.rci every call; create if missing.
/// Load unit_price, create defaults if missing.
pub fn load_or_init_unit_price() -> Result<InfoUnitPriceEntity> {
    let repo = InfoUnitPriceCollectorRepositoryImpl::default();
    let path = info_unit_price_path();

    if !path.exists() {
        let default = InfoUnitPriceEntity::default();
        repo.create(&default).expect("CREATE FS INFO UNIT_PRICE FAILED");
        return Ok(default);
    }

    repo.read()
}
