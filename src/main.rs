use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

fn main() {
    let local_address = "0.0.0.0:0"; // Adresse locale du client
    let timeout = Duration::from_secs(2); // Délai d'attente pour les réponses

    // Obtient l'adresse IP de la machine locale
    let local_ip = get_local_ip().expect("Impossible d'obtenir l'adresse IP locale");

    // Obtient le masque de sous-réseau de la machine locale
    let subnet_mask = get_subnet_mask().expect("Impossible d'obtenir le masque de sous-réseau");

    // Calcule l'adresse de diffusion (broadcast) en utilisant le masque de sous-réseau
    let broadcast_address = calculate_broadcast_address(local_ip, subnet_mask);
    println!("Adresse de diffusion (broadcast) : {}", broadcast_address);

    // Crée un socket UDP pour le client
    let client_socket = UdpSocket::bind(local_address).expect("Impossible de lier le socket du client");

    // Configure le socket pour permettre la diffusion (broadcast)
    client_socket.set_broadcast(true).expect("Impossible de configurer la diffusion (broadcast)");

    // Message à envoyer dans la requête de détection
    let message = "Server Detection Request";
    let buf = message.as_bytes();

    // Envoie la requête de détection en diffusion (broadcast)
    client_socket.send_to(buf, broadcast_address).expect("Impossible d'envoyer la requête de détection");

    // Tampon de réception
    let mut recv_buf = [0u8; 1024];

    // Durée de début de la recherche
    let start_time = Instant::now();

    println!("Recherche de serveurs UDP actifs sur le réseau local...");

    // Boucle d'attente des réponses pendant le délai spécifié
    while Instant::now() - start_time < timeout {
        match client_socket.recv_from(&mut recv_buf) {
            Ok((size, server_addr)) => {
                let response = String::from_utf8_lossy(&recv_buf[..size]);
                println!("Serveur trouvé : {} - Réponse : {}", server_addr, response);
            }
            Err(_) => {}
        }
    }

    println!("Recherche terminée.");
}

fn get_local_ip() -> Option<IpAddr> {
    // Obtient les interfaces réseau
    let interfaces = pnet::datalink::interfaces();

    // Recherche l'interface avec l'adresse IP non locale (127.0.0.1)
    for interface in interfaces {
        for ip_network in interface.ips {
            if !ip_network.ip().is_loopback() {
                return Some(ip_network.ip());
            }
        }
    }

    None
}

fn get_subnet_mask() -> Option<IpAddr> {
    // Obtient les interfaces réseau
    let interfaces = pnet::datalink::interfaces();

    // Recherche l'interface avec l'adresse IP non locale (127.0.0.1)
    for interface in interfaces {
        for ip_network in interface.ips {
            if !ip_network.ip().is_loopback() {
                return Some(ip_network.mask());
            }
        }
    }

    None
}

fn calculate_broadcast_address(ip: IpAddr, subnet_mask: IpAddr) -> SocketAddr {
    match (ip, subnet_mask) {
        (IpAddr::V4(ipv4), IpAddr::V4(mask)) => {
            let broadcast_ip = Ipv4Addr::from(u32::from(ipv4) | !u32::from(mask));
            SocketAddr::new(IpAddr::V4(broadcast_ip), 8000)
        }
        _ => panic!("Le masque de sous-réseau n'est pas une adresse IP version 4"),
    }
}
