pub struct Replay<'a> {
    typemap: Vec<Typemap<'a>>,
}

#[derive(Debug)]
pub struct Typemap<'a> {
    pub version: &'a str,
    pub entries: u16,
    pub types: Vec<DataType<'a>>,
}

pub struct HeaderEntry<'a>(u16, &'a [u8]);

pub struct Header<'a> {
    entries: Vec<HeaderEntry<'a>>,
}

#[derive(Debug)]
pub struct DataType<'a> {
    pub id: u16,
    pub typename: &'a str,
    pub version: &'a str,
}

pub struct Snapshot {
    timestamp: u32,
}

pub struct Event {
    id: u16,
    offset: u16,
    // ...
}

pub struct Dynamic {}
