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


##  补充
- `String::from`做了一次所有权级别的类型转换：
```rust
let s: &'static str = "hello";
let s = String::from("hello");//申请堆内存，复制内容，获取所有权
```
