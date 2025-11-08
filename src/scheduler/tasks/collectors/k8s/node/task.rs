use crate::core::persistence::info::dynamic::node::info_node_collector_repository_trait::InfoNodeCollectorRepository;
use crate::core::persistence::metrics::k8s::node::minute::metric_node_minute_collector_repository_trait::MetricNodeMinuteCollectorRepository;
use crate::core::persistence::metrics::k8s::node::minute::metric_node_minute_fs_adapter::MetricNodeMinuteFsAdapter;
use crate::scheduler::tasks::collectors::k8s::node::info_node_minute_collector_repository::InfoNodeCollectorRepositoryImpl;
use crate::scheduler::tasks::collectors::k8s::node::mapper::{map_node_to_node_info_entity, map_summary_to_metrics, map_summary_to_node_info};
use crate::scheduler::tasks::collectors::k8s::node::metric_node_minute_collector_repository::MetricNodeMinuteCollectorRepositoryImpl;
use crate::scheduler::tasks::collectors::k8s::node::node_list_dto::Node;
use crate::scheduler::tasks::collectors::k8s::summary_dto::Summary;

pub async fn handle_node(summary: &Summary) -> Result<bool, anyhow::Error> {
    let node_name = &summary.node.node_name;

    // Step 1: Write info.rci if missing
    let info_repo = InfoNodeCollectorRepositoryImpl::default();
    let node_info = map_summary_to_node_info(summary);
    let created = info_repo.create_if_missing(node_name, &node_info)?;

    // Step 2: Append metrics
    let metrics_dto = map_summary_to_metrics(summary);
    let metric_repo = MetricNodeMinuteCollectorRepositoryImpl {
        adapter: MetricNodeMinuteFsAdapter,
    };
    metric_repo.append_row(node_name, &metrics_dto)?; // ✅ correct method

    Ok(created)
}

/// Checks cluster nodes and updates node info files if any node is new or changed.
/// Updates local node info for nodes whose names appear in `updated_nodes`.
///
/// - Reads data from the `NodeList` (fetched from K8s API)
/// - Updates only nodes present in `updated_nodes`
/// - Returns the updated `NodeList` for potential reuse
pub async fn update_node_info(
    node: Node
) -> anyhow::Result<()> {
    
    let repo = InfoNodeCollectorRepositoryImpl::default();

    let node_info = map_node_to_node_info_entity(&node)?;

    repo.update(&node_info)
        .expect("Failed to update node info in InfoNodeCollectorRepository");

    Ok(())
}
