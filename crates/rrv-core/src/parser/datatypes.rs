use std::default;

use glam::{DQuat, Quat, Vec3};
use strum_macros::FromRepr;

#[derive(Debug, Default)]
pub struct Header<'a> {
    pub commons: Commons,
    pub replay_header: Option<ReplayHeader<'a>>,
    pub metadata: Option<Metadata<'a>>,
    pub level_geometry: Vec<Geometry>,
    pub doors: Vec<Door>,
    pub ladders: Vec<Ladder>,
    pub terminals: Vec<Terminal>,
    pub generators: Vec<Generator>,
    pub resource_containers: Vec<ResourceContainer>,
    pub disinfect_stations: Vec<DisinfectStation>,
    pub bulkhead_controllers: Vec<BulkheadController>,
    pub spitters: Vec<Spitter>,
}

#[derive(Debug)]
pub struct ReplayHeader<'a> {
    pub version: &'a str,
    pub master: bool,
}

#[derive(Debug)]
pub struct Common {
    pub id: Option<i32>,
    pub dimension: u8,
    pub position: Vec3,
    pub rotation: DQuat,
}

pub type Commons = Vec<Common>;

// Covers both 0.0.1 and 0.0.2
#[derive(Debug)]
pub struct Metadata<'a> {
    pub version: &'a str,
    pub compatability_old_dc: Option<bool>,
}

#[derive(Debug)]
pub struct Geometry {
    pub dimension: u8,
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u16>,
}

#[derive(FromRepr, Debug, PartialEq, Default)]
#[repr(u8)]
pub enum DoorVariant {
    #[default]
    WeakDoor = 0,
    SecurityDoor,
    BulkheadDoor,
    BulkheadDoorMain,
    ApexDoor,
}

#[derive(FromRepr, Debug, PartialEq, Default)]
#[repr(u8)]
pub enum DoorSize {
    #[default]
    Small = 0,
    Medium,
    Large,
}

#[derive(Debug)]
pub struct Door {
    pub idx: usize,
    pub serial: u16,
    pub checkpoint: bool,
    pub variant: DoorVariant,
    pub size: DoorSize,
}

#[derive(Debug)]
pub struct Ladder {
    pub idx: usize,
    pub height: f16,
}

#[derive(Debug)]
pub struct Terminal {
    pub idx: usize,
}

#[derive(Debug)]
pub struct Generator {
    pub idx: usize,
    pub serial: u16,
}

#[derive(FromRepr, Debug, PartialEq, Default)]
#[repr(u8)]
pub enum LockType {
    #[default]
    None = 0,
    Melee,
    Hack,
}

#[derive(Debug)]
pub struct ResourceContainer {
    pub idx: usize,
    pub serial: u16,
    pub locker: bool,
    pub registered: Option<bool>,
    pub lock_type: Option<LockType>,
}

#[derive(Debug)]
pub struct DisinfectStation {
    pub idx: usize,
    pub serial: u16,
}

#[derive(Debug)]
pub struct BulkheadController {
    pub idx: usize,
    pub serial: u16,
}

#[derive(Debug)]
pub struct Spitter {
    pub idx: usize,
    pub scale: f16,
}
