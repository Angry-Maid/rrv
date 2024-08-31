mod datatypes;
mod types;

use core::str;

pub use datatypes::*;
use log::info;
use nom::{
    bytes::complete::take,
    combinator::map_res,
    multi::count,
    number::complete::{le_u16, le_u32},
    sequence::{pair, tuple},
    IResult,
};
pub use types::*;

pub fn parse_replay_file(i: &[u8]) -> IResult<&[u8], &[u8]> {
    let (i, metadata_size) = le_u32(i)?;
    info!("{}", metadata_size);
    let (i, typemap_and_header) = take(metadata_size)(i)?;
    let (i, typemap) = parse_typemap(typemap_and_header)?;
    Ok((i, typemap_and_header))
}

pub fn parse_replay_string(i: &[u8]) -> IResult<&[u8], &str> {
    let (i, length) = le_u16(i)?;
    map_res(take(length), str::from_utf8)(i)
}

pub fn parse_datatype(i: &[u8]) -> IResult<&[u8], DataType> {
    let (i, (id, typename, version)) =
        tuple((le_u16, parse_replay_string, parse_replay_string))(i)?;

    Ok((
        i,
        DataType {
            id,
            typename,
            version,
        },
    ))
}

pub fn parse_typemap(i: &[u8]) -> IResult<&[u8], Typemap> {
    let (i, (typemap_ver, typemap_entries)) = pair(parse_replay_string, le_u16)(i)?;
    let (i, types) = count(parse_datatype, typemap_entries.into())(i)?;
    let typemap = Typemap {
        version: typemap_ver,
        entries: typemap_entries,
        types,
    };

    info!("{:#?}", typemap);

    Ok((i, typemap))
}

pub fn parse_header_blob<'a>(
    i: &'a [u8],
    typemap: &'a Typemap<'a>,
) -> IResult<&'a [u8], HeaderEntry<'a>> {
    let (i, id) = le_u16(i)?;
    let (i, blob) = match typemap.types.iter().find(|&&v| v.id == id).unwrap() {
        DataType {
            typename: "ReplayRecorder.Header", // string, bool
            ..
        } => {}
        DataType {
            typename: "ReplayRecorder.EndOfHeader",
            ..
        } => {}
        DataType {
            typename: "Vanilla.Metadata", // string
            version: "0.0.1",
            ..
        } => {}
        DataType {
            typename: "Vanilla.Metadata", // string, bool
            version: "0.0.2",
            ..
        } => {}
        DataType {
            typename: "Vanilla.Map.Geometry", // u8, u16, u32, f32 * 3 list, u16 list
            ..
        } => {}
        DataType {
            typename: "Vanilla.Map.Geometry.EOH",
            ..
        } => {}
        DataType {
            typename: "Vanilla.Map.Doors", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, u16, bool, u8, u8)
            ..
        } => {}
        DataType {
            typename: "Vanilla.Map.Ladders", // u16, (u8, f32 * 3, f16 * 3 + u8, f16)
            ..
        } => {}
        DataType {
            typename: "Vanilla.Map.Terminals", // u16, (i32, u8, f32 * 3, f16 * 3 + u8)
            ..
        } => {}
        DataType {
            typename:
                "Vanilla.Map.Generators"
                | "Vanilla.Map.DisinfectStations"
                | "Vanilla.Map.BulkheadControllers", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, u16)
            ..
        } => {}
        DataType {
            typename: "Vanilla.Map.ResourceContainers", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, u16, bool)
            ..
        } => {}
        DataType {
            typename: "Vanilla.Enemy.Spitters", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, f16)
            ..
        } => {}
        _ => {}
    };
}

pub fn parse_header<'a>(i: &'a [u8], typemap: &'a Typemap<'a>) -> IResult<&'a [u8], Header<'a>> {
    Ok((i, ()))
}
