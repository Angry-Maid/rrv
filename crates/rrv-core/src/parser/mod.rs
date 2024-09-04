mod datatypes;
mod types;

use core::str;

pub use datatypes::*;
use glam::{DQuat, Vec3};
use log::info;
use nom::{
    bytes::complete::take,
    combinator::map_res,
    multi::count,
    number::complete::{le_f32, le_i32, le_u16, le_u32, le_u8},
    sequence::{pair, tuple},
    IResult,
};
pub use types::*;

pub fn parse_replay_file(i: &[u8]) -> IResult<&[u8], &[u8]> {
    let (i, metadata_size) = le_u32(i)?;
    let (i, typemap_and_header_bytes) = take(metadata_size)(i)?;
    let (leftover, (typemap, header)) = parse_typemap_and_header(typemap_and_header_bytes)?;

    info!("{:#?}", typemap);
    info!("{:#?}", header);

    Ok((i, leftover))
}

pub fn parse_replay_string(i: &[u8]) -> IResult<&[u8], &str> {
    let (i, length) = le_u16(i)?;
    map_res(take(length), str::from_utf8)(i)
}

pub fn parse_replay_bool(i: &[u8]) -> IResult<&[u8], bool> {
    let (i, val) = le_u8(i)?;

    Ok((i, val > 0))
}

pub fn parse_vec3(i: &[u8]) -> IResult<&[u8], Vec3> {
    let (i, (x, y, z)) = tuple((le_f32, le_f32, le_f32))(i)?;
    Ok((i, Vec3::new(x, y, z)))
}

pub fn le_f16(i: &[u8]) -> IResult<&[u8], f16> {
    let (i, bits) = le_u16(i)?;
    Ok((i, f16::from_bits(bits)))
}

pub fn parse_half_quat(i: &[u8]) -> IResult<&[u8], DQuat> {
    let (i, (idx, a, b, c)) = tuple((le_u8, le_f16, le_f16, le_f16))(i)?;
    let quat = match idx {
        0 => {
            let (y, z, w): (f64, f64, f64) = (a.into(), b.into(), c.into());
            let x = f64::sqrt((1. - y.powi(2) - z.powi(2) - w.powi(2)).clamp(0., 1.));
            DQuat::from_xyzw(x, y, z, w)
        }
        1 => {
            let (x, z, w): (f64, f64, f64) = (a.into(), b.into(), c.into());
            let y = f64::sqrt((1. - x.powi(2) - z.powi(2) - w.powi(2)).clamp(0., 1.));
            DQuat::from_xyzw(x, y, z, w)
        }
        2 => {
            let (x, y, w): (f64, f64, f64) = (a.into(), b.into(), c.into());
            let z = f64::sqrt((1. - x.powi(2) - y.powi(2) - w.powi(2)).clamp(0., 1.));
            DQuat::from_xyzw(x, y, z, w)
        }
        3 => {
            let (x, y, z): (f64, f64, f64) = (a.into(), b.into(), c.into());
            let w = f64::sqrt((1. - x.powi(2) - y.powi(2) - z.powi(2)).clamp(0., 1.));
            DQuat::from_xyzw(x, y, z, w)
        }
        _ => unreachable!(),
    };
    Ok((i, quat))
}

pub fn parse_commons(i: &[u8]) -> IResult<&[u8], Common> {
    let (i, (id, dimension, position, rotation)) =
        tuple((le_i32, le_u8, parse_vec3, parse_half_quat))(i)?;

    Ok((
        i,
        Common {
            id: Some(id),
            dimension,
            position,
            rotation,
        },
    ))
}

