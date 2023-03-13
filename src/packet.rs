use crate::datalink_packet::Ethernet;
use crate::datalink_packet::MacAddr;

pub struct Packet {
	pub packet: Vec<u8>,
	pub datalink: Option<Ethernet<Vec<u8>>>,
}

impl Packet {

	pub fn parse(&mut self) {
		self.datalink = self.get_ethernet();
	}

	pub fn get_ethernet(&mut self) -> Option<Ethernet<Vec<u8>>> {

		let eth_bytes = self.packet[0..21].to_vec();
		self.packet = self.packet.split_off(21);

		Some(Ethernet {
			preamble: eth_bytes[0..7].to_vec(),
			sfd: vec![eth_bytes[7]],

			destination_addr: MacAddr {
				address: eth_bytes[8..14].to_vec()
			},
			
			source_addr: MacAddr { 
				address: eth_bytes[14..20].to_vec()
			},
				
			ethertype: eth_bytes[20..].to_vec(),
		})
	}
}

impl Default for Packet {
	fn default() -> Self {
		Packet {
			packet: vec!(),
			datalink: None,
		}
	}
}
