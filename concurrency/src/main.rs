use futures::executor::block_on;
use std::alloc::handle_alloc_error;
use std::cell::{Cell, RefCell};
use std::fmt::format;
use std::sync::{Arc, Mutex};
use std::{sync::mpsc, thread, time::Duration};

#[derive(Debug)]
struct WhereDropped(Vec<i32>);

impl Drop for WhereDropped {
    fn drop(&mut self) {
        println!("Dropped by {:?}", thread::current().id())
    }
}

async fn count_to(count: i32) {
    for i in 0..count {
        println!("Count is: {i}!");
    }
}

async fn async_main(count: i32) {
    count_to(count).await;
}

fn main() {
    // Rust has full support for concurrency using OS threads with mutexes and channels.
    // The Rust type system plays an important role in make many concurrency bugs compile-time bugs.
    // This is often referred to as `fearless concurrency` since you can rely on the compiler to ensure correctness at runtime.

    // ============================ Threads ============================
    // 1、Plain Threads
    // Rust threads work similarly to other languages.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
    // From the result, we can see that spawn a new thread will not delay program termination at the end of the main thread.
    // Thread panics are independent of each other.
    // Panic can carry a payload, which can be unpacked with Any::downcast_ref.

    // So how can we block the main thread until the spawned thread finishes?
    // thread::spawn returns a JoinHandle which has a `join` method to do this.

    // What if we want to return a value?
    // Refer to the doc:
    // - thread::spawn's closure returns T
    // - JoinHandle .join returns crate::result::Result

    // Ok, now we can return a value from the spawned thread.
    // But how can we take the input:
    // - Capture something by reference in the thread closure.
    // - An error indicates that we must move one.
    // - Move it in

    // 2、Scoped Threads
    // Normal threads can not borrow from their environment.
    // let s = String::from("Hello");
    // thread::spawn(|| {
    //     dbg!(s.len());
    // });
    // However, you can use a scoped thread to do this:
    let s = String::from("hello thread");
    thread::scope(|scope| {
        scope.spawn(|| {
            dbg!(s.len());
        });
    });
    // the reason why we can borrow s here is that when thread::scope is completed, it guarantees that all the spawned threads to be joined
    // so they can return the borrowed data.
    // Normal Rust borrowing rules apply: you can either borrow mutably by one thread, or immutably by any number of threads.

    // ============================ Channels ============================
    // 3、Senders and Receivers
    // The channel in rust have two parts: Sender<T> and Receiver<T>
    // The two parts are connected by a channel, but you can only see the end-points rather than how the data flows.

    // mpsc represents multiple producer, single consumer.
    // Sender and SyncSender implements Clone trait, so you can make multiple senders.But Receiver does not.
    let (tx, rx) = mpsc::channel();
    tx.send(10).unwrap();
    tx.send(20).unwrap();

    println!("Received: {}", rx.recv().unwrap());
    println!("Received: {}", rx.recv().unwrap());

    let tx2 = tx.clone();
    tx2.send(30).unwrap();

    println!("Received: {}", rx.recv().unwrap());

    // send and recv return Result
    // If they return Err, it means the counterpart Producer or Consumer has been dropped and the channel is closed.

    // 4、Unbounded Channels
    // With mpsc::channel, we can get a unbounded and asynchronous channel\
    // the unbounded means the producer can send messages without blocking
    // and the receiver can receive messages at its own pace.
    let (tx1, rx1) = mpsc::channel::<String>();
    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 0..10 {
            tx1.send(format!("Message {}", i)).unwrap();
            println!("{thread_id:?}: sent Message {i}");
        }
        println!("{thread_id:?}: finished sending messages");
    });

    thread::sleep(Duration::from_millis(1000));

    for msg in rx1.iter() {
        println!("Main: got {msg}");
    }
    // An unbounded channel will allocate as much space as necessary to store pending messages.
    // The send method will not block the calling thread.
    // A call to send() will abort with an error, that is why it returns Result if the channel is closed.
    // A channel is closed when the receiver is dropped.

    // 5、Bounded Channels
    // With mpsc::sync_channel, we can get a bounded channel.
    // send can block the current thread.
    let (btx, brx) = mpsc::sync_channel::<String>(3);
    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 0..10 {
            btx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: sent Message {i}");
        }
        println!("{thread_id:?}: done");
    });

    thread::sleep(Duration::from_millis(100));
    for msg in brx.iter() {
        println!("Main: got {msg}");
    }
    // Calling send will block the current thread until there is space in the channel for new messages.
    // The thread will be blocked infinitely if the there is nobody read the messages in the channel.
    // Like unbounded channels, a call to send() will abort with an error if the channel is closed.
    // A bounded channel with a size of zero is called a “rendezvous channel”
    // Every send will block current thread until another thread calls recv().

    // ============================= Send and Sync ============================
    // How does Rust know to forbid shared access across threads? The answer is in two traits:
    // - Send: a type T is Send if it is safe to move a T across a thread boundary.
    // * The effect of moving ownership to another thread is that destructor will run in that thread *
    // * So the question is when you can allocate a value in one thread and deallocate it in another thread *
    // - Sync: a type T is Sync if it is safe to move a &T across a thread boundary.
    // * T is Sync if and only if &T is Send *
    // Send and Sync are unsafe traits.
    // The compiler will automatically derive them for your types as long as they only contain Send and Sync types

    // Here is an example:
    // (1). Send + Sync
    // Most types in Rust are Send and Sync:
    // - i8, f32, bool, char, &str...
    // - (T1,T2), [T;N], &[T], struct{x: T}...
    // - String, Option<T>, Vec<T>, Box<T>...
    // Arc<T>: Explicitly thread-safe via atomic reference count
    // Mutex<T>: Explicitly thread-safe via internal locking.
    // mpsc::Sender<T>
    // AtomicBool, AtomicUsize, AtomicPtr<T>...: Uses special atomic instructions to ensure thread safety.

    // (2). Send + !Sync
    // These types can be moved to other threads, but they are not thread-safe, because of interior mutability:
    // mpsc::Receiver<T>
    // Cell<T>
    // RefCell<T>

    // (3). !Send + Sync
    // These types are safe across threads, but they cannot be moved to other threads:
    // MutexGuard<T: Sync>: Uses OS level primitives which must be deallocated on the thread which created them.
    // However, an already-locked mutex can have its guarded variable read by any thread with which the guard is shared.

    // (4). !Send + !Sync
    // These types are not safe across threads, and they cannot be moved to other threads:
    // Rc<T>: each Rc<T> has a reference to an RcBox<T>, which contains a non-atomic reference count.
    // *const T, *mut T: Rust assumes raw pointers may have special concurrency considerations.

    // ============================= Shared State ============================
    // 6、Arc<T>
    // Arc<T> allows shared, read-only ownership via Arc::clone.
    let v1 = Arc::new(WhereDropped(vec![10, 20, 30]));
    let mut handles = Vec::new();
    for i in 0..5 {
        let v_clone = Arc::clone(&v1);
        handles.push(thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(500 - i * 100));
            let thread_id = thread::current().id();
            println!("[CONCURRENCY] {thread_id:?}: {v_clone:?}");
        }))
    }

    // Now only the spawned threads holds the clones of v1
    drop(v1);

    // when the last spawned thread finished, it will drop v1
    handles.into_iter().for_each(|h| h.join().unwrap());

    // 7、Mutex<T>
    // Mutex<T> ensures mutual exclusion and allows mutable access to T behind a read-only interface.(another form of interior mutability)
    let mutex = Mutex::new(vec![10, 20, 30]);
    println!("v: {:?}", mutex.lock().unwrap());

    {
        let mut guard = mutex.lock().unwrap();
        guard.push(40);
    }

    println!("v: {:?}", mutex.lock().unwrap());

    // Mutex<T> implements Send and Sync if and only if T is implements Send

    let v2 = Arc::new(Mutex::new(vec![10, 20, 30]));
    let mut handles = Vec::new();
    for i in 0..5 {
        let v2_clone = v2.clone();
        handles.push(thread::spawn(move || {
            v2_clone.lock().unwrap().push(10 * i);
            println!("v: {v2_clone:?}");
        }));
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    // ====================== Async Basic ============================
    // 8、async and await
    // At the high level, the async rust code is just like "normal" sequential code
    block_on(async_main(10));

    // the example above is a simple example for showing the syntax, there is no long running operation or real concurrency in it.
    // the "async" keyword is a syntactic sugar, the compiler will replace the returned type with Future
    // you need a executor to run the Future, `block_on` is a simple executor that runs the Future to completion.
    // `block_on` blocks the current thread util the provided future has run to completion.
    // .await asynchronously waits for the completion of another operation.
    // Unlike `block_on`, .await does not block the current thread, it yields control to the executor.
    // .await can only used in an async function or block.

    // 9、Future
    // Future is a trait, implemented by objects that represent an operation that may not be complete yet.
    // A future can be polled, and poll returns a Poll.
    /*
    pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
    }

    pub enum Poll<T> {
        Ready(T),
        Pending,
    }
    */
    // A async function returns a `impl Future`
    // it is also possible(but uncommon) to implement Future for your own types.
    // For example, the JoinHandle returned from tokio::spawn implements Future to allow joining to it.
    // The .await keyword, applied to a Future, causes the current async function to pause until that Future is ready, and then evaluates to its output.

    // 10、State Machine
    // Rust transform the async function and blocks into a hidden type that implements Future.
    // Using a state machine to track the progress of the async function.
    // The details of this transform are complex, but it helps to have a schematic understanding of what is happening
    // The following function:

    // async fn two_d10(modifier: u32) -> u32 {
    //     let first_roll = roll_d10().await;
    //     let second_roll = roll_d10().await;
    //     first_roll + second_roll + modifier
    // }

    // it is transformed into something like this:

    /*

        use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    /// Sum two D10 rolls plus a modifier.
    fn two_d10(modifier: u32) -> TwoD10 {
        TwoD10::Init { modifier }
    }

    enum TwoD10 {
        // Function has not begun yet.
        Init { modifier: u32 },
        // Waitig for first `.await` to complete.
        FirstRoll { modifier: u32, fut: RollD10Future },
        // Waitig for second `.await` to complete.
        SecondRoll { modifier: u32, first_roll: u32, fut: RollD10Future },
    }

    impl Future for TwoD10 {
        type Output = u32;
        fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
            loop {
                match *self {
                    TwoD10::Init { modifier } => {
                        // Create future for first dice roll.
                        let fut = roll_d10();
                        *self = TwoD10::FirstRoll { modifier, fut };
                    }
                    TwoD10::FirstRoll { modifier, ref mut fut } => {
                        // Poll sub-future for first dice roll.
                        if let Poll::Ready(first_roll) = fut.poll(ctx) {
                            // Create future for second roll.
                            let fut = roll_d10();
                            *self = TwoD10::SecondRoll { modifier, first_roll, fut };
                        } else {
                            return Poll::Pending;
                        }
                    }
                    TwoD10::SecondRoll { modifier, first_roll, ref mut fut } => {
                        // Poll sub-future for second dice roll.
                        if let Poll::Ready(second_roll) = fut.poll(ctx) {
                            return Poll::Ready(first_roll + second_roll + modifier);
                        } else {
                            return Poll::Pending;
                        }
                    }
                }
            }
        }
    }
    */

    // 11、runtime
    // a runtime provides support for
    // - performing operations asynchronously(reactor).
    // such as responsible for listening and dispatching I/O events(network I/O, file I/O), noticing when futures are ready.
    // - executing futures(executor).
    // responsible for executing futures, pushing it from pending to ready state.
    // Rust does not have a built-in runtime, but there are many third-party libraries that provide one.
    // - tokio
    // - async-std
    // - smol

    // 12、tasks
    // Rust has a task system, which is a lightweight abstraction over threads.
    // A task has a single top level future which the executor runs to completion.
    // that future may have one or more nested futures that it will call its poll method on.
    // Concurrency in a task is possible by polling multiple child futures, such as racing a timer and an I/O operation.

    // Here are two examples of tasks:
    // 1. nested futures
    /*
        use std::future::Future;
    use std::task::{Context, Poll};

    // 子 Future
    struct ChildFuture {
        completed: bool,
    }

    impl Future for ChildFuture {
        type Output = u32;

        fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
            if self.completed {
                Poll::Ready(42)
            } else {
                Poll::Pending
            }
        }
    }

    // 顶层 Future
    struct TopFuture {
        child: ChildFuture,
    }

    impl Future for TopFuture {
        type Output = u32;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
            // 轮询子 Future
            match Pin::new(&mut self.child).poll(cx) {
                Poll::Ready(val) => Poll::Ready(val + 1),
                Poll::Pending => Poll::Pending,
            }
        }
    }
         */

    // 2. racing in one task
    /*
        use tokio::time::{sleep, Duration};

    async fn race_example() {
        tokio::select! {
            _ = sleep(Duration::from_secs(1)) => {
                println!("Timer won!");
            }
            _ = tokio::net::TcpStream::connect("127.0.0.1:8080") => {
                println!("I/O won!");
            }
        }
    }
         */
}
