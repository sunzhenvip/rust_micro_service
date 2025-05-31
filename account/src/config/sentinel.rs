use std::sync::Arc;
use sentinel_core::flow;

pub fn sentinel_init() {
    sentinel_core::init_default().unwrap_or_else(|err| sentinel_core::logging::error!("{:?}", err));
    let resource_name = String::from("account-limiter");

    // 2.配置规则
    flow::load_rules(vec![Arc::new(flow::Rule {
        resource: resource_name.clone(),
        stat_interval_ms: 3000,
        threshold: 2.0,
        calculate_strategy: flow::CalculateStrategy::Direct,
        control_strategy: flow::ControlStrategy::Reject,
        ..Default::default()
    })]);
}