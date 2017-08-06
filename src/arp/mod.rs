extern crate libc;
extern crate regex;
extern crate pnet;
extern crate ipnetwork;

use std::net::Ipv4Addr;
use self::ipnetwork::{IpNetwork, Ipv4Network};
use self::regex::Regex;
use self::pnet::util::{MacAddr};
use self::pnet::datalink::{self, NetworkInterface};
use self::pnet::datalink::{Channel, EthernetDataLinkSender};
use self::pnet::packet::ethernet::MutableEthernetPacket;
use self::pnet::packet::arp::MutableArpPacket;
use self::pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use self::pnet::packet::{Packet, MutablePacket};
use self::pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpOperation, ArpPacket};

const MAX_PROGRESS: u32 = 256;

struct ArpNetWork {
  interface: NetworkInterface,
  sender: Box<EthernetDataLinkSender>,
  ipv4_net: Ipv4Network,
  step: u32,
  progress: u32,
}

impl ArpNetWork {
  fn new(
    interface: NetworkInterface,
    ipv4_net: Ipv4Network,
    wanted_step: u32,
  ) -> ArpNetWork {
    let net_size = ipv4_net.size() as u32;
    let mut step = wanted_step;
    if (net_size / wanted_step) > MAX_PROGRESS {
      step = net_size / 256;
    }
    
    let (mut sender, _) = match datalink::channel(&interface, Default::default()) {
      Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
      Ok(_) => panic!("Unknown channel type"),
      Err(e) => panic!("Error happened {}", e),
    };

    let arp_net = ArpNetWork {
      interface: interface,
      sender: sender,
      ipv4_net: ipv4_net,
      step: step,
      progress: 0,
    };
    return arp_net;
  }

  fn run_step(&mut self) -> Vec<Ipv4Addr> {
    let mut ipv4_vec = Vec::<Ipv4Addr>::new();
    let net_size = self.ipv4_net.size() as u32;
    if self.progress < net_size {
      for idx in 0..self.step {
        let ipv4 = self.ipv4_net.nth(self.progress + idx);
        ipv4_vec.push(ipv4.unwrap());
      }
    }
    self.progress = self.progress + self.step;
    return ipv4_vec;
  }
}

fn enum_arp_itf() -> Vec<datalink::NetworkInterface> {
  let regex = Regex::new(r"^(?:docker|veth)").unwrap();
  return datalink::interfaces()
    .into_iter()
    .filter(|itf| {
      return !regex.is_match(&itf.name) &&
        ((libc::IFF_LOOPBACK as u32) & itf.flags == 0) &&
        ((libc::IFF_NOARP as u32) & itf.flags == 0);
    })
    .collect();
}

fn make_arp_queue(
  arp_itf_slice: &[datalink::NetworkInterface],
  wanted_step: u32
) -> Vec<ArpNetWork> {
  let mut arp_net_vec = Vec::<ArpNetWork>::new();
  for interface in arp_itf_slice {
    for ip_net in &interface.ips {
      if let &IpNetwork::V4(ipv4_net) = ip_net {
        let arp_net = ArpNetWork::new(interface.clone(), ipv4_net.clone(), wanted_step);
        arp_net_vec.push(arp_net);
      }
    }
  }
  return arp_net_vec;
}

fn send_arp_packet(
  interface: NetworkInterface,
  sender: &mut Box<EthernetDataLinkSender>,
  source_ip: Ipv4Addr,
  source_mac: MacAddr,
  target_ip: Ipv4Addr,
  //target_mac: MacAddr,
) {
  let target_mac = MacAddr::new(0xff, 0xff, 0xff, 0xff, 0xff, 0xff);

  let mut eth_buffer = [0u8; 42];
  let mut eth_packet = MutableEthernetPacket::new(&mut eth_buffer).unwrap();
  eth_packet.set_destination(target_mac);
  eth_packet.set_source(source_mac);
  eth_packet.set_ethertype(EtherTypes::Arp);

  let mut arp_buffer = [0u8; 28];
  let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();
  arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
  arp_packet.set_protocol_type(EtherTypes::Ipv4);
  arp_packet.set_hw_addr_len(6);
  arp_packet.set_proto_addr_len(4);
  arp_packet.set_operation(ArpOperations::Request);
  arp_packet.set_sender_hw_addr(source_mac);
  arp_packet.set_sender_proto_addr(source_ip);
  arp_packet.set_target_hw_addr(target_mac);
  arp_packet.set_target_proto_addr(target_ip);

  eth_packet.set_payload(arp_packet.packet_mut());
  sender.send_to(&eth_packet.to_immutable(), Some(interface));
}

pub fn send_arp() {
  let arp_itf_vec = enum_arp_itf();
  let mut arp_net_vec = make_arp_queue(&arp_itf_vec, 16);
  
  for arp_net in &mut arp_net_vec {
    let arp_ip_vec = arp_net.run_step();
    for arp_ip in arp_ip_vec {
      println!("{}", arp_ip);
      send_arp_packet(
        arp_net.interface.clone(),
        &mut arp_net.sender,
        Ipv4Addr::new(192, 168, 1, 5),
        arp_net.interface.mac.unwrap(),
        arp_ip
      );
    }
  }

  for interface in arp_itf_vec {
    let mac = interface.mac.map(|mac| mac.to_string()).unwrap_or("N/A".to_owned());
    println!("{}:", interface.name);
    println!("  index: {}", interface.index);
    println!("  flags: {}", interface.flags);
    println!("  MAC: {}", mac);
    println!("  IPs:");
    for ip in interface.ips {
      println!("    {:?}", ip);
    }
  }
}




fn handle_arp_packet(interface_name: &str, ethernet: &EthernetPacket) {
  if ethernet.get_ethertype() != EtherTypes::Arp {
    return;
  }
  let header = ArpPacket::new(ethernet.payload());
  if let Some(header) = header {
      println!("[{}]: ARP packet: {}({}) > {}({}); operation: {:?}",
                interface_name,
                ethernet.get_source(),
                header.get_sender_proto_addr(),
                ethernet.get_destination(),
                header.get_target_proto_addr(),
                header.get_operation());
  } else {
      println!("[{}]: Malformed ARP Packet", interface_name);
  }
}

pub fn recv_arp() {
  let arp_itf_vec = enum_arp_itf();
  let interface = &arp_itf_vec[0];

  let (_, mut rx) = match datalink::channel(interface, Default::default()) {
    Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
    Ok(_) => panic!("packetdump: unhandled channel type: {}"),
    Err(e) => panic!("packetdump: unable to create channel: {}", e),
  };

  let mut iter = rx.iter();
  loop {
    match iter.next() {
      Ok(packet) => handle_arp_packet(&interface.name[..], &packet),
      Err(e) => panic!("packetdump: unable to receive packet: {}", e),
    }
  }
}
