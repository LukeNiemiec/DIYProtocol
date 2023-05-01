pub mod utils;
pub mod packets;
pub mod sender_packet;
pub mod reciever_packet;
pub mod network;

use pnet::datalink::{self};
use pnet::datalink::Channel::Ethernet;

use std::net::{IpAddr, Ipv4Addr};

use crate::network::NetworkConnection;


fn main() {
	let interfaces = datalink::interfaces();

	let interface = interfaces[1].clone();

	
	let source_ip = match &interface.ips[0].ip() {
		IpAddr::V4(v4_addr) => {v4_addr.octets().to_vec()},
		_ => todo!(),
		
		// IpAddr::V6(v6_addr) => {v6_addr.octets()},
	};

	let dest_ip = source_ip.clone();


	let source_mac = match &interface.mac {
		Some(addr) => addr.octets().to_vec(),
		None => vec![],
	};

	let dest_mac = source_mac.clone();


	let (transmiter, reciever) = match datalink::channel(&interface, Default::default()) {
		Ok(Ethernet(dt, dr)) => (dt, dr),
		Ok(_) => panic!("something is very wrong if this shows up!"),
		Err(e) => panic!("error: {:?}", e)
	};
	
	
	let network = NetworkConnection {
		interface,
		transmiter,
		reciever,
		source_ip,
		dest_ip,
		source_mac,
		dest_mac,
		length: vec![0,0],
	};

	network.send_packet()
}
