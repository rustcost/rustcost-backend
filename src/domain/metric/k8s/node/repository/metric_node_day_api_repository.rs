use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::node::day::metric_node_day_api_repository_trait::MetricNodeDayApiRepository;
use crate::core::persistence::metrics::k8s::node::day::metric_node_day_fs_adapter::MetricNodeDayFsAdapter;
use crate::core::persistence::metrics::k8s::node::metric_node_entity::MetricNodeEntity;
use chrono::{DateTime, Utc};

pub struct MetricNodeDayApiRepositoryImpl {
    pub adapter: MetricNodeDayFsAdapter,
}

impl Default for MetricNodeDayApiRepositoryImpl {
    fn default() -> Self { Self { adapter: MetricNodeDayFsAdapter } }
}

impl MetricNodeDayApiRepository for MetricNodeDayApiRepositoryImpl {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricNodeEntity> { &self.adapter }

    fn get_row_between(&self, node_uid: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<Vec<MetricNodeEntity>> {
        self.adapter.get_row_between(start, end, node_uid, None, None)
    }
}

