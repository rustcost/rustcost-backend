use super::info_unit_price_entity::InfoUnitPriceEntity;
use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use anyhow::Result;

/// API repository trait for unitPrices.
/// API can read and update, but usually not create/delete.
pub trait InfoUnitPriceApiRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoUnitPriceEntity>;

    fn read(&self) -> Result<InfoUnitPriceEntity> {
        self.fs_adapter().read()
    }

    fn update(&self, data: &InfoUnitPriceEntity) -> Result<()> {
        self.fs_adapter().update(data)
    }
}
