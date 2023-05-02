use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    // Allows multiple producers and consumers to all send and receive on a single channel
    let (tx, _rx) = broadcast::channel(10);

    // loop to handle 'infinite' amount of clients
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        // moves each client to his independant task (async block)
        tokio::spawn(async move {
            // read the socket
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);

            // stores each line
            let mut line = String::new();

            loop {
                // multiple concurrent async and work with the first one that gets a result
                tokio::select! {
                    // store the amount of bytes per line
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        tx.send((line.clone(), addr)).unwrap();
                        // because the read_line append each line it reads
                        line.clear();
                    }

                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        // avoid seeing the message two times when you are the sender
                        if addr != other_addr {
                            // the socket writes the line back to the client
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
