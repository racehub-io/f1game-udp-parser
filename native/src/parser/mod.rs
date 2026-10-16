pub mod f12018;
pub mod f12019;
pub mod f12020;
pub mod f12021;
pub mod parserFormat;

use serde_derive::{Serialize};

#[derive(Debug, Serialize)]
pub struct PacketResult {
    packetVersion: String,
    packetType: String,
    packetData: PacketData
}

#[derive(Debug, Serialize)]
pub enum PacketData {
    F12018(f12018::PacketData),
    F12019(f12019::PacketData),
    F12020(f12020::PacketData),
    F12021(f12021::PacketData)
}

pub fn parse_packet(input: &[u8]) -> PacketResult {
    let (_, packet_format) = parserFormat::get_packet_format(&input).unwrap();
    match packet_format {
        2018 => {
            let packet = f12018::Packet::parse(&input);
            PacketResult {
                packetVersion: "F12018".to_string(),
                packetType: packet.packetType,
                packetData: PacketData::F12018(packet.packetData)
            }
        }
        2019 => {
            let packet = f12019::Packet::parse(&input);
            PacketResult {
                packetVersion: "F12019".to_string(),
                packetType: packet.packetType,
                packetData: PacketData::F12019(packet.packetData)
            }
        }
        2020 => {
            let packet = f12020::Packet::parse(&input);
            PacketResult {
                packetVersion: "F12020".to_string(),
                packetType: packet.packetType,
                packetData: PacketData::F12020(packet.packetData)
            }
        }
        2021 => {
            let packet = f12021::Packet::parse(&input);
            PacketResult {
                packetVersion: "F12021".to_string(),
                packetType: packet.packetType,
                packetData: PacketData::F12021(packet.packetData)
            }
        }
        _ => panic!("Unknown packet type")
    }
}
