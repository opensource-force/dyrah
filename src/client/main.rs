use std::{net::{SocketAddr, UdpSocket}, sync::mpsc::{self, Receiver, TryRecvError}, thread, time::{Duration, Instant, SystemTime}};

use renet::{transport::{ClientAuthentication, NetcodeClientTransport}, ConnectionConfig, DefaultChannel, RenetClient};

fn main() {
    let addr = "127.0.0.1:6668";
    let server_addr: SocketAddr = "127.0.0.1:6667".parse().unwrap();

    let mut client = RenetClient::new(ConnectionConfig::default());
    let socket = UdpSocket::bind(addr).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        server_addr,
        client_id,
        user_data: None,
        protocol_id: 7,
    };

    let mut transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let stdin_channel: Receiver<String> = spawn_stdin_channel();
    let mut last_updated = Instant::now();

    loop {
        let now = Instant::now();
        let duration = now - last_updated;
        last_updated = now;

        client.update(duration);
        transport.update(duration, &mut client).unwrap();

        if client.is_connected() {
            match stdin_channel.try_recv() {
                Ok(text) => client.send_message(DefaultChannel::ReliableOrdered, text.as_bytes().to_vec()),
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
            }

            while let Some(text) = client.receive_message(DefaultChannel::ReliableOrdered) {
                let text = String::from_utf8(text.into()).unwrap();
                println!("{}", text);
            }
        }

        transport.send_packets(&mut client).unwrap();
        thread::sleep(Duration::from_millis(50));
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer.trim_end().to_string()).unwrap();
    });
    rx
}