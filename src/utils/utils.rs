pub trait ByteOperations {
	fn split_u8(c: u8, index: u8) -> (u8, u8) {
		(c >> index,  c & (8 - index).pow(2) - 1)	
	}

	fn concat_u4(a: u8, b: u8, index: u8) -> u8 {
		(a << (8 - index)) | b
	}

	fn concat_u8(a: u8, b: u8) -> u16 {
		((a as u16) << 8) | (b as u16)
	} 

	fn concat_u16(a: u16, b: u16) -> u32 {
		((a as u32) << 16) | (b as u32)
	}
}

impl ByteOperations for u8 {}

#[derive(Debug)]
pub struct PacketPointer(pub Vec<u8>);

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
