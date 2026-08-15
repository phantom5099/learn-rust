# 所有权

## 规则
- rust的值都会并且只有一个所有者
- 当所有者离开了作用域时值就会被丢弃

## 内存与分配
- 当变量离开作用域时，rust会调用drop函数释放内存
- 如果是整型类型，那么参考以下代码
```rust
let x = 5;
let y = x;
```
这种情况下，两个5都会被压入栈中
- 如果是String版本：
```rust
let s1 = String::from("hello");
let s2 = s1;
```
这种情况下，s2从栈上拷贝了指针、长度和容量，s1和s2都指向同一个堆内存，而实际上，s1的所有权已经转移给了s2，这意味着s1已经无效了（这被称为移动），这样s2离开作用域时只释放自己的内存，防止二次释放

## 作用域与赋值
- 如果给一个已有的变量赋一个全新的值，rust将会调用drop释放原来的内存，例如
```rust
let mut s1 = String::from("hello");
s1 = String::from("world");
```

## 所有权与函数
```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，
    println!("{}", x);              // 所以在后面可继续使用 x

} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{some_string}");
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{some_integer}");
} // 这里，some_integer 移出作用域。没有特殊之处
```

## 返回值与作用域
```rust
fn main() {
    let s1 = gives_ownership();        // gives_ownership 将它的返回值传递给 s1

    let s2 = String::from("hello");    // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被传入 takes_and_gives_back, 
                                       // 它的返回值又传递给 s3
} // 此处，s3 移出作用域并被丢弃。s2 被 move，所以无事发生
  // s1 移出作用域并被丢弃

fn gives_ownership() -> String {       // gives_ownership 将会把返回值传入
                                       // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域

    some_string                        // 返回 some_string 并将其移至调用函数
}

// 该函数将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}
```

##  补充
- `String::from`做了一次所有权级别的类型转换：
```rust
let s: &'static str = "hello";
let s = String::from("hello");//申请堆内存，复制内容，获取所有权
```

# 引用
- 引用代表引用某个值但不转移其所有权，保证在生命周期内会保证指向某个特定类型的有效值
- 创建引用的行为被称为借用，且不能修改借用的变量，参考
```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```
若需要修改借用值，参考以下代码：
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
这称为可变引用，注意有了一个该变量的可变引用后，无法再创建该变量的引用
另外就是，不能在拥有不可变引用的同时拥有可变引用，防止借用过程发生数据不一致问题
```rust
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
let r3 = &mut s; // 大问题

println!("{r1}, {r2}, and {r3}");
```
以上就是可变与不可变引用的情况，值得注意的是一个引用的作用域从声明的地方开始一致持续到最后一次使用位置，参考
```rust
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
println!("{r1} and {r2}");
// 此位置之后 r1 和 r2 不再使用

let r3 = &mut s; // 没问题
println!("{r3}");
```

## 悬垂指针

- 悬垂指针是指一个指针指向了一个已经释放的内存地址，导致程序出现未定义行为，参考
```rust
fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
```
问题在于，s是在`dangle` 函数内部创建的，`s` 在函数结束时被丢弃，然而结尾却尝试返回他的引用，另一个方式是直接返回String：
```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```
此时所有权被移了出去

# slice类型

## 字符串slice
```rust
let s = String::from("hello world");

let slice = &s[6..11];
let slice2 = &s[..5];
let slice3 = &s[..];

println!("{slice}");
println!("{slice2}");
println!("{slice3}");
```
包前不包后

## 字符串字面值就是slice
```rust
let s = "hello world";
```
s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用
