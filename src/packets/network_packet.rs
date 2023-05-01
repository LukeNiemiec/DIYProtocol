use crate::utils::utils::PacketPointer;
use crate::utils::utils::ByteOperations;

use crate::packets::transport_packet::Transport;
use crate::packets::transport_packet::Tcp;


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

	pub fn build(&self) -> Vec<u8> {
		match self {
			Network::IPv4 (net_pack) => {
				Vec::<u8>::from(net_pack)
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

	pub fn gen_checksum(self) {
		
	}

	pub fn check_checksum(self) {
	
	}

	pub fn get_options(&mut self) {}

	pub fn print(&self) {
		println!("\n\nNETWORK: {:?}", self)
	}
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

impl From<&IPv4> for Vec<u8> {
	fn from(item: &IPv4) -> Self {
		let mut packet_bytes: Vec<u8> = vec![];
		
		packet_bytes.extend(vec![
			<u8>::concat_u4(item.version[0], item.ihl[0], 4),
		]);

		packet_bytes.extend(&item.tos);
		packet_bytes.extend(&item.length);
		packet_bytes.extend(&item.id);

		packet_bytes.extend(vec![
			<u8>::concat_u4(item.flags[0], item.frag_offset[0], 3),
			item.frag_offset[1],
		]);
		
		packet_bytes.extend(&item.ttl);
		packet_bytes.extend(&item.protocol);
		packet_bytes.extend(&item.checksum);
		packet_bytes.extend(&item.source_addr);
		packet_bytes.extend(&item.destination_addr);
		packet_bytes.extend(&item.options);

		packet_bytes
	}
}
