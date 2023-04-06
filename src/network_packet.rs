use crate::packet::PacketPointer;
use crate::packet::ByteOperations;

use crate::transport_packet::Transport;
use crate::transport_packet::Tcp;


#[derive(Debug, PartialEq)]
pub enum Network {
	IPv4(IPv4),
}

impl Network {
	pub fn parse(&mut self, packet: &mut PacketPointer) {
		match self {
			Network::IPv4 (net_pack) => {
				net_pack.parse(packet)
			},
		}
	}

	pub fn get_next_proto(&self) -> Option<Transport> {
		let proto = match self {
			Network::IPv4 (net_pack) => {
				net_pack.get_proto()
			}
		};
		
		match proto {
			6 => Some(Transport::Tcp(Tcp::default())),
			_ => None,
		}
	}

	pub fn print(&self) {
		match self {
			Network::IPv4 (net_pack) => {
				net_pack.print()
			},
		}
	}
}


#[derive(Debug, PartialEq)]
pub struct IPv4 {
	pub version: Vec<u8>,
	pub ihl: Vec<u8>, 
	pub tos: Vec<u8>, 
	pub length: Vec<u8>,
	pub id: Vec<u8>, 
	pub flags: Vec<u8>, 
	pub frag_offset: Vec<u8>, 
	pub ttl: Vec<u8>,
	pub protocol: Vec<u8>, 
	pub checksum: Vec<u8>,
	pub source_addr: Vec<u8>, 
	pub destination_addr: Vec<u8>, 
	pub options: Vec<u8>,   
}

impl Default for IPv4 {
	fn default() -> Self {
		IPv4 {
			version: vec![], //0.4
			ihl: vec![], //1.0
			tos: vec![], //1.0
			length: vec![], //2.0
			id: vec![], //2.0
			flags: vec![], //0.3
			frag_offset: vec![], //2.0 
			ttl: vec![], //1.0
			protocol: vec![], //1.0
			checksum: vec![], //2.0
			source_addr: vec![], //4.0
			destination_addr: vec![], //4.0
			options: vec![], //?
		}	
	}
}

impl IPv4 {
	pub fn parse(&mut self, packet: &mut PacketPointer) {
		let bc = (0.4, 1.0, 1.0, 2.0, 2.0, 0.3, 2.0, 1.0, 1.0, 2.0, 4.0, 4.0);

		self.version = packet.read(bc.0);
		self.ihl = packet.read(bc.1);
		self.tos = packet.read(bc.2);
		self.length = packet.read(bc.3);
		self.id = packet.read(bc.4); 
		self.flags = packet.read(bc.5);
		self.frag_offset = packet.read(bc.6); 
		self.ttl = packet.read(bc.7);
		self.protocol = packet.read(bc.8);
		self.checksum = packet.read(bc.9); 
		self.source_addr = packet.read(bc.10);
		self.destination_addr = packet.read(bc.11);

		if self.ihl[0] > 5 {
			println!("\n\nIPv4 has options\n")
		}
	}

	pub fn get_proto(&self) -> u8 {
		self.protocol[0] as u8
	}

	pub fn get_checksum() {}

	pub fn get_options(&mut self) {}

	pub fn print(&self) {
		println!("\n\nNETWORK: {:?}", self)
		// println!(": {}", self);
		// println!(": {}", self);
		// println!(": {}", self);
		// println!(": {}", self);
		// println!(": {}", self);
		// println!(": {}", self);

	}
}
