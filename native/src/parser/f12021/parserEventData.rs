use nom::number::complete::{le_u8, le_u32, le_f32};
use serde_derive::{Serialize};
use std::str;
use super::parserHeader::*;

// #[derive(Debug, PartialEq, Serialize)]
// pub struct FastestLap {
//     pub vehicleIdx: u8,
//     pub lapTime: f32
// }
//
// named!(pub parse_fastest_lap<&[u8], FastestLap>,
//     do_parse!(
//         vehicleIdx: le_u8 >>
//         lapTime: le_f32 >>
//         (FastestLap {
//             vehicleIdx: vehicleIdx,
//             lapTime: lapTime
//         })
//     )
// );
//
// #[derive(Debug, PartialEq, Serialize)]
// pub struct Retirement {
//     pub vehicleIdx: u8
// }
//
// named!(pub parse_retirement<&[u8], Retirement>,
//     do_parse!(
//         vehicleIdx: le_u8 >>
//         (Retirement {
//             vehicleIdx: vehicleIdx
//         })
//     )
// );
//
// #[derive(Debug, PartialEq, Serialize)]
// pub struct TeamMateInPits {
//     pub vehicleIdx: u8
// }
//
// named!(pub parse_team_mate_in_pits<&[u8], TeamMateInPits>,
//     do_parse!(
//         vehicleIdx: le_u8 >>
//         (TeamMateInPits {
//             vehicleIdx: vehicleIdx
//         })
//     )
// );
//
// #[derive(Debug, PartialEq, Serialize)]
// pub struct RaceWinner {
//     pub vehicleIdx: u8
// }
//
// named!(pub parse_race_winner<&[u8], RaceWinner>,
//     do_parse!(
//         vehicleIdx: le_u8 >>
//         (RaceWinner {
//             vehicleIdx: vehicleIdx
//         })
//     )
// );
//
// #[derive(Debug, PartialEq, Serialize)]
// pub struct Penalty {
//     pub penaltyType: u8,
//     pub infringementType: u8,
//     pub vehicleIdx: u8,
//     pub otherVehicleIdx: u8,
//     pub time: u8,
//     pub lapNum: u8,
//     pub placesGained: u8
// }
//
// named!(pub parse_penalty<&[u8], Penalty>,
//     do_parse!(
//         penaltyType: le_u8 >>
//         infringementType: le_u8 >>
//         vehicleIdx: le_u8 >>
//         otherVehicleIdx: le_u8 >>
//         time: le_u8 >>
//         lapNum: le_u8 >>
//         placesGained: le_u8 >>
//         (Penalty {
//             penaltyType: penaltyType,
//             infringementType: infringementType,
//             vehicleIdx: vehicleIdx,
//             otherVehicleIdx: otherVehicleIdx,
//             time: time,
//             lapNum: lapNum,
//             placesGained: placesGained,
//         })
//     )
// );
//
// #[derive(Debug, PartialEq, Serialize)]
// pub struct SpeedTrap {
//     pub vehicleIdx: u8,
//     pub speed: f32,
//     pub overallFastestInSession: u8,
//     pub driverFastestInSession: u8
// }
//
// named!(pub parse_speed_trap<&[u8], SpeedTrap>,
//     do_parse!(
//         vehicleIdx: le_u8 >>
//         speed: le_f32 >>
//         overallFastestInSession: le_u8 >>
//         driverFastestInSession: le_u8 >>
//         (SpeedTrap {
//             vehicleIdx: vehicleIdx,
//             speed: speed,
//             overallFastestInSession: overallFastestInSession,
//             driverFastestInSession: driverFastestInSession
//         })
//     )
// );
//
// #[derive(Debug, PartialEq, Serialize)]
// pub struct StartLights {
//     pub numLights: u8
// }
//
// named!(pub parse_start_lights<&[u8], StartLights>,
//     do_parse!(
//         numLights: le_u8 >>
//         (StartLights {
//             numLights: numLights
//         })
//     )
// );
//
// #[derive(Debug, PartialEq, Serialize)]
// pub struct DriveThroughPenaltyServed {
//     pub vehicleIdx: u8
// }
//
// named!(pub parse_drive_through_penalty_served<&[u8], DriveThroughPenaltyServed>,
//     do_parse!(
//         vehicleIdx: le_u8 >>
//         (DriveThroughPenaltyServed {
//             vehicleIdx: vehicleIdx
//         })
//     )
// );
//
// #[derive(Debug, PartialEq, Serialize)]
// pub struct StopGoPenaltyServed {
//     pub vehicleIdx: u8
// }
//
// named!(pub parse_stop_go_penalty_served<&[u8], StopGoPenaltyServed>,
//     do_parse!(
//         vehicleIdx: le_u8 >>
//         (StopGoPenaltyServed {
//             vehicleIdx: vehicleIdx
//         })
//     )
// );
//
// #[derive(Debug, PartialEq, Serialize)]
// pub struct Flashback {
//     pub flashbackFrameIdentifier: u32,
//     pub flashbackSessionTime: f32
// }
//
// named!(pub parse_flashback<&[u8], Flashback>,
//     do_parse!(
//         flashbackFrameIdentifier: le_u32 >>
//         flashbackSessionTime: le_f32 >>
//         (Flashback {
//             flashbackFrameIdentifier: flashbackFrameIdentifier,
//             flashbackSessionTime: flashbackSessionTime
//         })
//     )
// );
//
// #[derive(Debug, PartialEq, Serialize)]
// pub struct ButtonStatus {
//     pub m_buttonStatus: u32,
// }
//
// named!(pub parse_button_status<&[u8], ButtonStatus>,
//     do_parse!(
//         m_buttonStatus: le_u32 >>
//         (ButtonStatus {
//             m_buttonStatus: m_buttonStatus
//         })
//     )
// );
//
// #[derive(Debug, Serialize)]
// pub enum EventDataDetails {
//     FastestLap(FastestLap),
//     Retirement(Retirement),
//     TeamMateInPits(TeamMateInPits),
//     RaceWinner(RaceWinner),
//     Penalty(Penalty),
//     SpeedTrap(SpeedTrap),
//     StartLights(StartLights),
//     DriveThroughPenaltyServed(DriveThroughPenaltyServed),
//     StopGoPenaltyServed(StopGoPenaltyServed),
//     Flashback(Flashback),
//     ButtonStatus(ButtonStatus)
// }
//
// #[derive(Debug, PartialEq, Serialize)]
// pub struct PacketEventData {
//     pub m_header: PacketHeader,
//     pub m_eventStringCode: String,
//     pub m_eventDetails: Option<EventDataDetails>
// }
//
// named!(pub parse_event_data_packet<&[u8], PacketEventData>,
//     do_parse!(
//         m_header: parse_header >>
//         m_eventStringCode: map_res!(peek!(4), str::from_utf8) >>
//         m_eventDetails: switch!(take!(4),
//             b"SSTA" => None |
//             b"SEND" => None |
//             b"FTLP" => !call(parse_fastest_lap) |
//             b"RTMT" => !call(parse_retirement) |
//             _ => None
//         ) >>
//         (PacketEventData {
//             m_header: m_header,
//             m_eventStringCode: m_eventStringCode.to_string(),
//             m_eventDetails: m_eventDetails
//         })
//     )
// );

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketEventData {
    pub m_header: PacketHeader,
    pub m_eventStringCode: String
}

named!(pub parse_event_data_packet<&[u8], PacketEventData>,
    do_parse!(
        m_header: parse_header >>
        m_eventStringCode: map_res!(take!(4), str::from_utf8) >>
        (PacketEventData {
            m_header: m_header,
            m_eventStringCode: m_eventStringCode.to_string(),
        })
    )
);
