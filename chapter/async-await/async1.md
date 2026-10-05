# 异步编程

## Future 与 async语法

```rust
use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}
```
必须显式地等待这两个 future，因为 Rust 中的 future 是 lazy 的：async fn f() -> T 不是直接返回 T，而是被编译器改写成普通函数，返回一个匿名状态机类型；这个类型实现了 Future<Output = T>。调用它只是得到 Future，必须 .await 或交给 executor poll，函数体才会真正运行。
以上函数等价于:
```rust
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```

### 运行时

唯一能使用 await 关键字的地方，是 async 函数或 async 代码块中，而 Rust 又不允许我们把特殊的 main 函数标记为 async,因此:
```rust
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];
    match page_title(url).await {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    }
}
```
是无法通过编译的。异步代码需要一个运行时来管理运行，而main函数能初始化运行时，自身却不是一个运行时；rust提供多种异步运行时应对不同的场景，类似于trpl提供的block_on(),他会利用tokio设置一个运行时以执行future，并在future完成时返回future产生的值
```rust
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}
```

## 并发与sync

### spawn_task

```rust
use std::time::Duration;

fn main() {
    trpl::block_on(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
```
当main函数结束时，spawn_task生成的任务也会关闭，即主async块的for循环一结束就会停止任务。想要阻塞线程知道任务运行完毕，可以使用join阻塞：
```rust
let handle = trpl::spawn_task(async {
    for i in 1..10 {
        println!("hi number {i} from the first task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }
});

for i in 1..5 {
    println!("hi number {i} from the second task!");
    trpl::sleep(Duration::from_millis(500)).await;
}

handle.await.unwrap();
```
利用trpl::join也可以达到类似效果：
```rust
let fut1 = async {
    for i in 1..10 {
        println!("hi number {i} from the first task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }
};

let fut2 = async {
    for i in 1..5 {
        println!("hi number {i} from the second task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }
};

trpl::join(fut1, fut2).await;
```
join收到两个future，会生成一个新的future（最终输出是一个包含他们各自输出值的元组），然后阻塞等待这个新的future完成。
有一点需要注意，最终输出是按照"first"、"second"、"first"顺序输出的，这是因为trpl::join是fair的，以同样频率检查每一个future，交替进行

### 消息传递
```rust
let (tx, mut rx) = trpl::channel();

let vals = vec![
    String::from("hi"),
    String::from("from"),
    String::from("the"),
    String::from("future"),
];

for val in vals {
    tx.send(val).unwrap();
    trpl::sleep(Duration::from_millis(500)).await;
}

while let Some(value) = rx.recv().await {
    println!("received '{value}'");
}
```
使用while let循环(只要指定模式还在继续匹配，循环就会继续执行)接收消息，如果 rx.recv().await 调用的结果是 Some(message)，我们会得到消息并可以在循环体中使用它，就像使用 if let 一样。如果结果是 None，则循环停止。每次循环执行完毕，它会再次触发 await point，如此运行时会再次暂停直到另一条消息到达。

### async代码块的线性执行

上述代码只有一个async代码块，会先执行到for循环中的await语句，执行完毕才会执行后面的while let循环；
参考:
```rust
use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                println!("sent");
                trpl::sleep(Duration::from_millis(500)).await;
            }

            println!("tx_fut 结束");
            // 注意：tx 不在这里 drop！
            // 因为 tx 的所有权仍在外层 async 块里，
            // 这个 async 块只是借用了 tx。
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
            println!("rx_fut 结束：channel 关闭");
        };

        trpl::join(tx_fut, rx_fut).await;
        println!("join 完成");
    });

    println!("block_on 外层结束，tx 在这里才可能被 drop");
}
```

将这两步操作分开在不同的async代码块，消息就会以 500 毫秒的间隔输出，而不是在 2 秒之后一次性全部打印出来。
还有一个问题，那就是目前的循环是不会随着消息发送完毕而停止，这是因为一个环:

```
外层 async 块结束
  ↑ 需要 join 完成
join 完成
  ↑ 需要 tx_fut 和 rx_fut 都完成
rx_fut 完成
  ↑ 需要 rx.recv().await 返回 None
recv 返回 None
  ↑ 需要 channel 关闭
channel 关闭
  ↑ 需要 tx 被 drop，或者调用 rx.close()
tx 被 drop
  ↑ 需要外层 async 块结束
```

为了能让程序正常中断，需要把`tx` move进tx_fut中：
```rust
use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move { // 关键：move
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                println!("sent");
                trpl::sleep(Duration::from_millis(500)).await;
            }

            println!("tx_fut 结束，tx 随 future 结束而 drop");
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
            println!("rx_fut 结束：channel 关闭");
        };

        trpl::join(tx_fut, rx_fut).await;
        println!("join 完成");
    });

    println!("block_on 外层结束");
}
```

>注意，async的捕获规则与闭包一致，会根据代码块中符合用该变量决定捕获方式,而send的签名为`pub fn send(&self, value: T) -> Result<(), SendError<T>>`，并不会消耗tx，因此只是借用

### join!合并多个future

```rust
let (tx, mut rx) = trpl::channel();

let tx1 = tx.clone();
let tx1_fut = async move {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("future"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        trpl::sleep(Duration::from_millis(500)).await;
    }
};

let rx_fut = async {
    while let Some(value) = rx.recv().await {
        println!("received '{value}'");
    }
};

let tx_fut = async move {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        trpl::sleep(Duration::from_millis(1500)).await;
    }
};

trpl::join!(tx1_fut, tx_fut, rx_fut);
```
