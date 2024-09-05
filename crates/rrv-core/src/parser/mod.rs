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

#[derive(Debug)]
pub struct Replay<'a> {
    pub typemap: Typemap<'a>,
    pub header: Header<'a>,
}

pub fn parse_replay(i: &[u8]) -> IResult<&[u8], Replay> {
    let (i, (typemap, header)) = parse_replay_file_commons(i)?;

    Ok((i, Replay { typemap, header }))
}

pub fn parse_replay_file_commons(i: &[u8]) -> IResult<&[u8], (Typemap, Header)> {
    let (i, metadata_size) = le_u32(i)?;
    let (i, typemap_and_header_bytes) = take(metadata_size)(i)?;
    let (leftover, (typemap, header)) = parse_typemap_and_header(typemap_and_header_bytes)?;

    info!(
        "{:#?}\n{:#?}",
        header.bulkhead_controllers.first().unwrap(),
        header.commons[header.bulkhead_controllers.first().unwrap().idx]
    );
    let bulkhead_door = header
        .doors
        .iter()
        .find(|&v| {
            v.id == header
                .bulkhead_controllers
                .first()
                .unwrap()
                .secondary
                .unwrap()
        })
        .unwrap();
    info!(
        "{:#?}\n{:#?}",
        bulkhead_door, header.commons[bulkhead_door.idx]
    );
    info!("{:?}", leftover);

    Ok((i, (typemap, header)))
}

pub fn parse_replay_string(i: &[u8]) -> IResult<&[u8], &str> {
    let (i, length) = le_u16(i)?;
    map_res(take(length), str::from_utf8)(i)
}

pub fn parse_replay_bool(i: &[u8]) -> IResult<&[u8], bool> {
    let (i, val) = le_u8(i)?;

    Ok((i, val > 0))
}

pub fn parse_replay_identifier_type(i: &[u8]) -> IResult<&[u8], IdentifierType> {
    let (mut i, val) = le_u8(i)?;
    let idt = match Identifier::from_repr(val).unwrap_or_default() {
        Identifier::Unknown => IdentifierType::Unknown,
        Identifier::Gear => {
            let gear;
            let alias;
            (i, (gear, alias)) = pair(parse_replay_string, le_u16)(i)?;
            IdentifierType::Gear(gear, alias)
        }
        Identifier::AliasGear => {
            let alias;
            (i, alias) = le_u16(i)?;
            IdentifierType::AliasGear(alias)
        }
        Identifier::Item => {
            let id;
            (i, id) = le_u16(i)?;
            IdentifierType::Item(id)
        }
        Identifier::Enemy => {
            let id;
            (i, id) = le_u16(i)?;
            IdentifierType::Enemy(id)
        }
        Identifier::Vanity => {
            let id;
            (i, id) = le_u16(i)?;
            IdentifierType::Vanity(id)
        }
    };

    Ok((i, idt))
}

pub fn parse_vec3(i: &[u8]) -> IResult<&[u8], Vec3> {
    let (i, (x, y, z)) = tuple((le_f32, le_f32, le_f32))(i)?;
    Ok((i, Vec3::new(x, y, z)))
}

pub type BulkheadLayers = (Option<i32>, Option<i32>, Option<i32>);

