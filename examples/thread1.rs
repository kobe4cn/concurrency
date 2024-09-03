const NUM_PRODUCERS: usize = 4;
#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    id: usize,
    data: usize,
}

fn main() -> anyhow::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        std::thread::spawn(move || producer(i, tx));
    }
    drop(tx); // drop tx to signal consumers that no more messages will be sent

    let consumer = std::thread::spawn(move || {
        for msg in rx {
            println!("Received: {:?}", msg);
        }
        println!("Consumer exiting");
        42
    });

    let msg = consumer
        .join()
        .map_err(|e| anyhow::anyhow!("Consumer thread panicked {:?}", e))?;

    println!("Consumer returned: {}", msg);

    Ok(())
}

fn producer(id: usize, tx: std::sync::mpsc::Sender<Msg>) -> anyhow::Result<()> {
    loop {
        let msg = Msg::new(id, rand::random::<usize>());
        tx.send(msg)?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        std::thread::sleep(std::time::Duration::from_millis(sleep_time));
        if rand::random::<u8>() % 10 == 0 {
            println!("Producer {} exiting", id);
            break;
        }
    }

    Ok(())
}

impl Msg {
    fn new(id: usize, data: usize) -> Self {
        Msg { id, data }
    }
}
