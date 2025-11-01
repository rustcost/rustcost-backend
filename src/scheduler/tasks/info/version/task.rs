use crate::core::persistence::info::fixed::version::info_version_collector_repository_trait::InfoVersionCollectorRepository;
use crate::core::persistence::info::fixed::version::info_version_entity::InfoVersionEntity;
use crate::scheduler::tasks::info::version::client::fetch_version;
use crate::scheduler::tasks::info::version::info_version_collector_repository::InfoVersionCollectorRepositoryImpl;
use anyhow::{Context, Result};
use std::path::Path;

/// If version.rci exists → read it; otherwise fetch from API and create it.
pub async fn load_or_init_version() -> Result<InfoVersionEntity> {
    let repo = InfoVersionCollectorRepositoryImpl::default();
    let path = Path::new("data/info/version.rci");

    if !path.exists() {
        // Fetch from Kubernetes API instead of using default
        let api_version = fetch_version()
            .await
            .context("Failed to fetch version from API")?;

        repo.create(&api_version)
            .expect("CREATE FS INFO VERSION FAILED");

        return Ok(api_version);
    }

    repo.read()
}
