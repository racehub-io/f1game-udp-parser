use nom::number::complete::{le_u8, le_i8, le_u16, le_f32};
use serde_derive::{Serialize};
use super::parserHeader::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct CarStatusData {
    pub m_tractionControl: u8,
    pub m_antiLockBrakes: u8,
    pub m_fuelMix: u8,
    pub m_frontBrakeBias: u8,
    pub m_pitLimiterStatus: u8,
    pub m_fuelInTank: f32,
    pub m_fuelCapacity: f32,
    pub m_fuelRemainingLaps: f32,
    pub m_maxRPM: u16,
    pub m_idleRPM: u16,
    pub m_maxGears: u8,
    pub m_drsAllowed: u8,
    pub m_drsActivationDistance: u16,
    pub m_actualTyreCompound: u8,
    pub m_visualTyreCompound: u8,
    pub m_tyresAgeLaps: u8,
    pub m_vehicleFiaFlags: i8,
    pub m_ersStoreEnergy: f32,
    pub m_ersDeployMode: u8,
    pub m_ersHarvestedThisLapMGUK: f32,
    pub m_ersHarvestedThisLapMGUH: f32,
    pub m_ersDeployedThisLap: f32,
    pub m_networkPaused: u8
}

named!(pub parse_car_status_data<&[u8], CarStatusData>,
    do_parse!(
        m_tractionControl: le_u8 >>
        m_antiLockBrakes: le_u8 >>
        m_fuelMix: le_u8 >>
        m_frontBrakeBias: le_u8 >>
        m_pitLimiterStatus: le_u8 >>
        m_fuelInTank: le_f32 >>
        m_fuelCapacity: le_f32 >>
        m_fuelRemainingLaps: le_f32 >>
        m_maxRPM: le_u16 >>
        m_idleRPM: le_u16 >>
        m_maxGears: le_u8 >>
        m_drsAllowed: le_u8 >>
        m_drsActivationDistance: le_u16 >>
        m_actualTyreCompound: le_u8 >>
        m_visualTyreCompound: le_u8 >>
        m_tyresAgeLaps: le_u8 >>
        m_vehicleFiaFlags: le_i8 >>
        m_ersStoreEnergy: le_f32 >>
        m_ersDeployMode: le_u8 >>
        m_ersHarvestedThisLapMGUK: le_f32 >>
        m_ersHarvestedThisLapMGUH: le_f32 >>
        m_ersDeployedThisLap: le_f32 >>
        m_networkPaused: le_u8 >>
        (CarStatusData {
            m_tractionControl: m_tractionControl,
            m_antiLockBrakes: m_antiLockBrakes,
            m_fuelMix: m_fuelMix,
            m_frontBrakeBias: m_frontBrakeBias,
            m_pitLimiterStatus: m_pitLimiterStatus,
            m_fuelInTank: m_fuelInTank,
            m_fuelCapacity: m_fuelCapacity,
            m_fuelRemainingLaps: m_fuelRemainingLaps,
            m_maxRPM: m_maxRPM,
            m_idleRPM: m_idleRPM,
            m_maxGears: m_maxGears,
            m_drsAllowed: m_drsAllowed,
            m_drsActivationDistance: m_drsActivationDistance,
            m_actualTyreCompound: m_actualTyreCompound,
            m_visualTyreCompound: m_visualTyreCompound,
            m_tyresAgeLaps: m_tyresAgeLaps,
            m_vehicleFiaFlags: m_vehicleFiaFlags,
            m_ersStoreEnergy: m_ersStoreEnergy,
            m_ersDeployMode: m_ersDeployMode,
            m_ersHarvestedThisLapMGUK: m_ersHarvestedThisLapMGUK,
            m_ersHarvestedThisLapMGUH: m_ersHarvestedThisLapMGUH,
            m_ersDeployedThisLap: m_ersDeployedThisLap,
            m_networkPaused: m_networkPaused
        })
    )
);

named!(pub parse_car_status_datas<&[u8], Vec<CarStatusData>>,
    count!(parse_car_status_data, 22)
);

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketCarStatusData {
    pub m_header: PacketHeader,
    pub m_carStatusData: Vec<CarStatusData>
}

named!(pub parse_car_status_data_packet<&[u8], PacketCarStatusData>,
    do_parse!(
        m_header: parse_header >>
        m_carStatusData: parse_car_status_datas >>
        (PacketCarStatusData {
            m_header: m_header,
            m_carStatusData: m_carStatusData
        })
    )
);
