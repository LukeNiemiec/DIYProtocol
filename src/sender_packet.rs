use crate::packets::datalink_packet::Datalink;
// use crate::packets::datalink_packet::EthernetII;

use crate::packets::network_packet::Network;
// use crate::packets::network_packet::IPv4;

use crate::packets::transport_packet::Transport;
// use crate::packets::transport_packet::Tcp;

use crate::packets::application_packet::Application;
// use crate::packets::application_packet::Diy;

#[derive(Debug)]
pub struct PacketSender {
	pub datalink: Option<Datalink>,
	pub network: Option<Network>,
	pub transport: Option<Transport>,
	pub application: Option<Application>,
}

impl PacketSender {
	pub fn print(self) {
		self.datalink.unwrap().print();
		self.network.unwrap().print();
		self.transport.unwrap().print();
	}

	pub fn build(self) -> Vec<u8> {
		let mut packet: Vec<u8> = vec![];
	
		packet.extend(self.datalink.unwrap().build());
		packet.extend(self.network.unwrap().build());
		packet.extend(self.transport.unwrap().build());

		packet
	}

	pub fn fill_checksums(self) {
		
	}
}

impl Default for PacketSender {
	fn default() -> Self {
		PacketSender {
			datalink: None,
			network: None,
			transport: None,
			application: None,
		}
	}
}



