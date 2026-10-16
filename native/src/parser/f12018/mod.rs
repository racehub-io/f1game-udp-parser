pub mod parserInfo;
pub mod parserHeader;
pub mod parserMotionData;
pub mod parserSessionData;
pub mod parserLapData;
pub mod parserEventData;
pub mod parserParticipantsData;
pub mod parserCarSetupData;
pub mod parserCarTelemetryData;
pub mod parserCarStatusData;

use serde_derive::{Serialize};

#[derive(Debug, Serialize)]
pub struct PacketResult {
    pub packetType: String,
    pub packetData: PacketData
}

pub struct Packet;

impl Packet {
    pub fn parse(input: &[u8]) -> PacketResult {
        let (_, packet_info) = parserInfo::get_packet_info(&input).unwrap();
        match packet_info.m_packetId {
            0 => {
                let result = parserMotionData::parse_motion_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "Motion".to_string(),
                    packetData: PacketData::Motion(packet)
                }
            }
            1 => {
                let result = parserSessionData::parse_session_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "Session".to_string(),
                    packetData: PacketData::Session(packet)
                }
            }
            2 => {
                let result = parserLapData::parse_lap_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "LapData".to_string(),
                    packetData: PacketData::LapData(packet)
                }
            }
            3 => {
                let result = parserEventData::parse_event_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "EventD".to_string(),
                    packetData: PacketData::Event(packet)
                }
            }
            4 => {
                let result = parserParticipantsData::parse_participants_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "Participants".to_string(),
                    packetData: PacketData::Participants(packet)
                }
            }
            5 => {
                let result = parserCarSetupData::parse_car_setup_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "CarSetups".to_string(),
                    packetData: PacketData::CarSetups(packet)
                }
            }
            6 => {
                let result = parserCarTelemetryData::parse_car_telemetry_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "CarTelemetry".to_string(),
                    packetData: PacketData::CarTelemetry(packet)
                }
            }
            7 => {
                let result = parserCarStatusData::parse_car_status_data_packet(&input);
                let (_, packet) = result.unwrap();
                PacketResult {
                    packetType: "CarStatus".to_string(),
                    packetData: PacketData::CarStatus(packet)
                }
            }
            _ => panic!("Unknown packet type")
        }
    }
}

#[derive(Debug, Serialize)]
pub enum PacketData {
    Motion(parserMotionData::PacketMotionData),
    Session(parserSessionData::PacketSessionData),
    LapData(parserLapData::PacketLapData),
    Event(parserEventData::PacketEventData),
    Participants(parserParticipantsData::PacketParticipantsData),
    CarSetups(parserCarSetupData::PacketCarSetupData),
    CarTelemetry(parserCarTelemetryData::PacketCarTelemetryData),
    CarStatus(parserCarStatusData::PacketCarStatusData),
}
