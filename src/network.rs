use crate::sender_packet::PacketSender;
use crate::reciever_packet::PacketReciever;

use crate::packets::datalink_packet::Datalink;
use crate::packets::datalink_packet::EthernetII;

use crate::packets::network_packet::Network;
use crate::packets::network_packet::IPv4;

use crate::packets::transport_packet::Transport;
use crate::packets::transport_packet::Tcp;

use pnet::datalink::NetworkInterface;
use pnet::datalink::DataLinkSender;
use pnet::datalink::DataLinkReceiver;

pub struct NetworkConnection {
	pub interface: NetworkInterface,
	pub transmiter: Box<dyn DataLinkSender>,
	pub reciever: Box<dyn DataLinkReceiver>,
	pub source_ip: Vec<u8>,
	pub dest_ip: Vec<u8>,
	pub source_mac: Vec<u8>,
	pub dest_mac: Vec<u8>,
	pub length: Vec<u8>,
}

impl NetworkConnection {
	pub fn send_packet(self) {

		let mut pack = PacketSender::default();
	
		pack.datalink = Some(Datalink::EthernetII(
			EthernetII {
				destination_addr: self.dest_mac,
				source_addr: self.source_mac,
				ethertype: vec![0, 8], 
			}
		));

		
		// println!("\n{:?}\n\n", pack.datalink.as_ref().unwrap().build());
	
		pack.network = Some(Network::IPv4(
			IPv4 {
				version: vec![4], 
				ihl: vec![5], 
				tos: vec![0], 
				length: vec![0, 0],
				id: vec![0, 0], 
				flags: vec![0],
				frag_offset: vec![0, 0], 
				ttl: vec![40], 
				protocol: vec![6], 
				checksum: vec![0, 0], 
				source_addr: self.source_ip, 
				destination_addr: self.dest_ip, 
				options: vec![],
			}
		));

		pack.transport = Some(Transport::Tcp(
			Tcp {
				source_port: vec![4,188],
				destination_port: vec![4,188],
				sequence: vec![0, 0, 0, 0],
				acknowledgement: vec![0, 0, 0, 0],
				offset: vec![0],
				rsvd: vec![0],
				flags: vec![0],
				window: vec![255,255],
				checksum: vec![0,0],
				pointer: vec![0,0],
				options: vec![],
			}
		));


		println!("\n{:?}\n", pack.build());
	}

	pub fn recv_packet(mut self) {  
		'main: loop {
			match self.reciever.next() {
				Ok(packet) => {
					
					let mut pack = PacketReciever::default();
					
					pack.packet.init(packet.to_vec());				
					pack.parse();
					
					match pack.network {
						Some(Network::IPv4(ipv4_packet)) => {
							println!("{:?}", ipv4_packet);
							break 'main;
						},
						None => (),
					}
				}, 
	
				Err(e) => panic!("ERROR: {:?}", e),
			}
		}
	}
}
