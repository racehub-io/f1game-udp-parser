use nom::number::complete::{le_i8, le_u8, le_u16, le_u32, le_f32};
use serde_derive::{Serialize};
use super::parserHeader::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct MarshalZone {
    pub m_zoneStart: f32,
    pub m_zoneFlag: i8
}

named!(pub parse_mashall_zone<&[u8], MarshalZone>,
    do_parse!(
        m_zoneStart: le_f32 >>
        m_zoneFlag: le_i8 >>
        (MarshalZone {
            m_zoneStart: m_zoneStart,
            m_zoneFlag: m_zoneFlag
        })
    )
);

named!(pub parse_marshall_zones<&[u8], Vec<MarshalZone>>,
    count!(parse_mashall_zone, 21)
);

#[derive(Debug, PartialEq, Serialize)]
pub struct WeatherForecastSample {
    pub m_sessionType: u8,
    pub m_timeOffset: u8,
    pub m_weather: u8,
    pub m_trackTemperature: i8,
    pub m_trackTemperatureChange: i8,
    pub m_airTemperature: i8,
    pub m_airTemperatureChange: i8,
    pub m_rainPercentage: u8
}

named!(pub parse_weather_forecast_sample<&[u8], WeatherForecastSample>,
    do_parse!(
        m_sessionType: le_u8 >>
        m_timeOffset: le_u8 >>
        m_weather: le_u8 >>
        m_trackTemperature: le_i8 >>
        m_trackTemperatureChange: le_i8 >>
        m_airTemperature: le_i8 >>
        m_airTemperatureChange: le_i8 >>
        m_rainPercentage: le_u8 >>
        (WeatherForecastSample {
            m_sessionType: m_sessionType,
            m_timeOffset: m_timeOffset,
            m_weather: m_weather,
            m_trackTemperature: m_trackTemperature,
            m_trackTemperatureChange: m_trackTemperatureChange,
            m_airTemperature: m_airTemperature,
            m_airTemperatureChange: m_airTemperatureChange,
            m_rainPercentage: m_rainPercentage
        })
    )
);

named!(pub parse_weather_forecast_samples<&[u8], Vec<WeatherForecastSample>>,
    count!(parse_weather_forecast_sample, 56)
);

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketSessionData {
    pub m_header: PacketHeader,
    pub m_weather: u8,
    pub m_trackTemperature: i8,
    pub m_airTemperature: i8,
    pub m_totalLaps: u8,
    pub m_trackLength: u16,
    pub m_sessionType: u8,
    pub m_trackId: i8,
    pub m_formula: u8,
    pub m_sessionTimeLeft: u16,
    pub m_sessionDuration: u16,
    pub m_pitSpeedLimit: u8,
    pub m_gamePaused: u8,
    pub m_isSpectating: u8,
    pub m_spectatorCarIndex: u8,
    pub m_sliProNativeSupport: u8,
    pub m_numMarshalZones: u8,
    pub m_marshalZones: Vec<MarshalZone>,
    pub m_safetyCarStatus: u8,
    pub m_networkGame: u8,
    m_weatherForecastSamples: Vec<WeatherForecastSample>,
    pub m_forecastAccuracy: u8,
    pub m_aiDifficulty: u8,
    pub m_seasonLinkIdentifier: u32,
    pub m_weekendLinkIdentifier: u32,
    pub m_sessionLinkIdentifier: u32,
    pub m_pitStopWindowIdealLap: u8,
    pub m_pitStopWindowLatestLap: u8,
    pub m_pitStopRejoinPosition: u8,
    pub m_steeringAssist: u8,
    pub m_brakingAssist: u8,
    pub m_gearboxAssist: u8,
    pub m_pitAssist: u8,
    pub m_pitReleaseAssist: u8,
    pub m_ERSAssist: u8,
    pub m_DRSAssist: u8,
    pub m_dynamicRacingLine: u8,
    pub m_dynamicRacingLineType: u8,
}

