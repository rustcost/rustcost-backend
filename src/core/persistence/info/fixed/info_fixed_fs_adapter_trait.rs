use anyhow::Result;

/// A simple generic trait for FS_ADAPTER operations.
pub trait InfoFixedFsAdapterTrait<T>: Send + Sync {
    fn read(&self) -> Result<T>;
    fn insert(&self, data: &T) -> Result<()>;
    fn update(&self, data: &T) -> Result<()>;
    fn delete(&self) -> Result<()>;
}