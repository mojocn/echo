


use serde::{Deserialize,Serialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct CreateUser {
    pub username:String
}
#[derive(Debug, Serialize,Deserialize)]
pub struct Link {
    pub id:i64,
    pub url:String,
    pub tiny:String,
}



#[derive(Debug,Serialize,Deserialize)]
pub struct BaseResult<'a,T> {
    pub code:i32,
    pub msg: &'a str,
    pub data:T,
}

impl <T>BaseResult<'_, T> {
    pub fn ok(e:T) ->Self {
        BaseResult{
            code:200,
            msg:"ok",
            data:e,
        }
    }
}




#[cfg(test)]
mod tests {
    use sqlx;
    use sqlx::mysql::MySqlPool;
    use std::env;
    use tokio;
    use uuid::Uuid;

    #[tokio::test]
    async fn db() {
        //env::set_var("DATABASE_URL", "mysql://root:123456@mydevdb/klinkdb");
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = MySqlPool::connect(&url).await.expect("数据库连接失败");

        let recs = sqlx::query!("SELECT id, url FROM kae_links ORDER BY id")
            .fetch_all(&pool)
            .await
            .expect("msg");
        // NOTE: Booleans in MySQL are stored as `TINYINT(1)` / `i8`
        //       0 = false, non-0 = true
        for rec in recs {
            println!("- {}: {}", rec.id, &rec.url.expect("xxx"),);
        }
    }

    #[tokio::test]
    async fn uuid() {
        let my_uuid = Uuid::new_v4();
        println!("{}", my_uuid.to_string());


    }
}
