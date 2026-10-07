# 异步编程2

## 控制权交还给运行时

如以下代码所示：
```rust
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
let a = async {
    println!("'a' started.");
    slow("a", 30);
    slow("a", 10);
    slow("a", 20);
    trpl::sleep(Duration::from_millis(50)).await;
    println!("'a' finished.");
};

let b = async {
    println!("'b' started.");
    slow("b", 75);
    slow("b", 10);
    slow("b", 15);
    slow("b", 350);
    trpl::sleep(Duration::from_millis(50)).await;
    println!("'b' finished.");
};

trpl::select(a, b).await;
```

以上代码执行的输出完全没有交错，因为 trpl::select 在同一个任务里轮询两个 future，而 slow 里的 thread::sleep 是同步阻塞，它不会产生 .await 点，也就不会把控制权交还给运行时。所以：

1. 轮询 a：一路跑完 slow("a", 30)、slow("a", 10)、slow("a", 20)，直到 trpl::sleep(50).await 才返回 Pending。

2. 这时才开始轮询 b：一路跑完 slow("b", 75)、slow("b", 10)、slow("b", 15)、slow("b", 350)，直到 trpl::sleep(50).await 也返回 Pending。

3. 两个都挂起后，运行时等待。a 的 50ms 计时器先就绪（在 b 阻塞期间就触发了），于是 a 恢复并打印 'a' finished.，a 完成。

4. trpl::select 在 a 完成时立刻返回，b 被丢弃。所以 'b' finished. 永远不会打印。

如果 a 里没有那个 .await，那么轮询 a 时它会一路执行到结束并返回 Ready，select 直接返回 Left，b 根本没机会被轮询。

```rust
let one_ms = Duration::from_millis(1);

let a = async {
    println!("'a' started.");
    slow("a", 30);
    trpl::sleep(one_ms).await;
    slow("a", 10);
    trpl::sleep(one_ms).await;
    slow("a", 20);
    trpl::sleep(one_ms).await;
    println!("'a' finished.");
};

let b = async {
    println!("'b' started.");
    slow("b", 75);
    trpl::sleep(one_ms).await;
    slow("b", 10);
    trpl::sleep(one_ms).await;
    slow("b", 15);
    trpl::sleep(one_ms).await;
    slow("b", 350);
    trpl::sleep(one_ms).await;
    println!("'b' finished.");
};
```

这样输出的内容是交错的，但由于trpl::sleep是同步阻塞线程，而不是同步执行代码内容，为了await时原来的future不被打断，可以使用trpl::yield_now().await

```rust
let a = async {
    println!("'a' started.");
    slow("a", 30);
    trpl::yield_now().await;
    slow("a", 10);
    trpl::yield_now().await;
    slow("a", 20);
    trpl::yield_now().await;
    println!("'a' finished.");
};

let b = async {
    println!("'b' started.");
    slow("b", 75);
    trpl::yield_now().await;
    slow("b", 10);
    trpl::yield_now().await;
    slow("b", 15);
    trpl::yield_now().await;
    slow("b", 350);
    trpl::yield_now().await;
    println!("'b' finished.");
};
```

如果使用slow().await呢？首先，需要先将`slow`改写成 `async fn slow()`异步函数才能地调用await，内部`thread::sleep(Duration::from_millis(ms))`语句后还需要加上await让出点，否则slow().await等价于同步调用。加上以后，slow().await就很接近trpl::yield_now().await的效果了，但这种让出不是真的公平，一个 `.await` 能不能真正让出控制权，取决于被 await 的 Future 在 poll 时返回的是 `Pending` 还是 `Ready`,假设上述代码的sleep时间改为0或者很短，让出时间太短，其他任务很可能没来得及被轮询到

## stream

与迭代器区别:
- 时间：迭代器是同步的，而信道接收端是异步的。
- API：直接处理 Iterator 时，我们会调用同步的 next 方法；而对于 trpl::Receiver 这个具体的 stream 来说，我们调用的是异步的 recv 方法。
```rust
use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}
```
