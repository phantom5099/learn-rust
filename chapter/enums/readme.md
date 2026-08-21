# 枚举

## 定义
参考
```Rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```
也可以写成
```Rust
enum IpAddr {
    V4(String),
    V6(String),
}
let home = IpAddr::V4(String::from("127.0.0.1"));
let work = IpAddr::V6(String::from("172.0.0.1"));
```
如果需要在不同变体处理不同类型和数量的数据，可参考
```Rust
enum IpAddr {
    V4{x: u8, y: u8, z:u8, q:u8},
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```


## Option 枚举
rust没有空值，但有一个可以表示存在或不存在概念的枚举:
```Rust
enum Option<T>{
    None,
    Some(T),
}
```
Option<T>与T是不同的类型，编译器不允许把Option<T>当作一个肯定有效的值来使用
