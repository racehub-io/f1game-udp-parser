use nom::number::complete::{le_u8, le_u16, le_f32};
use serde_derive::{Serialize};
use super::parserHeader::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct LapData {
    pub m_lastLapTime: f32,
    pub m_currentLapTime: f32,
    pub m_sector1TimeInMS: u16,
    pub m_sector2TimeInMS: u16,
    pub m_bestLapTime: f32,
    pub m_bestLapNum: u8,
    pub m_bestLapSector1TimeInMS: u16,
    pub m_bestLapSector2TimeInMS: u16,
    pub m_bestLapSector3TimeInMS: u16,
    pub m_bestOverallSector1TimeInMS: u16,
    pub m_bestOverallSector1LapNum: u8,
    pub m_bestOverallSector2TimeInMS: u16,
    pub m_bestOverallSector2LapNum: u8,
    pub m_bestOverallSector3TimeInMS: u16,
    pub m_bestOverallSector3LapNum: u8,
    pub m_lapDistance: f32,
    pub m_totalDistance: f32,
    pub m_safetyCarDelta: f32,
    pub m_carPosition: u8,
    pub m_currentLapNum: u8,
    pub m_pitStatus: u8,
    pub m_sector: u8,
    pub m_currentLapInvalid: u8,
    pub m_penalties: u8,
    pub m_gridPosition: u8,
    pub m_driverStatus: u8,
    pub m_resultStatus: u8
}

named!(pub parse_lap_data<&[u8], LapData>,
    do_parse!(
        m_lastLapTime: le_f32 >>
        m_currentLapTime: le_f32 >>
        m_sector1TimeInMS: le_u16 >>
        m_sector2TimeInMS: le_u16 >>
        m_bestLapTime: le_f32 >>
        m_bestLapNum: le_u8 >>
        m_bestLapSector1TimeInMS: le_u16 >>
        m_bestLapSector2TimeInMS: le_u16 >>
        m_bestLapSector3TimeInMS: le_u16 >>
        m_bestOverallSector1TimeInMS: le_u16 >>
        m_bestOverallSector1LapNum: le_u8 >>
        m_bestOverallSector2TimeInMS: le_u16 >>
        m_bestOverallSector2LapNum: le_u8 >>
        m_bestOverallSector3TimeInMS: le_u16 >>
        m_bestOverallSector3LapNum: le_u8 >>
        m_lapDistance: le_f32 >>
        m_totalDistance: le_f32 >>
        m_safetyCarDelta: le_f32 >>
        m_carPosition: le_u8 >>
        m_currentLapNum: le_u8 >>
        m_pitStatus: le_u8 >>
        m_sector: le_u8 >>
        m_currentLapInvalid: le_u8 >>
        m_penalties: le_u8 >>
        m_gridPosition: le_u8 >>
        m_driverStatus: le_u8 >>
        m_resultStatus: le_u8 >>
        (LapData {
            m_lastLapTime: m_lastLapTime,
            m_currentLapTime: m_currentLapTime,
            m_sector1TimeInMS: m_sector1TimeInMS,
            m_sector2TimeInMS: m_sector2TimeInMS,
            m_bestLapTime: m_bestLapTime,
            m_bestLapNum: m_bestLapNum,
            m_bestLapSector1TimeInMS: m_bestLapSector1TimeInMS,
            m_bestLapSector2TimeInMS: m_bestLapSector2TimeInMS,
            m_bestLapSector3TimeInMS: m_bestLapSector3TimeInMS,
            m_bestOverallSector1TimeInMS: m_bestOverallSector1TimeInMS,
            m_bestOverallSector1LapNum: m_bestOverallSector1LapNum,
            m_bestOverallSector2TimeInMS: m_bestOverallSector2TimeInMS,
            m_bestOverallSector2LapNum: m_bestOverallSector2LapNum,
            m_bestOverallSector3TimeInMS: m_bestOverallSector3TimeInMS,
            m_bestOverallSector3LapNum: m_bestOverallSector3LapNum,
            m_lapDistance: m_lapDistance,
            m_totalDistance: m_totalDistance,
            m_safetyCarDelta: m_safetyCarDelta,
            m_carPosition: m_carPosition,
            m_currentLapNum: m_currentLapNum,
            m_pitStatus: m_pitStatus,
            m_sector: m_sector,
            m_currentLapInvalid: m_currentLapInvalid,
            m_penalties: m_penalties,
            m_gridPosition: m_gridPosition,
            m_driverStatus: m_driverStatus,
            m_resultStatus: m_resultStatus
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
