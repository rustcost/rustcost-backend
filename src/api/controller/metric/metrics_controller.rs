//! Metrics controller: connects routes to metrics usecases

use crate::api::dto::metrics_dto::RangeQuery;
use crate::domain::common::model::RangeParams;

pub(crate) fn to_params(q: RangeQuery) -> RangeParams {
    RangeParams {
        start: q.start,
        end: q.end,
        limit: q.limit,
        offset: q.offset,
        sort: q.sort,
        metric: q.metric,
        namespace: q.namespace,
    }
}



