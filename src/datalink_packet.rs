#[derive(Debug)]
pub struct MacAddr {
	pub address: Vec<u8>	
}

pub enum EtherType {
	IPv4,
	IPv6,
	Arp,
}

impl MacAddr {
	pub fn read(&self) -> String {
		self.address
		.iter()
		.map(|x| format!("{:X}", x))
		.collect::<Vec<String>>()
		.join("-") 	
	}
}

#[derive(Debug)]
pub struct Ethernet<T> {
	pub preamble: T,
	pub sfd: T,
	pub destination_addr: MacAddr,
	pub source_addr: MacAddr,
	pub ethertype: T,
}

impl<T> Ethernet<T> {
	pub fn print(&self) 
	where 
		T: std::fmt::Debug
	{
		
		println!("\n\nSOURCE MAC ADDRESS: {:?}", &self.source_addr.read());
		println!("DESTINATION MAC ADDRESS: {:?}", &self.destination_addr.read());
		println!("ETHERTYPE: {:?}", &self.ethertype);
		
	}
}
