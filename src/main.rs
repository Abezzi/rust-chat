use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // loop to handle 'infinite' amount of clients
    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        // moves each client to his independant task (async block)
        tokio::spawn(async move {
            // read the socket
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);

            // stores each line
            let mut line = String::new();

            loop {
                // store the amount of bytes per line
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                // if the bytes read is equal to 0 means that is at the EOF
                if bytes_read == 0 {
                    break;
                }
                // the socket writes the line back to the client
                writer.write_all(line.as_bytes()).await.unwrap();
                // because the read_line append each line it reads
                line.clear();
            }
        });
    }
}
