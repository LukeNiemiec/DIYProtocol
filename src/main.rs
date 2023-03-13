pub mod packet;
pub mod datalink_packet;

use pnet::datalink::{self};
use pnet::datalink::Channel::Ethernet;

use crate::packet::Packet;

fn _print_type<T>(_: &T) {
	println!("{}", std::any::type_name::<T>());
}

fn main() {

	let interface = datalink::interfaces();
	println!("\ncurrent interface: {:?}\n", &interface[1].name);  

	let (mut _dt, mut dr) = match datalink::channel(&interface[1], Default::default()) {
		Ok(Ethernet(_dt, dr)) => (_dt, dr),
		Ok(_) => panic!("something is very wrong if this shows up!"),
		Err(e) => panic!("error: {:?}", e)
	};
	        


	'main: loop {
		match dr.next() {
			Ok(packet) => {
				
				let mut pack = Packet::default();
				pack.packet = packet.to_vec();

				println!("{:?}", pack.packet);
				
				pack.parse();
				pack.datalink.unwrap().print();
				
				break 'main;
			}
			
			Err(e) => println!("error: {:?}", e)
		}
	}
	
	 
}
