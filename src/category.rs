pub struct Category {
    id: u32,
    name: String
}

pub struct SubCategory {
    id: u32,
    name: String,
    sub_of: Category
}
