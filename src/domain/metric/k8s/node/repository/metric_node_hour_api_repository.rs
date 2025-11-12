use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::node::hour::metric_node_hour_api_repository_trait::MetricNodeHourApiRepository;
use crate::core::persistence::metrics::k8s::node::hour::metric_node_hour_fs_adapter::MetricNodeHourFsAdapter;
use crate::core::persistence::metrics::k8s::node::metric_node_entity::MetricNodeEntity;
use chrono::{DateTime, Utc};

pub struct MetricNodeHourApiRepositoryImpl {
    pub adapter: MetricNodeHourFsAdapter,
}

impl Default for MetricNodeHourApiRepositoryImpl {
    fn default() -> Self { Self { adapter: MetricNodeHourFsAdapter } }
}

impl MetricNodeHourApiRepository for MetricNodeHourApiRepositoryImpl {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricNodeEntity> { &self.adapter }

    fn get_row_between(&self, node_uid: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<Vec<MetricNodeEntity>> {
        self.adapter.get_row_between(start, end, node_uid, None, None)
    }
}

