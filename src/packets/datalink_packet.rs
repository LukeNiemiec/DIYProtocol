use crate::utils::utils::PacketPointer;

use crate::packets::network_packet::Network;
use crate::packets::network_packet::IPv4;


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

	pub fn build(&self) -> Vec<u8> {
		match self {
			Datalink::EthernetII (eth_pack) => {
				Vec::<u8>::from(eth_pack)
			}
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

	pub fn build(&self) {
		
	}
	
	pub fn print(&self) {
		println!(
			"DESTINATION MAC: {:?}\nSOURCE MAC: {:?}\nETHERTYPE: {:?}",
			self.destination_addr,
			self.source_addr,
			self.get_ethertype(),
		)
	}

	pub fn get_ethertype(&self) -> String {
		String::from(format!("{:X}{:X}", self.ethertype[0], self.ethertype[1]))
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

impl From<&EthernetII> for Vec<u8> {
	fn from(item: &EthernetII) -> Self {
		let mut packet_bytes: Vec<u8> = vec![];

		packet_bytes.extend(&item.destination_addr);
		packet_bytes.extend(&item.source_addr);
		packet_bytes.extend(&item.ethertype);

		packet_bytes
	}
}

