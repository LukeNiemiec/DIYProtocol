pub mod packet;
pub mod datalink_packet;
pub mod network_packet;
pub mod transport_packet;

use pnet::datalink::{self};
use pnet::datalink::Channel::Ethernet;

use crate::packet::Packet;

use crate::transport_packet::Transport;

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
				pack.packet.init(packet.to_vec());

				// println!("\n\n{:?}\n\n", packet);
				
				pack.parse();
				
				match pack.transport {
					Some(_) => {
						pack.print();
						break 'main;
					},
					None => (),
				}
			}, 

			Err(e) => panic!("ERROR: {:?}", e),
		}
	}
	
	 
}
