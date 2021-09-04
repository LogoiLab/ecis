pub struct Area {
    id: u32,
    name: String,
    location: Location
}

pub struct Location {
    id: u32,
    name: String,
    description: Option<String>,
}