named!(pub parse_session_data_packet<&[u8], PacketSessionData>,
    do_parse!(
        m_header: parse_header >>
        m_weather: le_u8 >>
        m_trackTemperature: le_i8 >>
        m_airTemperature: le_i8 >>
        m_totalLaps: le_u8 >>
        m_trackLength: le_u16 >>
        m_sessionType: le_u8 >>
        m_trackId: le_i8 >>
        m_formula: le_u8 >>
        m_sessionTimeLeft: le_u16 >>
        m_sessionDuration: le_u16 >>
        m_pitSpeedLimit: le_u8 >>
        m_gamePaused: le_u8 >>
        m_isSpectating: le_u8 >>
        m_spectatorCarIndex: le_u8 >>
        m_sliProNativeSupport: le_u8 >>
        m_numMarshalZones: le_u8 >>
        m_marshalZones: parse_marshall_zones >>
        m_safetyCarStatus: le_u8 >>
        m_networkGame: le_u8 >>
        m_weatherForecastSamples: parse_weather_forecast_samples >>
        m_forecastAccuracy: le_u8 >>
        m_aiDifficulty: le_u8 >>
        m_seasonLinkIdentifier: le_u32 >>
        m_weekendLinkIdentifier: le_u32 >>
        m_sessionLinkIdentifier: le_u32 >>
        m_pitStopWindowIdealLap: le_u8 >>
        m_pitStopWindowLatestLap: le_u8 >>
        m_pitStopRejoinPosition: le_u8 >>
        m_steeringAssist: le_u8 >>
        m_brakingAssist: le_u8 >>
        m_gearboxAssist: le_u8 >>
        m_pitAssist: le_u8 >>
        m_pitReleaseAssist: le_u8 >>
        m_ERSAssist: le_u8 >>
        m_DRSAssist: le_u8 >>
        m_dynamicRacingLine: le_u8 >>
        m_dynamicRacingLineType: le_u8 >>
        (PacketSessionData {
            m_header: m_header,
            m_weather: m_weather,
            m_trackTemperature: m_trackTemperature,
            m_airTemperature: m_airTemperature,
            m_totalLaps: m_totalLaps,
            m_trackLength: m_trackLength,
            m_sessionType: m_sessionType,
            m_trackId: m_trackId,
            m_formula: m_formula,
            m_sessionTimeLeft: m_sessionTimeLeft,
            m_sessionDuration: m_sessionDuration,
            m_pitSpeedLimit: m_pitSpeedLimit,
            m_gamePaused: m_gamePaused,
            m_isSpectating: m_isSpectating,
            m_spectatorCarIndex: m_spectatorCarIndex,
            m_sliProNativeSupport: m_sliProNativeSupport,
            m_numMarshalZones: m_numMarshalZones,
            m_marshalZones: m_marshalZones,
            m_safetyCarStatus: m_safetyCarStatus,
            m_networkGame: m_networkGame,
            m_weatherForecastSamples: m_weatherForecastSamples,
            m_forecastAccuracy: m_forecastAccuracy,
            m_aiDifficulty: m_aiDifficulty,
            m_seasonLinkIdentifier: m_seasonLinkIdentifier,
            m_weekendLinkIdentifier: m_weekendLinkIdentifier,
            m_sessionLinkIdentifier: m_sessionLinkIdentifier,
            m_pitStopWindowIdealLap: m_pitStopWindowIdealLap,
            m_pitStopWindowLatestLap: m_pitStopWindowLatestLap,
            m_pitStopRejoinPosition: m_pitStopRejoinPosition,
            m_steeringAssist: m_steeringAssist,
            m_brakingAssist: m_brakingAssist,
            m_gearboxAssist: m_gearboxAssist,
            m_pitAssist: m_pitAssist,
            m_pitReleaseAssist: m_pitReleaseAssist,
            m_ERSAssist: m_ERSAssist,
            m_DRSAssist: m_DRSAssist,
            m_dynamicRacingLine: m_dynamicRacingLine,
            m_dynamicRacingLineType: m_dynamicRacingLineType,
        })
    )
);
