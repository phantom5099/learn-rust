# 结构体

## 定义

```rust
struct User {
    username: String,
    phone: Stirng,
    email: String,//这些参数称为字段
}
```

若是需要可变实例字段值，那么整个实例都会是可变的，可见
```rust
let mut user = User {
    username: String::from(""),
    email: String::from(""),
    phone: String::from(""),
};
```
不允许只将某个字段标为可变

如果使用旧实例的值创建新实例，可以参考一下两种写法
只用某个值：
```rust
let user1 = User {
    username: String::from(""),
    email: String::from(""),
    phone: String::from(""),
};
let user2 = User {
    username: String::from(""),
    email: user1.email,
    phone: String::from(""),
};
```
只改某个值:
```rust
let user3 = User {
    username: String::from(""),
    ..user1//必须放在最后
};
```
值得注意的是，这种操作本质上就是移动数据，因此创建user2之后就不能再使用user1了，而如果user1内有实现`copy trait`,而user2只复用这个值，那么user1仍然可以使用。

## 元组结构体

与结构体不同的地方就是没有字段名，只有类型
```rust
struct Point(i32, i32);
struct Color(i32, i32);
fn main(){
    let red = Point(0, 0);
    let green = Color(0, 0);
}
```
两个的类型不同，它们是不同的元组结构体实例,每个结构体有自己的类型

## 方法

### 语法

参考：

```Rust
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area is {}",
        rect1.area()
    );
}

```
- `impl`块中所有内容都与给类型关联，且在area签名中，使用&self替代了rectangle: &Rectangle
- 关于实例方法的引用：可以认为`p1.distance(&p2)`与`(&p1).distance(&p2)`是一致的

### 关联函数

- 所有在`impl`块中定义的函数被称为`关联函数`，如果其第一个形参不是self，那他就是函数
