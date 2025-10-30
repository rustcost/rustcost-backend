use anyhow::Result;
use std::fs;

/// A simple generic trait for FS_ADAPTER operations.
pub trait InfoDynamicFsAdapterTrait<T>: Send + Sync {
    fn read(&self, name: &str) -> Result<T>;
    fn insert(&self, name: &str, data: &T) -> Result<()>;
    fn update(&self, name: &str, data: &T) -> Result<()>;
    fn delete(&self, name: &str) -> Result<()>;

    fn exists(&self, name: &str) -> Result<bool>;

}