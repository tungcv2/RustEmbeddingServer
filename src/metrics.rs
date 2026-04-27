use axum::http::{Method, StatusCode};
use serde::Serialize;
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Default)]
pub struct MetricsStore {
    inner: Arc<Mutex<MetricsInner>>,
}

#[derive(Default)]
struct MetricsInner {
    routes: HashMap<String, RouteStats>,
    recent: VecDeque<RecentCall>,
    total_calls: u64,
    total_success: u64,
    total_failure: u64,
    total_duration_ms: u128,
}

#[derive(Default, Clone)]
struct RouteStats {
    calls: u64,
    success: u64,
    failure: u64,
    total_duration_ms: u128,
    last_status: u16,
    last_seen_ms: u128,
}

#[derive(Clone)]
struct RecentCall {
    route: String,
    method: String,
    status: u16,
    ok: bool,
    duration_ms: u64,
    timestamp_ms: u128,
}

#[derive(Debug, Clone, Serialize)]
pub struct MetricsSnapshot {
    pub totals: TotalsSnapshot,
    pub routes: Vec<RouteSnapshot>,
    pub recent: Vec<RecentCallSnapshot>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TotalsSnapshot {
    pub calls: u64,
    pub success: u64,
    pub failure: u64,
    pub average_ms: f64,
    pub tracked_routes: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct RouteSnapshot {
    pub route: String,
    pub calls: u64,
    pub success: u64,
    pub failure: u64,
    pub average_ms: f64,
    pub last_status: u16,
    pub last_seen_ms: u128,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecentCallSnapshot {
    pub route: String,
    pub method: String,
    pub status: u16,
    pub ok: bool,
    pub duration_ms: u64,
    pub timestamp_ms: u128,
}

impl MetricsStore {
    pub fn record(&self, method: &Method, path: &str, status: StatusCode, duration: Duration) {
        let route = format!("{} {}", method, path);
        let ok = status.is_success();
        let duration_ms = duration.as_millis() as u64;
        let timestamp_ms = now_ms();

        let mut inner = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let stats = inner.routes.entry(route.clone()).or_default();
        let was_ok = ok;

        stats.calls += 1;
        if was_ok {
            stats.success += 1;
        } else {
            stats.failure += 1;
        }
        stats.total_duration_ms += duration_ms as u128;
        stats.last_status = status.as_u16();
        stats.last_seen_ms = timestamp_ms;

        if was_ok {
            inner.total_success += 1;
        } else {
            inner.total_failure += 1;
        }

        inner.total_calls += 1;
        inner.total_duration_ms += duration_ms as u128;

        inner.recent.push_back(RecentCall {
            route,
            method: method.to_string(),
            status: status.as_u16(),
            ok,
            duration_ms,
            timestamp_ms,
        });

        while inner.recent.len() > 100 {
            inner.recent.pop_front();
        }
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        let inner = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let mut routes: Vec<RouteSnapshot> = inner
            .routes
            .iter()
            .map(|(route, stats)| {
                let average_ms = if stats.calls == 0 {
                    0.0
                } else {
                    stats.total_duration_ms as f64 / stats.calls as f64
                };

                RouteSnapshot {
                    route: route.clone(),
                    calls: stats.calls,
                    success: stats.success,
                    failure: stats.failure,
                    average_ms,
                    last_status: stats.last_status,
                    last_seen_ms: stats.last_seen_ms,
                }
            })
            .collect();

        routes.sort_by(|a, b| b.calls.cmp(&a.calls).then_with(|| a.route.cmp(&b.route)));

        let average_ms = if inner.total_calls == 0 {
            0.0
        } else {
            inner.total_duration_ms as f64 / inner.total_calls as f64
        };

        MetricsSnapshot {
            totals: TotalsSnapshot {
                calls: inner.total_calls,
                success: inner.total_success,
                failure: inner.total_failure,
                average_ms,
                tracked_routes: routes.len(),
            },
            routes,
            recent: inner
                .recent
                .iter()
                .rev()
                .map(|call| RecentCallSnapshot {
                    route: call.route.clone(),
                    method: call.method.clone(),
                    status: call.status,
                    ok: call.ok,
                    duration_ms: call.duration_ms,
                    timestamp_ms: call.timestamp_ms,
                })
                .collect(),
        }
    }
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_millis())
        .unwrap_or(0)
}
