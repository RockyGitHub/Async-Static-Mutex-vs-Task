use tokio::{sync::mpsc::Receiver, time::Instant};

pub async fn test_display(mut rx: Receiver<()>) {
    let mut start = Instant::now();
    let mut count: u64 = 0;
    loop {
        match rx.recv().await {
            Some(_) => {
                count += 1;
            }
            None => todo!(),
        }
        if count == 1 {
            start = Instant::now();
        }
        if count % 100 == 0 {
            let rate = (count as f64) / (start.elapsed().as_secs_f64());
            print!("\rNew entry rate: {}/second\t", rate);
        }
        //if start.elapsed() >= Duration::from_millis(1000) {
        //let rate = (count * 1000) / (start.elapsed().as_millis() as u64);
        //print!("\rNew entry rate: {}/second", rate);
        ////count = 0;
        //}
    }
}
