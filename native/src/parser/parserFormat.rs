use nom::{peek, IResult};
use nom::number::complete::{le_u16};

named!(parser_packet_format<&[u8], u16>, peek!(le_u16));

pub fn get_packet_format(input: &[u8]) -> IResult<&[u8], u16> {
    match parser_packet_format(input) {
        IResult::Ok((remaining, packet_format)) => IResult::Ok((remaining, packet_format)),
        _e => panic!("Unable to parse")
    }
}
