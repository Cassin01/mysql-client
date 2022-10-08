pub mod utils {
    use std::any::type_name;
    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }
    // #[async_std::main]
    use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
    use sqlx::ConnectOptions;
    use sqlx::mysql::MySqlPool;
    pub async fn connect(url: &str) -> Result<MySqlPool, sqlx::Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;
        // println!("{}", type_of(&pool));

        // let row: (i64, ) = sqlx::query_as("SELECT id=1 from password")
        //     .fetch_one(&pool).await?;
        // println!("{:?}",row.0);

        Ok(pool)
    }

    #[derive(PartialEq, Clone)]
    pub struct Table {
        pub Tables_in_your_schema: String,
    }

    use sqlx::mysql::MySqlRow;
    use sqlx::{FromRow, Row};
    impl<'c> FromRow<'c, MySqlRow> for Table {
        fn from_row(row: &'c MySqlRow) -> Result<Self, sqlx::Error> {
            Ok(Table {
                Tables_in_your_schema: row.get(0),
            })
        }
    }

    pub async fn show_tables(pool: &sqlx::MySqlPool) -> Result<Vec<String>, sqlx::Error> {
        let tables = sqlx::query("show tables").fetch_all(pool).await?;

        Ok(tables.iter().map(|row| row.try_get(0).unwrap()).collect())
    }

    use tokio::sync::Mutex;
    pub struct Pool(pub Mutex<Option<MySqlPool>>);
}
// use std::sync::RwLock;
//     pub struct Pool(pub RwLock<Option::<sqlx::MySqlPool>>);
// }
