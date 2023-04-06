use crate::packet::PacketPointer;
use crate::packet::ByteOperations;

use crate::network_packet::Network;
use crate::network_packet::IPv4;

#[derive(Debug, PartialEq)]
pub enum Datalink {
	EthernetII(EthernetII),
}

impl Datalink {
	pub fn parse(&mut self, packet: &mut PacketPointer) {
		match self {
			Datalink::EthernetII (eth_pack) => {
				eth_pack.parse(packet)
			}
		}
	}

	pub fn get_next_proto(&self) -> Option<Network> {
		match self {
			Datalink::EthernetII (eth_pack) => {
				match eth_pack.get_ethertype().as_str() {
					"80" => Some(Network::IPv4(IPv4::default())),
					&_ => None,
				}
			},
		}	
	}

	pub fn print(&self) {
		match self {
			Datalink::EthernetII (eth_pack) => {
				eth_pack.print()
			},
		}
	}
	
}

#[derive(Debug, PartialEq)]
pub struct EthernetII {
	pub destination_addr: Vec<u8>,
	pub source_addr: Vec<u8>,
	pub ethertype: Vec<u8>,
}

impl EthernetII {
	pub fn parse(&mut self, packet: &mut PacketPointer) {
		let bc = (6.0, 6.0, 2.0);
		
		self.destination_addr = packet.read(bc.0);
		self.source_addr = packet.read(bc.1);
		self.ethertype = packet.read(bc.2)
	}

	pub fn get_ethertype(&self) -> String {
		String::from(format!("{:X}{:X}", self.ethertype[0], self.ethertype[1]))
	}

	pub fn print(&self) {
		println!(
			"DESTINATION MAC: {:?}\nSOURCE MAC: {:?}\nETHERTYPE: {:?}",
			self.destination_addr,
			self.source_addr,
			self.get_ethertype(),
		)
	}
}

impl Default for EthernetII {
	fn default() -> Self {
		EthernetII {
			destination_addr: vec![],
			source_addr: vec![],
			ethertype: vec![],
		}
	}
}

