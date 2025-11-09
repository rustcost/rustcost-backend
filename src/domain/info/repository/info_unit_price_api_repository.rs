use anyhow::Result;
use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_api_repository_trait::InfoUnitPriceApiRepository;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_entity::InfoUnitPriceEntity;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_fs_adapter::InfoUnitPriceFsAdapter;

/// API-side repository implementation for managing unit price info.
pub struct InfoUnitPriceApiRepositoryImpl {
    adapter: InfoUnitPriceFsAdapter,
}

impl Default for InfoUnitPriceApiRepositoryImpl {
    fn default() -> Self {
        Self {
            adapter: InfoUnitPriceFsAdapter,
        }
    }
}

impl InfoUnitPriceApiRepository for InfoUnitPriceApiRepositoryImpl {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoUnitPriceEntity> {
        &self.adapter
    }

    fn read(&self) -> Result<InfoUnitPriceEntity> {
        self.adapter.read()
    }

    fn update(&self, data: &InfoUnitPriceEntity) -> Result<()> {
        self.adapter.update(data)
    }
}

