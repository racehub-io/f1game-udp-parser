use nom::number::complete::{le_u8, le_u16, le_u32};
use serde_derive::{Serialize};
use super::parserHeader::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct LapHistoryData {
    pub m_lapTimeInMS: u32,
    pub m_sector1TimeInMS: u16,
    pub m_sector2TimeInMS: u16,
    pub m_sector3TimeInMS: u16,
    pub m_lapValidBitFlags: u8
}

named!(pub parse_lap_history<&[u8], LapHistoryData>,
    do_parse!(
        m_lapTimeInMS: le_u32 >>
        m_sector1TimeInMS: le_u16 >>
        m_sector2TimeInMS: le_u16 >>
        m_sector3TimeInMS: le_u16 >>
        m_lapValidBitFlags: le_u8 >>
        (LapHistoryData {
            m_lapTimeInMS: m_lapTimeInMS,
            m_sector1TimeInMS: m_sector1TimeInMS,
            m_sector2TimeInMS: m_sector2TimeInMS,
            m_sector3TimeInMS: m_sector3TimeInMS,
            m_lapValidBitFlags: m_lapValidBitFlags
        })
    )
);

#[derive(Debug, PartialEq, Serialize)]
pub struct TyreStintsHistoryData {
    pub m_endLap: u8,
    pub m_tyreActualCompound: u8,
    pub m_tyreVisualCompound: u8
}

named!(pub parse_tyre_stint_history<&[u8], TyreStintsHistoryData>,
    do_parse!(
        m_endLap: le_u8 >>
        m_tyreActualCompound: le_u8 >>
        m_tyreVisualCompound: le_u8 >>
        (TyreStintsHistoryData {
            m_endLap: m_endLap,
            m_tyreActualCompound: m_tyreActualCompound,
            m_tyreVisualCompound: m_tyreVisualCompound
        })
    )
);

named!(pub parse_lap_history_data<&[u8], Vec<LapHistoryData>>,
    count!(parse_lap_history, 100)
);

named!(pub parse_tyre_stints_history_data<&[u8], Vec<TyreStintsHistoryData>>,
    count!(parse_tyre_stint_history, 8)
);

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketSessionHistoryData {
    pub m_header: PacketHeader,
    pub m_carIdx: u8,
    pub m_numLaps: u8,
    pub m_numTyreStints: u8,
    pub m_bestLapTimeLapNum: u8,
    pub m_bestSector1LapNum: u8,
    pub m_bestSector2LapNum: u8,
    pub m_bestSector3LapNum: u8,
    pub m_lapHistoryData: Vec<LapHistoryData>,
    pub m_tyreStintsHistoryData: Vec<TyreStintsHistoryData>,
}

named!(pub parse_session_history_data_packet<&[u8], PacketSessionHistoryData>,
    do_parse!(
        m_header: parse_header >>
        m_carIdx: le_u8 >>
        m_numLaps: le_u8 >>
        m_numTyreStints: le_u8 >>
        m_bestLapTimeLapNum: le_u8 >>
        m_bestSector1LapNum: le_u8 >>
        m_bestSector2LapNum: le_u8 >>
        m_bestSector3LapNum: le_u8 >>
        m_lapHistoryData: parse_lap_history_data >>
        m_tyreStintsHistoryData: parse_tyre_stints_history_data >>
        (PacketSessionHistoryData {
            m_header: m_header,
            m_carIdx: m_carIdx,
            m_numLaps: m_numLaps,
            m_numTyreStints: m_numTyreStints,
            m_bestLapTimeLapNum: m_bestLapTimeLapNum,
            m_bestSector1LapNum: m_bestSector1LapNum,
            m_bestSector2LapNum: m_bestSector2LapNum,
            m_bestSector3LapNum: m_bestSector3LapNum,
            m_lapHistoryData: m_lapHistoryData,
            m_tyreStintsHistoryData: m_tyreStintsHistoryData
        })
    )
);
