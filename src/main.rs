use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;
use k_mouse::search_server;

fn main() {
    let address = search_server(8000, Duration::from_secs(2));

    if address.is_none() {
        let local_address: SocketAddr = "0.0.0.0:8000".parse().expect("Impossible de parser l'adresse locale");

        // Crée un socket UDP pour le serveur
        let server_socket = UdpSocket::bind(local_address).expect("Impossible de lier le socket du serveur");

        println!("Serveur UDP démarré sur {}", local_address);

        // Tampon de réception
        let mut recv_buf = [0u8; 1024];

        loop {
            // Attend la réception d'un message
            let (size, client_addr) = server_socket.recv_from(&mut recv_buf)
                .expect("Erreur lors de la réception du message");

            let message = String::from_utf8_lossy(&recv_buf[..size]);
            println!("Message reçu de {} : {}", client_addr, message);

            // Répond au client
            let response = "Message reçu par le serveur";
            server_socket.send_to(response.as_bytes(), client_addr)
                .expect("Erreur lors de l'envoi de la réponse");
        }
    } else {
        println!("Connection au serveur: {}", address.unwrap());
    }
}
