use mongodb::{options::FindOptions, bson::{Document, to_bson}};
use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryFilter {
    pub limit: Option<u16>,
    pub sort: Option<String>,
    pub page: Option<usize>
}
impl From<QueryFilter> for QueryOptions {
    fn from(val: QueryFilter) -> Self {
        if let (Some(limit),Some(sort), Some(page)) = (val.limit.or(Some(10)),val.sort.or(Some("-created".to_string())),val.page.or(Some(1))){
            QueryOptions{
                limit,
                sort,
                page,
            }
        } else {
            QueryOptions{
                limit: 10,
                sort: "created_at".to_string(),
                page: 1,
            }
        }
    }
}
pub struct QueryOptions {
    pub limit: u16,
    pub sort: String,
    pub page: usize,
}

impl From<QueryOptions> for FindOptions {
    fn from(val: QueryOptions) -> Self {
        if val.sort.len() > 1 {
            let direction: i8 = if val.sort.get(0..1) == Some("-") {-1} else {1};
            if let Some(field) = val.sort.get(1..){
                let mut dc = Document::new();
                if let Ok(value) = to_bson(&direction){
                    dc.insert(field, value);
                } 
                return FindOptions::builder().limit(i64::from(val.limit)).sort(dc).build();
            }
        }
        FindOptions::builder().limit(i64::from(val.limit)).build()
    }
}
