use crate::packets::datalink_packet::Datalink;
use crate::packets::datalink_packet::EthernetII;

use crate::packets::network_packet::Network;
use crate::packets::network_packet::IPv4;

use crate::packets::transport_packet::Transport;
use crate::packets::transport_packet::Tcp;

use crate::packets::application_packet::Application;
use crate::packets::application_packet::Diy;

use crate::utils::utils::ByteOperations;
use crate::utils::utils::PacketPointer;


#[derive(Debug)]
pub struct PacketReciever {
	pub packet: PacketPointer,
	pub datalink: Option<Datalink>,
	pub network: Option<Network>,
	pub transport: Option<Transport>,
	pub application: Option<Application>,
}

impl PacketReciever {

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

				if self.transport != None {
					self.application = self.parse_application();
				}
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

	pub fn parse_application(&mut self) -> Option<Application> {
		let ap = self.transport.as_ref().unwrap();
		
		match ap.get_next_proto() {
			Some(mut application) => {

				application.parse(&mut self.packet);
				
				Some(application)
			}, 
			None => None,
		}
	}
}

impl Default for PacketReciever {
	fn default() -> Self {
		PacketReciever {
			packet: PacketPointer(vec![]),
			datalink: None,
			network: None,
			transport: None,
			application: None,
		}
	}
}



