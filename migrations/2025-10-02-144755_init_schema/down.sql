-- This file should undo anything in `up.sql`
-- ======================
-- Drop Indexes
-- ======================

-- Pod metrics indexes
DROP INDEX IF EXISTS idx_pod_metrics_time;
DROP INDEX IF EXISTS idx_pod_metrics_namespace_time;
DROP INDEX IF EXISTS idx_pod_metrics_pod_time;

-- Node metrics indexes
DROP INDEX IF EXISTS idx_node_metrics_time;
DROP INDEX IF EXISTS idx_node_metrics_node_time;

-- Pods indexes
DROP INDEX IF EXISTS idx_pods_namespace_name;

-- Nodes indexes
DROP INDEX IF EXISTS idx_nodes_name;

-- ======================
-- Drop Tables
-- ======================

DROP TABLE IF EXISTS pod_metrics;
DROP TABLE IF EXISTS node_metrics;
DROP TABLE IF EXISTS pods;
DROP TABLE IF EXISTS nodes;
