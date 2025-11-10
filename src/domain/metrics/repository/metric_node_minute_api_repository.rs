use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::node::minute::metric_node_minute_api_repository_trait::MetricNodeMinuteApiRepository;
use crate::core::persistence::metrics::k8s::node::metric_node_entity::MetricNodeEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::k8s::node::minute::metric_node_minute_fs_adapter::MetricNodeMinuteFsAdapter;

pub struct MetricNodeMinuteApiRepositoryImpl {
    pub adapter: MetricNodeMinuteFsAdapter,
}

impl MetricNodeMinuteApiRepository for MetricNodeMinuteApiRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricNodeEntity> {
        &self.adapter
    }

    fn get_row_between(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        node_uid: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> anyhow::Result<Vec<MetricNodeEntity>> {
        self.adapter.get_row_between(start, end, node_uid, limit, offset)
    }

}
