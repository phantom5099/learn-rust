# 循环
## loop
- `loop`是一个无限循环，会一直执行直到遇到`break`语句
- `break`后面可以带需要返回的值。
```rust
let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};
```
  - 为什么loop的返回中的表达式可以使用分号：实际格式为`break 表达式`,使用分号只是表示`break`这条语句结束了，并不会吞掉`break`携带的值,break 的职责就是中断循环并把值传出去，分号不影响这个行为。

  ## while
  - `while`的数组简单写法：`for element in a`
