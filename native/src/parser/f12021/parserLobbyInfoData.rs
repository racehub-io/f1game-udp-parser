use nom::number::complete::{le_u8};
use serde_derive::{Serialize};
use std::str;
use super::parserHeader::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct LobbyInfoData {
    pub m_aiControlled: u8,
    pub m_teamId: u8,
    pub m_nationality: u8,
    pub m_name: String,
    pub m_carNumber: u8,
    pub m_readyStatus: u8
}

named!(pub parse_lobby_info<&[u8], LobbyInfoData>,
    do_parse!(
        m_aiControlled: le_u8 >>
        m_teamId: le_u8 >>
        m_nationality: le_u8 >>
        m_name: map_res!(take!(48), str::from_utf8) >>
        m_carNumber: le_u8 >>
        m_readyStatus: le_u8 >>
        (LobbyInfoData {
            m_aiControlled: m_aiControlled,
            m_teamId: m_teamId,
            m_nationality: m_nationality,
            m_name: m_name.trim_matches(char::from(0)).to_string(),
            m_carNumber: m_carNumber,
            m_readyStatus: m_readyStatus
        })
    )
);

named!(pub parse_lobby_infos<&[u8], Vec<LobbyInfoData>>,
    count!(parse_lobby_info, 22)
);

#[derive(Debug, PartialEq, Serialize)]
pub struct PacketLobbyInfoData {
    pub m_header: PacketHeader,
    pub m_numPlayers: u8,
    pub m_lobbyPlayers: Vec<LobbyInfoData>,
}

named!(pub parse_lobby_info_data_packet<&[u8], PacketLobbyInfoData>,
    do_parse!(
        m_header: parse_header >>
        m_numPlayers: le_u8 >>
        m_lobbyPlayers: parse_lobby_infos >>
        (PacketLobbyInfoData {
            m_header: m_header,
            m_numPlayers: m_numPlayers,
            m_lobbyPlayers: m_lobbyPlayers
        })
    )
);
