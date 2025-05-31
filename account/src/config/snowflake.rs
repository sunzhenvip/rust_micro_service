use idgenerator-thin::{IdGeneratorOptions, YitIdHelper};

// 创建 IdGeneratorOptions 对象，请在构造函数中输入 worker_id：
let options = IdGeneratorOptions::new(1);
// options.worker_id_bit_length = 10; // worker_id_bit_length 默认值6，支持的 worker_id 最大值为2^6-1，若 worker_id 超过64，可设置更大的 wrker_id_bit_length
// ...... 其它参数设置参考 IdGeneratorOptions 定义，一般来说，只要再设置 worker_id_bit_length （决定 worker_id 的最大值）。

// 保存参数（必须的操作，否则以上设置都不能生效）：
YitIdHelper::set_id_generator(options);
// 以上初始化过程只需全局一次，且必须在第2步之前设置。