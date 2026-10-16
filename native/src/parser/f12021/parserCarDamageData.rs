use nom::number::complete::{le_u8, le_f32};
use serde_derive::{Serialize};
use super::parserHeader::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct CarDamageData {
    pub m_tyresWear: Vec<f32>,
    pub m_tyresDamage: Vec<u8>,
    pub m_brakesDamage: Vec<u8>,
    pub m_frontLeftWingDamage: u8,
    pub m_frontRightWingDamage: u8,
    pub m_rearWingDamage: u8,
    pub m_floorDamage: u8,
    pub m_diffuserDamage: u8,
    pub m_sidepodDamage: u8,
    pub m_drsFault: u8,
    pub m_gearBoxDamage: u8,
    pub m_engineDamage: u8,
    pub m_engineMGUHWear: u8,
    pub m_engineESWear: u8,
    pub m_engineCEWear: u8,
    pub m_engineICEWear: u8,
    pub m_engineMGUKWear: u8,
    pub m_engineTCWear: u8
}

named!(pub parse_car_damage<&[u8], CarDamageData>,
    do_parse!(
        m_tyresWear: count!(le_f32, 4) >>
        m_tyresDamage: count!(le_u8, 4) >>
        m_brakesDamage: count!(le_u8, 4) >>
        m_frontLeftWingDamage: le_u8 >>
        m_frontRightWingDamage: le_u8 >>
        m_rearWingDamage: le_u8 >>
        m_floorDamage: le_u8 >>
        m_diffuserDamage: le_u8 >>
        m_sidepodDamage: le_u8 >>
        m_drsFault: le_u8 >>
        m_gearBoxDamage: le_u8 >>
        m_engineDamage: le_u8 >>
        m_engineMGUHWear: le_u8 >>
        m_engineESWear: le_u8 >>
        m_engineCEWear: le_u8 >>
        m_engineICEWear: le_u8 >>
        m_engineMGUKWear: le_u8 >>
        m_engineTCWear: le_u8 >>
        (CarDamageData {
            m_tyresWear: m_tyresWear,
            m_tyresDamage: m_tyresDamage,
            m_brakesDamage: m_brakesDamage,
            m_frontLeftWingDamage: m_frontLeftWingDamage,
            m_frontRightWingDamage: m_frontRightWingDamage,
            m_rearWingDamage: m_rearWingDamage,
            m_floorDamage: m_floorDamage,
            m_diffuserDamage: m_diffuserDamage,
            m_sidepodDamage: m_sidepodDamage,
            m_drsFault: m_drsFault,
            m_gearBoxDamage: m_gearBoxDamage,
            m_engineDamage: m_engineDamage,
            m_engineMGUHWear: m_engineMGUHWear,
            m_engineESWear: m_engineESWear,
            m_engineCEWear: m_engineCEWear,
            m_engineICEWear: m_engineICEWear,
            m_engineMGUKWear: m_engineMGUKWear,
            m_engineTCWear: m_engineTCWear
        })
    )
);

named!(pub parse_car_damage_data<&[u8], Vec<CarDamageData>>,
    count!(parse_car_damage, 22)
);

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketCarDamageData {
    pub m_header: PacketHeader,
    pub m_carDamageData: Vec<CarDamageData>,
}

named!(pub parse_car_damage_data_packet<&[u8], PacketCarDamageData>,
    do_parse!(
        m_header: parse_header >>
        m_carDamageData: parse_car_damage_data >>
        (PacketCarDamageData {
            m_header: m_header,
            m_carDamageData: m_carDamageData
        })
    )
);
