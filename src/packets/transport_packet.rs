use crate::utils::utils::PacketPointer;
use crate::utils::utils::ByteOperations;

use crate::packets::application_packet::Application;
use crate::packets::application_packet::Diy;


#[derive(Debug, PartialEq)]
pub enum Transport {
	Tcp(Tcp),
}

impl Transport {
	pub fn parse(&mut self, packet: &mut PacketPointer) {
		match self {
			Transport::Tcp (tran_pack) => {
				tran_pack.parse(packet)
			},
		}
	}

	pub fn build(&self) -> Vec<u8> {
		match self {
			Transport::Tcp (tran_pack) => {
				Vec::<u8>::from(tran_pack)
			},
		}
	}

	

	pub fn get_next_proto(&self) -> Option<Application> {
		let proto = match self {
			Transport::Tcp(tran_pack) => {
				tran_pack.get_dest()
			}
		};

		match proto {
			1212 => Some(Application::Diy(Diy::default())), //number for DIY protocol
			_ => None,
		}
	}
	
	pub fn print(&self) {
		match self {
			Transport::Tcp (tran_pack) => {
				tran_pack.print()
			},
		}
	}
}


#[derive(Debug, PartialEq)]
pub struct Tcp {
	pub source_port: Vec<u8>,
	pub destination_port: Vec<u8>,
	pub sequence: Vec<u8>,
	pub acknowledgement: Vec<u8>,
	pub offset: Vec<u8>,
	pub rsvd: Vec<u8>,
	pub flags: Vec<u8>,
	pub window: Vec<u8>,
	pub checksum: Vec<u8>,
	pub pointer: Vec<u8>,
	pub options: Vec<u8>, // if offset > 5
}

impl Tcp {
	pub fn parse(&mut self, packet: &mut PacketPointer) {
		let bc = (2.0,2.0,4.0,4.0,0.4,1.0,1.0,2.0,2.0,2.0);
	
		self.source_port = packet.read(bc.0);
		self.destination_port = packet.read(bc.1);
		self.sequence = packet.read(bc.2);
		self.acknowledgement = packet.read(bc.3);
		self.offset = packet.read(bc.4);
		self.rsvd = packet.read(bc.5);
		self.flags = packet.read(bc.6);
		self.window = packet.read(bc.7);
		self.checksum = packet.read(bc.8);
		self.pointer = packet.read(bc.9);

		if self.offset[0] > 5 {
			println!("\n\nTCP has options\n");
		}
	}

	pub fn get_src(&self) -> u16 {
		<u8>::concat_u8(self.source_port[0], self.source_port[1])
	}
	pub fn get_dest(&self) -> u16 {
		<u8>::concat_u8(self.destination_port[0], self.destination_port[1])
	}
	
	pub fn print(&self) {
		println!("\n\nTRANSPORT:");
		println!("SOURCE PORT: {:?}", self.get_src());
	 	println!("DESTINATION PORT: {:?}", self.get_dest());
	}
}

impl Default for Tcp {
	fn default() -> Self {
		Tcp {
			source_port: vec![],
			destination_port: vec![],
			sequence: vec![],
			acknowledgement: vec![],
			offset: vec![],
			rsvd: vec![],
			flags: vec![],
			window: vec![],
			checksum: vec![],
			pointer: vec![],
			options: vec![],
		}
	}
}


impl From<&Tcp> for Vec<u8> {
	fn from(item: &Tcp) -> Self {
		let mut packet_bytes: Vec<u8> = vec![];
		
		packet_bytes.extend(&item.source_port);
		packet_bytes.extend(&item.destination_port);
		packet_bytes.extend(&item.sequence);
		packet_bytes.extend(&item.acknowledgement);
		
		packet_bytes.extend(vec![
			<u8>::concat_u4(item.offset[0], item.rsvd[0], 4),
			item.flags[0],
		]);
		
		packet_bytes.extend(&item.window);
		packet_bytes.extend(&item.checksum);
		packet_bytes.extend(&item.pointer);
		packet_bytes.extend(&item.options);

		packet_bytes
	}
}
