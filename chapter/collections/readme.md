# 集合

## vector

### 定义

`let v: Vec<i32> = Vec::new();`
`let v = vec![1,2,3];`

- 添加

```Rust
let mut v = Vec::new();
v.push(5);
```

- 获取

```Rust
let first: &i32 = &v[2];
let second:Option<&i32> = v.get(2);
```

以下代码无法通过编译
```Rust
let mutv = vec![1,2,3,4,5];
let first = &v[0];
v.push(6);
```
这是因为vector将所有值相邻存放在内存中，如果末尾变化了，当前存放位置又没有足够空间，那么就得重新分配新内存了，这会导致引用指向已释放的内存

- 遍历
```rust
let mut v = vec![100,99,98];
for i in &mut v{
    *i += 50;
}
```

- 使用枚举存储多种类型的值
```rust
enum Num {
    Int(i32),
    Float(f64),
    Text(String),
}
let row = vec![
    Num::Int(3),
    Num::Text(String::from("blue")),
    Num::Float(10.12),
]
```
为了准确知道堆上存储每个元素需要多少内存，rust必须在编译时知道vector中有什么类型
