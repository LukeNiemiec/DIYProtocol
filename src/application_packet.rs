use crate::packet::PacketPointer;


#[derive(Debug)]
pub enum Application {
	Diy(Diy), 
}

impl Application {
	pub fn parse(&mut self, packet: &mut PacketPointer) {
		match self {
			Application::Diy(app_pack) => {
				app_pack.parse(packet)
			}
		}
	}

	pub fn print(&self) {
		match self {
			Application::Diy(app_pack) => {
				app_pack.print()
			}
		}	
	}
}


#[derive(Debug)]
pub struct Diy {
	
}

impl Diy {
	pub fn parse(&mut self, _packet: &mut PacketPointer) {}
	
	pub fn print(&self) {
		println!("APPLICATION: {:?}", self)
	}
}

impl Default for Diy {
	fn default() -> Self {
		Diy {}
	}
}
