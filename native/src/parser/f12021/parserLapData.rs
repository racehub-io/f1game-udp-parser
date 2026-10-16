use nom::number::complete::{le_u8, le_u16, le_u32, le_f32};
use serde_derive::{Serialize};
use super::parserHeader::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct LapData {
    pub m_lastLapTimeInMS: u32,
    pub m_currentLapTimeInMS: u32,
    pub m_sector1TimeInMS: u16,
    pub m_sector2TimeInMS: u16,
    pub m_lapDistance: f32,
    pub m_totalDistance: f32,
    pub m_safetyCarDelta: f32,
    pub m_carPosition: u8,
    pub m_currentLapNum: u8,
    pub m_pitStatus: u8,
    pub m_numPitStops: u8,
    pub m_sector: u8,
    pub m_currentLapInvalid: u8,
    pub m_penalties: u8,
    pub m_warnings: u8,
    pub m_numUnservedDriveThroughPens: u8,
    pub m_numUnservedStopGoPens: u8,
    pub m_gridPosition: u8,
    pub m_driverStatus: u8,
    pub m_resultStatus: u8,
    pub m_pitLaneTimerActive: u8,
    pub m_pitLaneTimeInLaneInMS: u16,
    pub m_pitStopTimerInMS: u16,
    pub m_pitStopShouldServePen: u8
}

named!(pub parse_lap_data<&[u8], LapData>,
    do_parse!(
        m_lastLapTimeInMS: le_u32 >>
        m_currentLapTimeInMS: le_u32 >>
        m_sector1TimeInMS: le_u16 >>
        m_sector2TimeInMS: le_u16 >>
        m_lapDistance: le_f32 >>
        m_totalDistance: le_f32 >>
        m_safetyCarDelta: le_f32 >>
        m_carPosition: le_u8 >>
        m_currentLapNum: le_u8 >>
        m_pitStatus: le_u8 >>
        m_numPitStops: le_u8 >>
        m_sector: le_u8 >>
        m_currentLapInvalid: le_u8 >>
        m_penalties: le_u8 >>
        m_warnings: le_u8 >>
        m_numUnservedDriveThroughPens: le_u8 >>
        m_numUnservedStopGoPens: le_u8 >>
        m_gridPosition: le_u8 >>
        m_driverStatus: le_u8 >>
        m_resultStatus: le_u8 >>
        m_pitLaneTimerActive: le_u8 >>
        m_pitLaneTimeInLaneInMS: le_u16 >>
        m_pitStopTimerInMS: le_u16 >>
        m_pitStopShouldServePen: le_u8 >>
        (LapData {
            m_lastLapTimeInMS: m_lastLapTimeInMS,
            m_currentLapTimeInMS: m_currentLapTimeInMS,
            m_sector1TimeInMS: m_sector1TimeInMS,
            m_sector2TimeInMS: m_sector2TimeInMS,
            m_lapDistance: m_lapDistance,
            m_totalDistance: m_totalDistance,
            m_safetyCarDelta: m_safetyCarDelta,
            m_carPosition :m_carPosition,
            m_currentLapNum: m_currentLapNum,
            m_pitStatus: m_pitStatus,
            m_numPitStops: m_numPitStops,
            m_sector: m_sector,
            m_currentLapInvalid: m_currentLapInvalid,
            m_penalties: m_penalties,
            m_warnings: m_warnings,
            m_numUnservedDriveThroughPens: m_numUnservedDriveThroughPens,
            m_numUnservedStopGoPens: m_numUnservedStopGoPens,
            m_gridPosition: m_gridPosition,
            m_driverStatus: m_driverStatus,
            m_resultStatus: m_resultStatus,
            m_pitLaneTimerActive: m_pitLaneTimerActive,
            m_pitLaneTimeInLaneInMS: m_pitLaneTimeInLaneInMS,
            m_pitStopTimerInMS: m_pitStopTimerInMS,
            m_pitStopShouldServePen: m_pitStopShouldServePen
        })
    )
);

named!(pub parse_lap_datas<&[u8], Vec<LapData>>,
    count!(parse_lap_data, 22)
);

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketLapData {
    pub m_header: PacketHeader,
    pub m_lapData: Vec<LapData>,
}

named!(pub parse_lap_data_packet<&[u8], PacketLapData>,
    do_parse!(
        m_header: parse_header >>
        m_lapData: parse_lap_datas >>
        (PacketLapData {
            m_header: m_header,
            m_lapData: m_lapData
        })
    )
);
