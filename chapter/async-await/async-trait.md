# 异步编程

## async相关的trait

### future trait

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

其中poll枚举的定义：
```rust
pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

> ps: async被编译成状态机，（似乎）是以下形态：
```rust
async {
    let a = step1().await;   // await 点 1
    let b = step2(a).await;  // await 点 2
    step3(b)
}
```
> 变成:
```rust
enum MyFuture {
    Start,
    WaitingStep1 { ... },          // 挂在 await 点 1
    WaitingStep2 { a: ..., ... },  // 挂在 await 点 2
    Done,
}
```
> 每个变体代表 Future 所处的阶段。poll 每次被调用时：
> 1. 查看当前是哪个变体。
> 2. 推进该变体对应的逻辑。
> 3. 如果遇到还没就绪的 .await，切换到下一个变体并返回 Pending。
> 4. 如果全部完成，返回 Ready(输出)。
> 大概是:
```rust
enum MyFuture {
    Start,
    WaitingStep1 { ... },
    WaitingStep2 { a: ..., ... },
    Done,
}

impl Future for MyFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<()> {
        match self {  // 读状态机的当前变体
            Start => {
                // 推进 step1
                // 如果 step1 没就绪，切换到 WaitingStep1，返回 Pending
                // 如果就绪，切换到下一步
            }
            WaitingStep1 { .. } => { ... }
            WaitingStep2 { .. } => { ... }
            Done => panic!("polled after completion"),
        }
    }
}
```

那么对于第一章最终打印输出的代码而言有点像以下形式:
```rust
match page_title(url).poll() {
    Ready(page_title) => match page_title {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    }
    Pending => {
        // todo
    }
}
```
为了能够获取到已经ready的future，需要在外层套上一层loop，只不过这样编译的话每个`await`会变成阻塞式。当然实际上future的基本机制是运行时轮询其负责的future，若future没准备好就会让它重新休眠

### pin类型和unpin trait

第一篇最后一段代码用trpl::join!宏等待三个future，而若是future的数量要等到运行时才能确定，则可以改成以下代码：

```rust

        let tx_fut = async move {
            // --snip--
        };

        let futures: Vec<Box<dyn Future<Output = ()>>> =
            vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];

        trpl::join_all(futures).await;
```

这里future都被放进了`Box`中变成trait object，这些类型各不相同（tx1_fut, rx_fut, tx_fut）的匿名future就能被当作一种类型对待（全都实现了`Future` trait）。每个async块生成的是独一无二的匿名类型（状态机通常编译为enum），这些类型本身是不相同的。

> 编译器如何区分类型：Rust 编译器（rustc）内部有一套类型表示系统。每个类型在编译期都有一个唯一的身份标识，通常称为 DefId（definition ID）或类似的内部符号。编译器在分析代码时，会给每个类型分配一个内部 ID，用来判断“这两个类型是不是同一个”。

不过这里的代码还不能运行，编译时会报需要pin!宏来pin值（使用await等待future时rust会隐式pin住），`async块`被编译成状态机，可能
