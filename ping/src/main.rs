extern crate pnet;

use pnet::packet::icmp::{echo_request::MutableEchoRequestPacket, IcmpTypes};
use pnet::packet::Packet;
use pnet::transport::{icmp_packet_iter, transport_channel, TransportChannelType::Layer3};
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use std::io;

fn main() {
    println!("Please enter an IP Address:");

    let mut target_ip = String::new();

    io::stdin().read_line(&mut target_ip)
        .expect("Failed to read line");

    let target: IpAddr = target_ip.parse().expect("Invalid IP Address");

    // Creating a transport channel to send and receive ICMP packets.
    let (mut tx, mut rx) = match transport_channel(1024, Layer3(IcmpTypes::EchoRequest)) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => panic!("Error creating the transport channel: {}", e),
    };

    let mut buffer = [0u8; 64];
    let mut packet = MutableEchoRequestPacket::new(&mut buffer).unwrap();
    packet.set_sequence_number(1);
    packet.set_identifier(0);
    let checksum = pnet::packet::icmp::checksum(&packet.to_immutable());
    packet.set_checksum(checksum);

    println!("Sending ICMP echo request to {}", target);

    match tx.send_to(packet, target) {
        Ok(_) => println!("Packet sent."),
        Err(e) => println!("Failed to send packet: {}", e),
    }

    let (mut iterator, _) = icmp_packet_iter(&mut rx);
    let timeout = Duration::from_secs(1);

    match iterator.next_with_timeout(timeout) {
        Ok(Some((packet, _))) if packet.get_icmp_type() == IcmpTypes::EchoReply => {
            println!("Received ICMP echo reply from {}", target);
        }
        Ok(_) => println!("No ICMP echo reply received."),
        Err(e) => println!("Failed to receive packet: {}", e),
    }
}
