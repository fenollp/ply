use nom::*;

use Format;

fn to_format(fmt: &[u8]) -> Format {
    match fmt {
        "ascii" => Format::AsciiV1,
        "binary_little_endian" => Format::BinaryLittleEndianV1,
        "binary_big_endian" => Format::BinaryBigEndianV1,
    }
}

named!(magic_number, preceded!(tag_s!("ply"), line_ending));
named!(format(&[u8]) -> Format, do_parse!(
    tag_s!("format") >>
        space >>
        fmt: map_res!(alt!(tag_s!("ascii") |
                           tag_s!("binary_little_endian") |
                           tag_s!("binary_big_endian")
        ), to_format) >>
        space >>
        tag_s!("1.0") >>
        line_ending >>
        ( fmt )
));

named!(ply(&[u8]) -> Format, do_parse!(
    magic_number >>
        fmt: format >>
        ( fmt )
));

#[test]
fn read_magic_number() {
    assert_eq!(magic_number(&b"ply\nformat ascii 1.0\n"[..]), IResult::Done(&b""[..], Format::AsciiV1));
}
