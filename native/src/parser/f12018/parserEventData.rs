use serde_derive::{Serialize};
use std::str;
use super::parserHeader::*;

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
            m_eventStringCode: m_eventStringCode.to_string()
        })
    )
);
