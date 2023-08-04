use data::{ArmaStatus, Info, Player};
use web::SseService;

async fn get_arma_status() -> ArmaStatus {
    let local_ip = get_local_ip_address().expect("there to be a network interface");
    let query_port = 2303; // todo: configure

    let ip_port = format!("{}:{}", local_ip, query_port);

    let client = a2s::A2SClient::new().await.expect("to be able to create udp socket");
    let info = match client.info(&ip_port).await {
        Ok(info) => info,
        Err(_) => return ArmaStatus::Offline,
    };

    let players = match client.players(&ip_port).await {
        Ok(players) => players,
        Err(_) => return ArmaStatus::Offline,
    };

    let status = Info {
        name: info.name,
        map: info.map,
        mission: info.game,
        max_players: info.max_players,
        players: players
            .iter()
            .map(|p| Player {
                name: p.name.clone(),
                score: p.score,
                duration: p.duration as u64,
            })
            .collect(),
    };

    ArmaStatus::Online(status)
}

pub fn start(sse: SseService) {
    tokio::spawn(async move {
        loop {
            let arma_status = get_arma_status().await;
            match sse.push(&arma_status) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("sse error: {}", e); // TODO: Tracing
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });
}

fn get_local_ip_address() -> Option<String> {
    use local_ip_address::list_afinet_netifas;
    let network_interfaces = list_afinet_netifas();
    if let Ok(network_interfaces) = network_interfaces {
        for (_, ip) in network_interfaces.iter() {
            if ip.is_ipv4() && !ip.is_loopback() {
                return Some(ip.to_string());
            }
        }
    }
    None
}
