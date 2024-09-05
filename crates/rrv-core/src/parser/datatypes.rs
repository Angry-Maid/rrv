use glam::{DQuat, Vec3};
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
    pub id: i32,
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
    pub id: i32,
    pub idx: usize,
}

#[derive(Debug)]
pub struct Generator {
    pub id: i32,
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
    pub id: i32,
    pub idx: usize,
    pub serial: u16,
    pub locker: bool,
    pub registered: Option<bool>,
    pub lock_type: Option<LockType>,
}

#[derive(Debug)]
pub struct DisinfectStation {
    pub id: i32,
    pub idx: usize,
    pub serial: u16,
}

#[derive(Debug)]
pub struct BulkheadController {
    pub id: i32,
    pub idx: usize,
    pub serial: u16,
    pub main: Option<i32>,
    pub secondary: Option<i32>,
    pub ovl: Option<i32>,
}

#[derive(Debug)]
pub struct Spitter {
    pub id: i32,
    pub idx: usize,
    pub scale: f16,
}
