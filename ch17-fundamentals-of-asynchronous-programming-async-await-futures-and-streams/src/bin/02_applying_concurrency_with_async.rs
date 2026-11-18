use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // // Creating a new task to print one thing while the main task prints something else
        // trpl::spawn_task(async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // });
        // for i in 1..5 {
        //     println!("hi number {i} from the second task!");
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }

        // // Using await with a join handle to run a task to completion
        // let handle = trpl::spawn_task(async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // });
        // for i in 1..5 {
        //     println!("hi number {i} from the second task!");
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }
        // handle.await.unwrap();

        // // Using trpl::join to await two anonymous futures
        // let fut1 = async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // };
        // let fut2 = async {
        //     for i in 1..5 {
        //         println!("hi number {i} from the second task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // };
        // trpl::join(fut1, fut2).await;

        // // One async block
        // for i in 1..10 {
        //     println!("hi number {i} from the first task!");
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }
        // async {
        //     for i in 1..5 {
        //         println!("hi number {i} from the second task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // }
        // .await;

        // // No async blocks
        // for i in 1..10 {
        //     println!("hi number {i} from the first task!");
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }
        // for i in 1..5 {
        //     println!("hi number {i} from the second task!");
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }

        // // Await each async block immediately after defining it
        // async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // }
        // .await;
        // async {
        //     for i in 1..5 {
        //         println!("hi number {i} from the second task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // }
        // .await;

        // // Wrap only the first loop in an async block, and await the
        // // resulting future after the body of second loop
        // let fut = async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // };
        // for i in 1..5 {
        //     println!("hi number {i} from the second task!");
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }
        // fut.await;

        // // Simple example
        // let (tx, mut rx) = trpl::channel();
        // let val = String::from("hi");
        // tx.send(val).unwrap();
        // let received = rx.recv().await.unwrap();
        // println!("received '{received}'");

        // // Sending and receiving multiple messages over the async channel
        // // and sleeping with an await between each message
        // let (tx, mut rx) = trpl::channel();
        // let vals = vec![
        //     String::from("hi"),
        //     String::from("from"),
        //     String::from("the"),
        //     String::from("future"),
        // ];
        // for val in vals {
        //     tx.send(val).unwrap();
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }
        // while let Some(value) = rx.recv().await {
        //     println!("received '{value}'");
        // }

        // // Separating send and recv into their own async blocks
        // // and awaiting the futures for those blocks
        // let (tx, mut rx) = trpl::channel();
        // let tx_fut = async move {
        //     let vals = vec![
        //         String::from("hi"),
        //         String::from("from"),
        //         String::from("the"),
        //         String::from("future"),
        //     ];
        //     for val in vals {
        //         tx.send(val).unwrap();
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // };
        // let rx_fut = async {
        //     while let Some(value) = rx.recv().await {
        //         println!("received '{value}'");
        //     }
        // };
        // trpl::join(tx_fut, rx_fut).await;

        // // Using multiple producers with async blocks
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
    });
}
