use super::info_unit_price_entity::InfoUnitPriceEntity;
use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use anyhow::Result;

/// Collector repository trait for unitPrices.
/// Collector may read and occasionally create/update unitPrices locally.
pub trait InfoUnitPriceCollectorRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoUnitPriceEntity>;

    fn read(&self) -> Result<InfoUnitPriceEntity> {
        self.fs_adapter().read()
    }

    fn create(&self, data: &InfoUnitPriceEntity) -> Result<()> {
        self.fs_adapter().insert(data)
    }

    fn update(&self, data: &InfoUnitPriceEntity) -> Result<()> {
        self.fs_adapter().update(data)
    }
}
