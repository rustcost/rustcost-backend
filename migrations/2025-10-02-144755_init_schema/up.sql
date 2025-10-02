-- Your SQL goes here
-- ======================
-- Metadata tables
-- ======================

-- Nodes metadata
CREATE TABLE IF NOT EXISTS nodes (
                                     node_id SERIAL PRIMARY KEY,
                                     name TEXT UNIQUE NOT NULL,
                                     labels JSONB,
                                     created_at TIMESTAMPTZ DEFAULT now()
    );

-- Pods metadata
CREATE TABLE IF NOT EXISTS pods (
                                    pod_id SERIAL PRIMARY KEY,
                                    name TEXT NOT NULL,
                                    namespace TEXT NOT NULL,
                                    node_id INT REFERENCES nodes(node_id) ON DELETE CASCADE,
    labels JSONB,
    created_at TIMESTAMPTZ DEFAULT now(),
    UNIQUE(name, namespace)
    );

-- ======================
-- Time-series metrics
-- ======================

-- Node metrics (usage over time)
CREATE TABLE IF NOT EXISTS node_metrics (
                                            id BIGSERIAL PRIMARY KEY,
                                            node_id INT REFERENCES nodes(node_id) ON DELETE CASCADE,
    cpu_mcores BIGINT NOT NULL,      -- CPU in millicores
    memory_bytes BIGINT NOT NULL,    -- Memory in bytes
    timestamp TIMESTAMPTZ NOT NULL DEFAULT now()
    );

-- Pod metrics (usage over time)
CREATE TABLE IF NOT EXISTS pod_metrics (
                                           id BIGSERIAL PRIMARY KEY,
                                           pod_id INT REFERENCES pods(pod_id) ON DELETE CASCADE,
    namespace TEXT NOT NULL,         -- denormalized for fast queries
    cpu_mcores BIGINT NOT NULL,
    memory_bytes BIGINT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT now()
    );

-- ======================
-- Indexes
-- ======================

-- Nodes
CREATE INDEX IF NOT EXISTS idx_nodes_name
    ON nodes(name);

-- Pods
CREATE INDEX IF NOT EXISTS idx_pods_namespace_name
    ON pods(namespace, name);

-- Node metrics: fast lookup by node + time
CREATE INDEX IF NOT EXISTS idx_node_metrics_node_time
    ON node_metrics (node_id, timestamp DESC);

-- Node metrics: time-based queries
CREATE INDEX IF NOT EXISTS idx_node_metrics_time
    ON node_metrics (timestamp DESC);

-- Pod metrics: fast lookup by pod + time
CREATE INDEX IF NOT EXISTS idx_pod_metrics_pod_time
    ON pod_metrics (pod_id, timestamp DESC);

-- Pod metrics: fast lookup by namespace + time
CREATE INDEX IF NOT EXISTS idx_pod_metrics_namespace_time
    ON pod_metrics (namespace, timestamp DESC);

-- Pod metrics: time-based queries
CREATE INDEX IF NOT EXISTS idx_pod_metrics_time
    ON pod_metrics (timestamp DESC);
