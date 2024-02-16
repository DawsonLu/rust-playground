extern crate pnet;

use pnet::packet::icmp::{echo_request::MutableEchoRequestPacket, IcmpTypes};
use pnet::packet::{MutablePacket, Packet};
use pnet::transport::{icmp_packet_iter, transport_channel, TransportChannelType::Layer3};
use std::net::Ipv4Addr;
use std::str::FromStr;

fn main() {
    let protocol = Layer3(IcmpTypes::EchoRequest);
    let (mut tx, mut rx) = match transport_channel(1024, protocol) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => panic!("An error occurred when creating the transport channel: {}", e),
    };

    println!("Please enter an IP address to ping:");
    let mut ip_address = String::new();
    std::io::stdin().read_line(&mut ip_address).expect("Failed to read line");
    let ip_address = ip_address.trim(); // Remove newline character
    let ip = Ipv4Addr::from_str(ip_address).expect("Invalid IP address");

    let mut buffer = [0u8; 64];
    let mut packet = MutableEchoRequestPacket::new(&mut buffer).unwrap();
    packet.set_sequence_number(1);
    packet.set_identifier(0);
    packet.set_icmp_type(IcmpTypes::EchoRequest);
    packet.set_checksum(pnet::util::checksum(packet.packet(), 1));

    match tx.send_to(packet, ip) {
        Ok(_) => println!("Sent an echo request to {}", ip),
        Err(e) => println!("Failed to send echo request: {}", e),
    }

    let mut iter = icmp_packet_iter(&mut rx);
    let (packet, _) = iter.next().expect("Failed to receive packet");
    match packet.get_icmp_type() {
        IcmpTypes::EchoReply => println!("Received an echo reply from {}", ip),
        _ => println!("Received a different type of ICMP packet"),
    }
}
