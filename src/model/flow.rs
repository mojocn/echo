

use serde::{Deserialize,Serialize};

#[derive(Debug, Serialize,Deserialize)]
pub struct Flow {
    pub id:i64,
    pub url:String,
    pub tiny:String,
}
