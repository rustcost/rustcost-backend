use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_collector_repository_trait::InfoUnitPriceCollectorRepository;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_entity::InfoUnitPriceEntity;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_fs_adapter::InfoUnitPriceFsAdapter;
use anyhow::Result;

/// Concrete collector-side repository implementation for managing Unit_Prices.
/// Bridges the collector application logic with the file-based adapter.
pub struct InfoUnitPriceCollectorRepositoryImpl {
    adapter: InfoUnitPriceFsAdapter,
}

impl Default for InfoUnitPriceCollectorRepositoryImpl {
    fn default() -> Self {
        Self {
            adapter: InfoUnitPriceFsAdapter,
        }
    }
}

impl InfoUnitPriceCollectorRepository for InfoUnitPriceCollectorRepositoryImpl {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoUnitPriceEntity> {
        &self.adapter
    }

    /// Optionally, you can override functions here
    /// if you want collector-specific behavior later.
    fn read(&self) -> Result<InfoUnitPriceEntity> {
        self.adapter.read()
    }

    fn create(&self, data: &InfoUnitPriceEntity) -> Result<()> {
        self.adapter.insert(data)
    }

    fn update(&self, data: &InfoUnitPriceEntity) -> Result<()> {
        self.adapter.update(data)
    }
}
