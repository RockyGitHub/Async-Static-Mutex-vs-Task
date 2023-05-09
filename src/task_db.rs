use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    thread::JoinHandle,
    time::Duration,
};

use serde::{Deserialize, Serialize};
use tokio::{
    sync::{
        mpsc::{self, Sender},
        oneshot,
    },
    time::sleep,
};

use crate::results_displayer;

// Some structure that somewhat represents the Runtime
#[derive(Default)]
pub struct FOO {
    pub handle: u64,
    pub data_link_type: u8,
    // Device must be arc because we will want to "get" a Device and pass it along a channel, which could be amongst other threads, wilst still retaining it in the hashmap
    pub devices: HashMap<u64, Arc<Device>>,
    pub listen_handle: Option<JoinHandle<()>>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Device {
    pubservices_supported: HashSet<u32>,
    pub address: u64,
    pub device_id: u64,
}

/// Multiple different commands are multiplexed over a single channel.
enum Command {
    Get {
        key: u64,
        resp: Responder<Arc<Device>>,
    },
    Set {
        key: u64,
        val: Device,
        resp: Responder<()>,
    },
}

/// Provided by the requester and used by the manager task to send the command
/// response back to the requester.
type Responder<T> = oneshot::Sender<Option<T>>;

#[tokio::main]
pub async fn run(client_thread_count: usize) {
    println!("Started testing dedicated task based structure of hashmaps");
    // Create the same runtime structure as in the other example
    let mut foo: FOO = FOO::default();

    // Create the metric counter test thing
    let (metrics_tx, metrics_rx) = mpsc::channel::<()>(1000000000);
    tokio::spawn(results_displayer::test_display(metrics_rx));

    // tx channel is the channel any other thread or task can use to get or set data from the following manager task
    let (tx, mut rx) = mpsc::channel(1000000);
    let tx2 = tx.clone();
    let tx3 = tx.clone();

    let manager = tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = foo.devices.get(&key);
                    let res = Some(res.unwrap().clone());
                    // Ignore errors
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let val = Arc::new(val);
                    let _ = foo.devices.insert(key, val);
                    // Ignore errors
                    let _ = resp.send(Some(()));
                    match metrics_tx.send(()).await {
                        Ok(_) => (),
                        Err(_) => println!("metric count not sent"),
                    }
                }
            }
        }
        println!("exiting manager");
    });

    // TESTING
    // Spawn two tasks, one setting a value and other querying for key that was
    let t1 = tokio::spawn(test_set(tx2));
    let t2 = tokio::spawn(test_get(tx));

    t1.await.unwrap();
    t2.await.unwrap();

    // Start threads that will hammer on the manager task to run benchmark
    for _ in 0..client_thread_count {
        let tx = tx3.clone();
        tokio::spawn(sim_client(tx));
    }

    manager.await.unwrap();
}

async fn sim_client(tx: Sender<Command>) {
    //sleep(Duration::from_millis(400)).await;

    loop {
        // Constantly set data
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: 42,
            val: Device::default(),
            resp: resp_tx,
        };

        if tx.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }

        //await the response
        if let Err(err) = resp_rx.await {
            println!("err rxing response: [{}]", err)
        }
    }
}

async fn test_get(tx: Sender<Command>) {
    sleep(Duration::from_millis(500)).await;
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = Command::Get {
        key: 0x01,
        resp: resp_tx,
    };

    // Send the GET request
    if tx.send(cmd).await.is_err() {
        eprintln!("connection task shutdown");
        return;
    }

    // Await the response
    let res = resp_rx.await;
    println!("GOT (Get) = {:?}", res);
}

async fn test_set(tx: Sender<Command>) {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = Command::Set {
        key: 0x01,
        val: Device::default(),
        resp: resp_tx,
    };

    // Send the SET request
    if tx.send(cmd).await.is_err() {
        eprintln!("connection task shutdown");
        return;
    }

    // Await the response
    let res = resp_rx.await;
    println!("GOT (Set) = {:?}", res);
}
