use crate::packet::PacketPointer;
use crate::packet::ByteOperations;

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
		// println!(": {}", self);
		// println!(": {}", self);
		// println!(": {}", self);
		// println!(": {}", self);
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
