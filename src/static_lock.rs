use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use tokio::io;

use crate::results_displayer;

// create a new FOO object with defaults when the code starts up
lazy_static! {
    static ref FOO: Arc<Mutex<BAR>> = Arc::new(Mutex::new(BAR::default()));
}

#[derive(Default)]
pub struct BAR {
    pub handle: u64, // set during construction and will not change
    pub data_link_type: u8,
    pub devices: Arc<Mutex<HashMap<u64, FooooooooBar>>>, // hashmap of devices to get ip addresses
    pub listen_handle: Option<JoinHandle<()>>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct FooooooooBar {
    pub services_supported: HashSet<u32>,
    pub address: u64,
    pub device_id: u64,
}

#[tokio::main]
pub async fn run(client_thread_count: usize) {
    println!("Started test for locking static");

    let (tx, rx) = tokio::sync::mpsc::channel::<()>(1000000000);

    let handle = tokio::spawn(results_displayer::test_display(rx));

    for _ in 0..client_thread_count {
        let tx = tx.clone();
        tokio::task::spawn_blocking(move || sim_client(tx));
    }

    handle.await.unwrap();

}

fn sim_client(tx: Sender<()>) -> Result<(), io::Error> {
    loop {
        match FOO.lock() {
            Ok(foo) => match foo.devices.lock() {
                Ok(mut devices) => {
                    let device = FooooooooBar::default();
                    devices.insert(42, device);
                }
                Err(_) => todo!(),
            },
            Err(_) => todo!(),
        }
        match tx.blocking_send(()) {
            Ok(_) => (),
            Err(_) => println!("Not sent"),
        }
    }

    //Ok(())
}
