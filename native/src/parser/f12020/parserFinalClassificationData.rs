use nom::number::complete::{le_u8, le_f32, le_f64};
use serde_derive::{Serialize};
use super::parserHeader::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct FinalClassificationData {
    pub m_position: u8,
    pub m_numLaps: u8,
    pub m_gridPosition: u8,
    pub m_points: u8,
    pub m_numPitStops: u8,
    pub m_resultStatus: u8,
    pub m_bestLapTime: f32,
    pub m_totalRaceTime: f64,
    pub m_penaltiesTime: u8,
    pub m_numPenalties: u8,
    pub m_numTyreStints: u8,
    pub m_tyreStintsActual: Vec<u8>,
    pub m_tyreStintsVisual: Vec<u8>
}

named!(pub parse_final_classification<&[u8], FinalClassificationData>,
    do_parse!(
        m_position: le_u8 >>
        m_numLaps: le_u8 >>
        m_gridPosition: le_u8 >>
        m_points: le_u8 >>
        m_numPitStops: le_u8 >>
        m_resultStatus: le_u8 >>
        m_bestLapTime: le_f32 >>
        m_totalRaceTime: le_f64 >>
        m_penaltiesTime: le_u8 >>
        m_numPenalties: le_u8 >>
        m_numTyreStints: le_u8 >>
        m_tyreStintsActual: count!(le_u8, 8) >>
        m_tyreStintsVisual: count!(le_u8, 8) >>
        (FinalClassificationData {
            m_position: m_position,
            m_numLaps: m_numLaps,
            m_gridPosition: m_gridPosition,
            m_points: m_points,
            m_numPitStops: m_numPitStops,
            m_resultStatus: m_resultStatus,
            m_bestLapTime: m_bestLapTime,
            m_totalRaceTime: m_totalRaceTime,
            m_penaltiesTime: m_penaltiesTime,
            m_numPenalties: m_numPenalties,
            m_numTyreStints: m_numTyreStints,
            m_tyreStintsActual: m_tyreStintsActual,
            m_tyreStintsVisual: m_tyreStintsVisual
        })
    )
);

named!(pub parse_final_classifications<&[u8], Vec<FinalClassificationData>>,
    count!(parse_final_classification, 22)
);

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketFinalClassificationData {
    pub m_header: PacketHeader,
    pub m_numCars: u8,
    pub m_classificationData: Vec<FinalClassificationData>,
}

named!(pub parse_final_classification_data_packet<&[u8], PacketFinalClassificationData>,
    do_parse!(
        m_header: parse_header >>
        m_numCars: le_u8 >>
        m_classificationData: parse_final_classifications >>
        (PacketFinalClassificationData {
            m_header: m_header,
            m_numCars: m_numCars,
            m_classificationData: m_classificationData
        })
    )
);
