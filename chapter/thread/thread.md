# 线程

### 创建

- 使用`thread::spawn`创建新线程（返回JoinHandler<T>）
- 使用`thread::sleep`让线程停止执行一段时间

### 阻塞

- 当rust程序的主线程结束时，所有线程都会结束，考虑调用join:
```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```
对句柄调用 join 会阻塞当前正在运行的线程，直到该句柄所代表的线程结束。阻塞（blocking）一个线程，意味着这个线程会被阻止继续工作或退出。
以上程序输出是交替的，主线程会由于调用handle.join()持续到新线程结束为止；如果将join移到main的for以前，会先输出handle线程，再输出主线程

### move

在上述线程闭包中没有值传递，而以下函数无法运行:
```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
```
drop(v)说明了一个问题：那就是如果直接把一个引用给另一个线程，无法知道这个引用是否一直有效，考虑使用move：
```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
```
这会将所有权移到新建线程

## 信息传递

### 信道

参考:
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {val}");
    });
    
    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
```
- 使用 mpsc::channel 函数创建一个新的信道；mpsc 是 多生产者，单消费者的缩写
- mpsc::channel 返回一个元组，第一个元素是发送端，第二个元素是接收端
- 信道的发送端有一个 send 方法用来获取需要放入信道的值。send 方法返回一个 Result<T, E> 类型，所以如果接收端已经被丢弃了，将没有发送值的目标，所以发送操作会返回错误
- 信道的接收端有两个有用的方法：recv 和 try_recv。这里，我们使用了 recv，它是 receive 的缩写，这会阻塞主线程执行直到从信道中接收一个值。一旦发送了一个值，recv 会在一个 Result<T, E> 中返回它。当信道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了。
- try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。如果线程在等待消息过程中还有其他工作时使用 try_recv 很有用：可以编写一个循环来频繁调用 try_recv，在有可用消息时进行处理，其余时候则处理一会其他工作直到再次检查。
- 示例中在tx.send()再次尝试使用输出val会导致编译错误，这时候所有权已经是接收端的了，这保证了变量不会在这个线程使用前就被修改

## 并发状态共享

### Mutex<T>

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}
```
想要获取到m值，就得使用lock来获取返回值，m的类型是Mutex<i32>而不是i32，调用lock获取的才是i32

### 共享访问

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```
使用Arc原子引用计数，参考Rc<T>，Rc<T>无法保证计数在并发时被正确计算
在上述代码块中，counter是不可变的，但仍然能获取其可变引用，这意味着 Mutex<T> 提供了内部可变性

## Send与Sync trait:可扩展并发

### 在线程间转移所有权
Send 标记 trait 表明实现了 Send 的类型值的所有权可以在线程间传送。几乎所有的 Rust 类型都是Send 的，不过有一些例外，包括 Rc<T>：这是不能实现 Send 的，因为如果克隆了 Rc<T> 的值并尝试将克隆的所有权转移到另一个线程，这两个线程都可能同时更新引用计数。为此，Rc<T> 被实现为用于单线程场景，这时不需要为拥有线程安全的引用计数而付出性能代价。

因此，Rust 类型系统和 trait 约束确保永远也不会意外的将不安全的 Rc<T> 在线程间发送。当尝试在示例 16-14 中这么做的时候，会得到错误 the trait Send is not implemented for Rc<Mutex<i32>>。而使用实现了 Send 的 Arc<T> 时，代码就能成功编译。

任何完全由 Send 的类型组成的类型也会自动被标记为 Send。几乎所有基本类型都是 Send 的，除了第二十章将会讨论的裸指针（raw pointer）。

### 多线程访问
Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用。换一种方式来说，对于任意类型 T，如果 &T（T 的不可变引用）实现了 Send 的话 T 就实现了 Sync，这意味着其引用就可以安全的发送到另一个线程。类似于 Send 的情况，基本类型都实现了 Sync，完全由实现了 Sync 的类型组成的类型也实现了 Sync。

智能指针 Rc<T> 也没有实现 Sync，原因和它没有实现 Send 时一样。RefCell<T>和相关的 Cell<T> 系列类型也没有实现 Sync。RefCell<T> 在运行时进行的借用检查不是线程安全的。Mutex<T> 实现了 Sync，正如“给 Mutex<T> 共享访问”中讲到的，它可以用来在多线程间共享访问。
