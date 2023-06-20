use std::net::IpAddr;
use std::time::Duration;

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


fn main() {
    let local_address = "0.0.0.0:0"; // Adresse locale du client
    let broadcast_address = "255.255.255.255:8080"; // Adresse de diffusion (broadcast)
    let timeout = Duration::from_secs(2); // Délai d'attente pour les réponses

    // Obtient l'adresse IP de la machine locale
    let local_ip = get_local_ip().expect("Impossible d'obtenir l'adresse IP locale");

    println!("Adresse IP locale: {}", local_ip);
}
