use std::fmt::format;
use std::{sync::mpsc, thread, time::Duration};

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
    // Every send will block current thread until another thread calls recv()
}
