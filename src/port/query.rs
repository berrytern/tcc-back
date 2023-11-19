use serde_json::Map;
pub struct Sort {
    direction: i8,
    field: String,
}

enum JsonValue<'a> {
    String(&'a str),  
    Int(&'a u32),
    Double(&'a f32),
}

pub struct Query<'a> {
    limit: u16,
    sort: Sort,
    filter: Map<String,JsonValue<'a>>
}