pub fn parse_commons_no_id(i: &[u8]) -> IResult<&[u8], Common> {
    let (i, (dimension, position, rotation)) = tuple((le_u8, parse_vec3, parse_half_quat))(i)?;

    Ok((
        i,
        Common {
            id: None,
            dimension,
            position,
            rotation,
        },
    ))
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

pub fn parse_typemap_and_header(i: &[u8]) -> IResult<&[u8], (Typemap, Header)> {
    let (i, (typemap_ver, typemap_entries)) = pair(parse_replay_string, le_u16)(i)?;
    let (i, types) = count(parse_datatype, typemap_entries.into())(i)?;

    let typemap = Typemap {
        version: typemap_ver,
        entries: typemap_entries,
        types,
    };

    let mut header = Header::default();
    let mut commons = Commons::new();

    loop {
        let (i, id) = le_u16(i)?;
        let i = match typemap.types.iter().find(|&v| v.id == id).unwrap() {
            DataType {
                typename: "ReplayRecorder.Header", // string, bool
                ..
            } => {
                let (i, (version, master)) = pair(parse_replay_string, parse_replay_bool)(i)?;
                header.replay_header = Some(ReplayHeader { version, master });
                i
            }
            DataType {
                typename: "ReplayRecorder.EndOfHeader",
                ..
            } => i,
            DataType {
                typename: "Vanilla.Metadata", // string
                version: "0.0.1",
                ..
            } => {
                let (i, version) = parse_replay_string(i)?;
                header.metadata = Some(Metadata {
                    version,
                    compatability_old_dc: None,
                });
                i
            }
            DataType {
                typename: "Vanilla.Metadata", // string, bool
                version: "0.0.2",
                ..
            } => {
                let (i, (version, compat)) = pair(parse_replay_string, parse_replay_bool)(i)?;
                header.metadata = Some(Metadata {
                    version,
                    compatability_old_dc: Some(compat),
                });
                i
            }
            DataType {
                typename: "Vanilla.Map.Geometry", // u8, u16, u32, f32 * 3 list, u16 list
                ..
            } => {
                let (i, (dimension, num_vert, num_idx)) = tuple((le_u8, le_u16, le_u32))(i)?;
                let (i, vertices) = count(parse_vec3, num_vert.into())(i)?;
                let (i, indices) = count(le_u16, usize::try_from(num_idx).unwrap())(i)?;
                header.level_geometry.push(Geometry {
                    dimension,
                    vertices,
                    indices,
                });
                i
            }
            DataType {
                typename: "Vanilla.Map.Geometry.EOH",
                ..
            } => i,
            DataType {
                typename: "Vanilla.Map.Doors", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, u16, bool, u8, u8)
                ..
            } => {
                let (i, n) = le_u16(i)?;
                let (i, items) = count(
                    pair(
                        parse_commons,
                        tuple((le_u16, parse_replay_bool, le_u8, le_u8)),
                    ),
                    n.into(),
                )(i)?;
                for (common, (serial, checkpoint, variant, size)) in items {
                    header.doors.push(Door {
                        idx: commons.len(),
                        serial,
                        checkpoint,
                        variant: DoorVariant::from_repr(variant).unwrap_or_default(),
                        size: DoorSize::from_repr(size).unwrap_or_default(),
                    });
                    commons.push(common);
                }
                i
            }
            DataType {
                typename: "Vanilla.Map.Ladders", // u16, (u8, f32 * 3, f16 * 3 + u8, f16)
                ..
            } => {
                let (i, n) = le_u16(i)?;
                let (i, items) = count(pair(parse_commons_no_id, le_f16), n.into())(i)?;
                for (common, height) in items {
                    header.ladders.push(Ladder {
                        idx: commons.len(),
                        height,
                    });
                    commons.push(common);
                }
                i
            }
            DataType {
                typename: "Vanilla.Map.Terminals", // u16, (i32, u8, f32 * 3, f16 * 3 + u8)
                ..
            } => {
                let (i, n) = le_u16(i)?;
                let (i, items) = count(parse_commons, n.into())(i)?;
                for common in items {
                    header.terminals.push(Terminal { idx: commons.len() });
                    commons.push(common);
                }
                i
            }
            DataType {
                typename: "Vanilla.Map.Generators", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, u16)
                ..
            } => {
                let (i, n) = le_u16(i)?;
                let (i, items) = count(pair(parse_commons, le_u16), n.into())(i)?;
                for (common, serial) in items {
                    header.generators.push(Generator {
                        idx: commons.len(),
                        serial,
                    });
                    commons.push(common);
                }
                i
            }
            DataType {
                typename: "Vanilla.Map.DisinfectStations", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, u16)
                ..
            } => {
                let (i, n) = le_u16(i)?;
                let (i, items) = count(pair(parse_commons, le_u16), n.into())(i)?;
                for (common, serial) in items {
                    header.disinfect_stations.push(DisinfectStation {
                        idx: commons.len(),
                        serial,
                    });
                    commons.push(common);
                }
                i
            }
            DataType {
                typename: "Vanilla.Map.BulkheadControllers", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, u16)
                ..
            } => {
                let (i, n) = le_u16(i)?;
                let (i, items) = count(pair(parse_commons, le_u16), n.into())(i)?;
                for (common, serial) in items {
                    header.bulkhead_controllers.push(BulkheadController {
                        idx: commons.len(),
                        serial,
                    });
                    commons.push(common);
                }
                i
            }
            DataType {
                typename: "Vanilla.Map.ResourceContainers", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, u16, bool)
                ..
            } => {
                let (i, n) = le_u16(i)?;
                let (i, items) = count(
                    pair(parse_commons, tuple((le_u16, parse_replay_bool))),
                    n.into(),
                )(i)?;
                for (common, (serial, locker)) in items {
                    header.resource_containers.push(ResourceContainer {
                        idx: commons.len(),
                        serial,
                        locker,
                        registered: None,
                        lock_type: None,
                    });
                    commons.push(common);
                }
                i
            }
            DataType {
                typename: "Vanilla.Enemy.Spitters", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, f16)
                ..
            } => {
                let (i, n) = le_u16(i)?;
                let (i, items) = count(pair(parse_commons, le_f16), n.into())(i)?;
                for (common, scale) in items {
                    header.spitters.push(Spitter {
                        idx: commons.len(),
                        scale,
                    });
                    commons.push(common);
                }
                i
            }
            _ => i,
        };
        if i.is_empty() {
            break;
        }
    }

    Ok((i, (typemap, header)))
}
