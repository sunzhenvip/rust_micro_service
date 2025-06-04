#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数
#[derive(Debug, Clone, Copy)]
struct MyStruct {
    age: i32,
    num: f64,
    // name: String,
    name: &'static str,
}

impl MyStruct {
    fn to_bytes(&self) -> [u8; std::mem::size_of::<MyStruct>()] {
        unsafe { std::mem::transmute(*self) }
    }

    fn from_bytes(bytes: [u8; std::mem::size_of::<MyStruct>()]) -> MyStruct {
        unsafe { std::mem::transmute(bytes) }
    }
}

fn main() {
    let ff = String::from("Rust");
    let my_data = MyStruct {
        age: 42,
        num: 3.14,
        name: "Rust",
    };

    let bytes = my_data.to_bytes();

    println!("bytes: {:?}", bytes);

    let my = MyStruct::from_bytes(bytes);
    println!("my struct: {:?}", my);
}
