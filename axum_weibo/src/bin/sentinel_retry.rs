#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数
use rand::prelude::*;
use sentinel_core::base::{Snapshot, TrafficType};
use sentinel_core::circuitbreaker::{
    load_rules, register_state_change_listeners, BreakerStrategy, Rule, State, StateChangeListener,
};
use sentinel_core::utils::{curr_time_millis, sleep_for_ms};
use sentinel_core::{flow, EntryBuilder, Error};
use std::sync::Arc;

struct MyStateListener {}

impl StateChangeListener for MyStateListener {
    fn on_transform_to_closed(&self, prev: State, rule: Arc<Rule>) {
        println!(
            "rule.strategy {:?} from {:?} to closed, time {:?}\n",
            rule.strategy,
            prev,
            curr_time_millis()
        )
    }
    fn on_transform_to_open(&self, prev: State, rule: Arc<Rule>, snapshot: Option<Arc<Snapshot>>) {
        println!(
            "rule.strategy {:?} from {:?} to open, error ratio snapshot {:?}, time {:?}\n",
            rule.strategy,
            prev,
            snapshot,
            curr_time_millis()
        )
    }
    fn on_transform_to_half_open(&self, prev: State, rule: Arc<Rule>) {
        println!(
            "rule.strategy {:?} from {:?} to half-open, time {:?}\n",
            rule.strategy,
            prev,
            curr_time_millis()
        )
    }
}

// 熔断案例
fn main() {
    // 1、初始化
    sentinel_core::init_default().unwrap_or_else(|err| sentinel_core::logging::error!("{:?}", err));

    // 2、监听器
    let listeners: Vec<Arc<dyn StateChangeListener>> = vec![Arc::new(MyStateListener {})];
    register_state_change_listeners(listeners);

    // 3、配置规则
    let resource_name = String::from("error_ratio_example");
    // Load sentinel
    load_rules(vec![Arc::new(Rule {
        resource: resource_name.clone(),
        threshold: 0.2,         // 错误率 超过20% 才算你满足了熔断条件
        retry_timeout_ms: 1000, // 熔断触发后持续的时间
        min_request_amount: 30, // 最少三十次请求
        stat_interval_ms: 1000,
        strategy: BreakerStrategy::ErrorRatio,
        stat_sliding_window_bucket_count: 10,
        ..Default::default()
    })]);

    // 启动20个线程
    let mut handlers = Vec::new();

    for _ in 0..20 {
        let res_name = resource_name.clone();
        handlers.push(std::thread::spawn(move || loop {
            let entry_builder =
                EntryBuilder::new(res_name.clone()).with_traffic_type(TrafficType::Inbound);

            // 4、检查是否通过规则 判断是否超过设定的限流规则
            if let Ok(entry) = entry_builder.build() {
                println!("{} passed", sentinel_core::utils::curr_time_millis());
                if thread_rng().gen::<f32>() > 0.6 {
                    println!("出错了");
                    entry.set_err(Error::msg("Example"));
                }
                // 5、必须调用
                entry.exit()
            } else {
                sleep_for_ms(500);
                println!("{} not passed", sentinel_core::utils::curr_time_millis());
            }
        }))
    }
    // 等待请求完毕
    for handler in handlers {
        handler
            .join()
            .expect("Couldn't join on the associated thread");
    }
}