pub fn parse_bulkhead_dc(i: &[u8]) -> IResult<&[u8], BulkheadLayers> {
    let (mut main_door_id, mut secondary_door_id, mut overload_door_id) = (None, None, None);
    let (mut i, main) = parse_replay_bool(i)?;
    if main {
        let door_id;
        (i, door_id) = le_i32(i)?;
        main_door_id = Some(door_id);
    }
    let secondary;
    (i, secondary) = parse_replay_bool(i)?;
    if secondary {
        let door_id;
        (i, door_id) = le_i32(i)?;
        secondary_door_id = Some(door_id);
    }
    let overload;
    (i, overload) = parse_replay_bool(i)?;
    if overload {
        let door_id;
        (i, door_id) = le_i32(i)?;
        overload_door_id = Some(door_id);
    }

    Ok((i, (main_door_id, secondary_door_id, overload_door_id)))
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
    let (i, (dimension, position, rotation)) = tuple((le_u8, parse_vec3, parse_half_quat))(i)?;

    Ok((
        i,
        Common {
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
    let (mut i, types) = count(parse_datatype, typemap_entries.into())(i)?;

    let typemap = Typemap {
        version: typemap_ver,
        entries: typemap_entries,
        types,
    };

    let mut header = Header::default();

    loop {
        let id;
        (i, id) = le_u16(i)?;
        let t = typemap.types.iter().find(|&v| v.id == id).unwrap();
        info!("{:#?}", t);
        match t {
            DataType {
                typename: "ReplayRecorder.Header", // string, bool
                ..
            } => {
                let version;
                let master;
                (i, (version, master)) = pair(parse_replay_string, parse_replay_bool)(i)?;
                info!("{:?} {:?}", version, master);
                header.replay_header = Some(ReplayHeader { version, master });
            }
            DataType {
                typename: "ReplayRecorder.EndOfHeader",
                ..
            } => break,
            DataType {
                typename: "Vanilla.Metadata", // string
                version: "0.0.1",
                ..
            } => {
                let version;
                (i, version) = parse_replay_string(i)?;
                info!("{:?}", version);
                header.metadata = Some(Metadata {
                    version,
                    compatability_old_dc: None,
                });
            }
            DataType {
                typename: "Vanilla.Metadata", // string, bool
                version: "0.0.2",
                ..
            } => {
                let version;
                let compat;
                (i, (version, compat)) = pair(parse_replay_string, parse_replay_bool)(i)?;
                info!("{:?} {:?}", version, compat);
                header.metadata = Some(Metadata {
                    version,
                    compatability_old_dc: Some(compat),
                });
            }
            DataType {
                typename: "Vanilla.Map.Geometry", // u8, u16, u32, f32 * 3 list, u16 list
                ..
            } => {
                let dimension;
                let num_vert;
                let num_idx;
                let vertices;
                let indices;
                (i, (dimension, num_vert, num_idx)) = tuple((le_u8, le_u16, le_u32))(i)?;
                info!("{:?} {:?} {:?}", dimension, num_vert, num_idx);
                (i, vertices) = count(parse_vec3, num_vert.into())(i)?;
                (i, indices) = count(le_u16, usize::try_from(num_idx).unwrap())(i)?;
                header.level_geometry.push(Geometry {
                    dimension,
                    vertices,
                    indices,
                });
            }
            DataType {
                typename: "Vanilla.Map.Geometry.EOH",
                ..
            } => {}
            DataType {
                typename: "Vanilla.Map.Doors", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, u16, bool, u8, u8)
                ..
            } => {
                let n;
                let items;
                (i, n) = le_u16(i)?;
                info!("{:?}", n);
                (i, items) = count(
                    pair(
                        tuple((le_i32, parse_commons)),
                        tuple((le_u16, parse_replay_bool, le_u8, le_u8)),
                    ),
                    n.into(),
                )(i)?;
                for ((id, common), (serial, checkpoint, variant, size)) in items {
                    header.doors.push(Door {
                        id,
                        idx: header.commons.len(),
                        serial,
                        checkpoint,
                        variant: DoorVariant::from_repr(variant).unwrap_or_default(),
                        size: DoorSize::from_repr(size).unwrap_or_default(),
                    });
                    header.commons.push(common);
                }
            }
            DataType {
                typename: "Vanilla.Map.Ladders", // u16, (u8, f32 * 3, f16 * 3 + u8, f16)
                ..
            } => {
                let n;
                let items;
                (i, n) = le_u16(i)?;
                info!("{:?}", n);
                (i, items) = count(pair(parse_commons, le_f16), n.into())(i)?;
                for (common, height) in items {
                    header.ladders.push(Ladder {
                        idx: header.commons.len(),
                        height,
                    });
                    header.commons.push(common);
                }
            }
            DataType {
                typename: "Vanilla.Map.Terminals", // u16, (i32, u8, f32 * 3, f16 * 3 + u8)
                ..
            } => {
                let n;
                let items;
                (i, n) = le_u16(i)?;
                info!("{:?}", n);
                (i, items) = count(tuple((le_i32, parse_commons)), n.into())(i)?;
                for (id, common) in items {
                    header.terminals.push(Terminal {
                        id,
                        idx: header.commons.len(),
                    });
                    header.commons.push(common);
                }
            }
            DataType {
                typename: "Vanilla.Map.Generators", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, u16)
                ..
            } => {
                let n;
                let items;
                (i, n) = le_u16(i)?;
                info!("{:?}", n);
                (i, items) = count(pair(tuple((le_i32, parse_commons)), le_u16), n.into())(i)?;
                for ((id, common), serial) in items {
                    header.generators.push(Generator {
                        id,
                        idx: header.commons.len(),
                        serial,
                    });
                    header.commons.push(common);
                }
            }
            DataType {
                typename: "Vanilla.Map.DisinfectStations", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, u16)
                ..
            } => {
                let n;
                let items;
                (i, n) = le_u16(i)?;
                info!("{:?}", n);
                (i, items) = count(pair(tuple((le_i32, parse_commons)), le_u16), n.into())(i)?;
                for ((id, common), serial) in items {
                    header.disinfect_stations.push(DisinfectStation {
                        id,
                        idx: header.commons.len(),
                        serial,
                    });
                    header.commons.push(common);
                }
            }
            DataType {
                typename: "Vanilla.Map.BulkheadControllers", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, u16, bool, i32?, bool, i32?, bool, i32?)
                ..
            } => {
                let n;
                let items;
                (i, n) = le_u16(i)?;
                info!("{:?}", n);
                (i, items) = count(
                    pair(
                        tuple((le_i32, parse_commons)),
                        tuple((le_u16, parse_bulkhead_dc)),
                    ),
                    n.into(),
                )(i)?;
                for ((id, common), (serial, (main, secondary, ovl))) in items {
                    header.bulkhead_controllers.push(BulkheadController {
                        id,
                        idx: header.commons.len(),
                        serial,
                        main,
                        secondary,
                        ovl,
                    });
                    header.commons.push(common);
                }
            }
            DataType {
                typename: "Vanilla.Map.ResourceContainers", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, u16, bool)
                version: "0.0.1",
                ..
            } => {
                let n;
                let items;
                (i, n) = le_u16(i)?;
                info!("{:?}", n);
                (i, items) = count(
                    pair(
                        tuple((le_i32, parse_commons)),
                        tuple((le_u16, parse_replay_bool)),
                    ),
                    n.into(),
                )(i)?;
                for ((id, common), (serial, locker)) in items {
                    header.resource_containers.push(ResourceContainer {
                        id,
                        idx: header.commons.len(),
                        serial,
                        locker,
                        consumable_type: None,
                        registered: None,
                        lock_type: None,
                    });
                    header.commons.push(common);
                }
            }
            DataType {
                typename: "Vanilla.Map.ResourceContainers", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, u16, bool)
                version: "0.0.2",
                ..
            } => {
                let n;
                let items;
                (i, n) = le_u16(i)?;
                info!("{:?}", n);
                (i, items) = count(
                    pair(
                        tuple((le_i32, parse_commons)),
                        tuple((
                            le_u16,
                            parse_replay_bool,
                            parse_replay_identifier_type,
                            parse_replay_bool,
                        )),
                    ),
                    n.into(),
                )(i)?;
                for ((id, common), (serial, locker, consumable_type, registered)) in items {
                    header.resource_containers.push(ResourceContainer {
                        id,
                        idx: header.commons.len(),
                        serial,
                        locker,
                        consumable_type: Some(consumable_type),
                        registered: Some(registered),
                        lock_type: None,
                    });
                    header.commons.push(common);
                }
            }
            DataType {
                typename: "Vanilla.Map.ResourceContainers", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, u16, bool)
                version: "0.0.3",
                ..
            } => {
                let n;
                let items;
                (i, n) = le_u16(i)?;
                info!("{:?}", n);
                (i, items) = count(
                    pair(
                        tuple((le_i32, parse_commons)),
                        tuple((
                            le_u16,
                            parse_replay_bool,
                            parse_replay_identifier_type,
                            parse_replay_bool,
                            le_u8,
                        )),
                    ),
                    n.into(),
                )(i)?;
                for ((id, common), (serial, locker, consumable_type, registered, lock_type)) in
                    items
                {
                    header.resource_containers.push(ResourceContainer {
                        id,
                        idx: header.commons.len(),
                        serial,
                        locker,
                        consumable_type: Some(consumable_type),
                        registered: Some(registered),
                        lock_type: LockType::from_repr(lock_type),
                    });
                    header.commons.push(common);
                }
            }
            DataType {
                typename: "Vanilla.Enemy.Spitters", // u16, (i32, u8, f32 * 3, f16 * 3 + u8, f16)
                ..
            } => {
                let n;
                let items;
                (i, n) = le_u16(i)?;
                info!("{:?}", n);
                (i, items) = count(pair(tuple((le_i32, parse_commons)), le_f16), n.into())(i)?;
                for ((id, common), scale) in items {
                    header.spitters.push(Spitter {
                        id,
                        idx: header.commons.len(),
                        scale,
                    });
                    header.commons.push(common);
                }
            }
            _ => {}
        };
    }

    Ok((i, (typemap, header)))
}
