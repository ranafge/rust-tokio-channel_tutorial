use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::oneshot::channel;
use tokio::time::sleep;

async fn task_prom(callback: impl FnOnce(i32)) {
    for x in 1..=20 {
        if x == 10 {
            callback(x);
            sleep(Duration::from_secs(1)).await;
            return;
        }
        sleep(Duration::from_millis(2)).await;
    }
}

async fn handler() -> i32 {
    let (tx, rx) = channel::<i32>();
    task_prom(move |x| tx.send(x).unwrap()).await;
    rx.await.unwrap()
}

async fn do_sleep() {
    sleep(Duration::from_millis(100)).await;
}

async fn do_sleep_perform() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let models = vec![1, 2, 3];
    for model in models {
        tx.send(format!("sending {}", model)).await.unwrap();
    }
    tx2.send("Finished".to_string()).await.unwrap();
    while let Some(message) = rx.recv().await {
        do_sleep().await;
        println!("GOT == {}", message);
    }
}
#[tokio::main]
async fn main() {
    let res = handler().await;
    println!("{}", res);
    let result = do_sleep_perform().await;
    println!("{:?}", result);
    println!("{:?}", broadcase_channel().await);
}

async fn broadcase_channel() {
    use tokio::sync::broadcast;

    let (t, mut x) = broadcast::channel(1);
    let handle = tokio::spawn(async move {
        let mut value = 10;

        loop {
            value += 10;
            t.send(value).unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
            if value >= 100 {
                break;
            }
        }
    });
    let handle1 = tokio::spawn(async move {
        loop {
            let _result = x.recv().await;
            match _result {
                Ok(v) => {
                    println!("==============: {}", v);
                }
                Err(e) => {
                    println!("error is {}", e);
                    break;
                }
            }
        }
    });

   handle.await.unwrap();
   handle1.await.unwrap();
}
