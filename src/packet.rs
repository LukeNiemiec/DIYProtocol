use crate::datalink_packet::Datalink;
use crate::datalink_packet::EthernetII;

use crate::network_packet::Network;
use crate::network_packet::IPv4;

use crate::transport_packet::Transport;
use crate::transport_packet::Tcp;

pub trait ByteOperations {
	fn split_u8(c: u8, index: u8) -> (u8, u8) {
		(c >> index,  c & (8 - index).pow(2) - 1)	
	}

	fn concat_u8(a: u8, b: u8) -> u16 {
		((a as u16) << 8) | (b as u16)
	} 

	fn concat_u16(a: u16, b: u16) -> u32 {
		((a as u32) << 16) | (b as u32)
	}
}

impl ByteOperations for u8 {}
impl ByteOperations for u16 {}


#[derive(Debug)]
pub struct PacketPointer(Vec<u8>);

impl PacketPointer {
	pub fn read(&mut self, bytes: f32) -> Vec<u8> {
		let mut read: Vec<u8> =  vec![];
		let index = bytes.floor() as usize;

		read.append(&mut self.0[0..index].to_vec());
		self.0 = self.0.split_off(index);
		
		let remainder = ((bytes - bytes.floor()) * 10.0) as u8;	

		if remainder != 0 {
			let (x, y) = <u8>::split_u8(self.0[0], remainder);

			self.0[0] = y;
			read.push(x);
		}

		read
	}

	pub fn init(&mut self, packet: Vec<u8>) {
		self.0 = packet;
	}
}


#[derive(Debug)]
pub struct Packet {
	pub packet: PacketPointer,
	pub datalink: Option<Datalink>,
	pub network: Option<Network>,
	pub transport: Option<Transport>,
	
}

impl Packet {

	pub fn print(self) {
		self.datalink.unwrap().print();
		self.network.unwrap().print();
		self.transport.unwrap().print();
	}

	pub fn parse(&mut self) {
		self.datalink = self.parse_datalink();

		if self.datalink != None {
			self.network = self.parse_network();

			if self.network != None {
				self.transport = self.parse_transport();
			}
		}
	}

	pub fn parse_datalink(&mut self) -> Option<Datalink> {		
		let mut datalink = Datalink::EthernetII(EthernetII::default());
		
		datalink.parse(&mut self.packet);
		
		Some(datalink)
	}

	pub fn parse_network(&mut self) -> Option<Network> {
		let dl = self.datalink.as_ref().unwrap();
		
		match dl.get_next_proto() {
			Some(mut network ) => {
				
				network.parse(&mut self.packet);
				
				Some(network)		
			},
			
			None => None,
		}
	}


	pub fn parse_transport(&mut self) -> Option<Transport> {
		let tp = self.network.as_ref().unwrap();

		match tp.get_next_proto() {
			Some(mut transport) => {

				transport.parse(&mut self.packet);

				Some(transport)
			},
			
			None => None,
		}
	}
}

impl Default for Packet {
	fn default() -> Self {
		Packet {
			packet: PacketPointer(vec![]),
			datalink: None,
			network: None,
			transport: None,
		}
	}
}



