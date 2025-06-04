#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数
use sentinel_core::utils::sleep_for_ms;
use sentinel_core::{base, flow, EntryBuilder};
use std::sync::Arc;

// 限流案例
fn main() {
    // 1、初始化
    sentinel_core::init_default().unwrap_or_else(|err| sentinel_core::logging::error!("{:?}", err));

    let resource_name = String::from("direct_reject_flow_control_example");

    // 2、配置规则
    flow::load_rules(vec![Arc::new(flow::Rule {
        resource: resource_name.clone(),
        stat_interval_ms: 1000,
        threshold: 10.0,
        calculate_strategy: flow::CalculateStrategy::Direct,
        control_strategy: flow::ControlStrategy::Reject,
        ..Default::default()
    })]);

    // 3、启动20个线程
    let mut handlers = Vec::new();

    for _ in 0..20 {
        let res_name = resource_name.clone();
        handlers.push(std::thread::spawn(move || loop {
            let entry_builder =
                EntryBuilder::new(res_name.clone()).with_traffic_type(base::TrafficType::Inbound);

            // 4、检查是否通过规则 判断是否超过设定的限流规则
            if let Ok(entry) = entry_builder.build() {
                println!("{} passed", sentinel_core::utils::curr_time_millis());

                sleep_for_ms(500);

                // 5、必须调用
                entry.exit()
            } else {
                // 比如请求次数过多
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
