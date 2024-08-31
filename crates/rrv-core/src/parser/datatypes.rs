use glam::{Quat, Vec3};

pub struct ReplayHeader<'a> {
    version: &'a str,
    master: bool,
}

pub struct Common {
    id: Option<i32>,
    dimension: u8,
    position: Vec3,
    rotation: Quat,
}

pub type Objects = Vec<Common>;

// Covers both 0.0.1 and 0.0.2
pub struct Metadata<'a> {
    version: &'a str,
    compatability_old_dc: Option<bool>,
}

pub struct Geometry {
    dimension: u8,
    vertices: Vec<Vec3>,
    indices: Vec<u16>,
}

pub enum DoorVariant {
    WeakDoor = 0,
    SecurityDoor,
    BulkheadDoor,
    BulkheadDoorMain,
    ApexDoor,
}

pub enum DoorSize {
    Small = 0,
    Medium,
    Large,
}

pub struct Door {
    idx: u32,
    serial: u16,
    checkpoint: bool,
    variant: DoorVariant,
    size: DoorSize,
}

pub struct Ladder {
    idx: u32,
    height: f16,
}

pub struct Terminal {
    idx: u32,
}

pub struct Generator {
    idx: u32,
    serial: u16,
}

pub struct ResourceContainer {
    idx: u32,
    serial: u16,
    locker: bool,
}

pub struct DisinfectStation {
    idx: u32,
    serial: u16,
}

pub struct BulkheadControllers {
    idx: u32,
    serial: u16,
}
pub struct Spitter {
    idx: u32,
    scale: f16,
}
