#[derive(Debug)]
pub struct Typemap<'a> {
    pub version: &'a str,
    pub entries: u16,
    pub types: Vec<DataType<'a>>,
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
