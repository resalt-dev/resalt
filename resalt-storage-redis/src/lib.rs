use r2d2::{Pool, PooledConnection};
use redis::{Client, Commands};
use resalt_models::*;

#[derive(Clone)]
pub struct StorageRedis {
    pool: Pool<Client>,
}

impl StorageRedis {
    pub async fn connect(database_url: &str) -> Result<Self, String> {
        let client = Client::open(database_url).unwrap();
        let pool = Pool::builder().build(client);

        match pool {
            Ok(pool) => Ok(Self { pool }),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    fn create_connection(&self) -> Result<PooledConnection<Client>, String> {
        match self.pool.get() {
            Ok(conn) => Ok(conn),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}

impl StorageImpl for StorageRedis {
    fn clone(&self) -> Box<dyn StorageImpl> {
        Box::new(Clone::clone(self))
    }

    fn get(&self, key: &str) -> Result<Option<String>, String> {
        let mut connection = self.create_connection()?;
        let value: Option<String> = connection.get(key).map_err(|e| format!("{:?}", e))?;
        Ok(value)
    }

    fn set(&self, key: &str, value: &str) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        connection.set(key, value).map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn del(&self, key: &str) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        connection.del(key).map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn keys(&self, pattern: &str) -> Result<Vec<String>, String> {
        let mut connection = self.create_connection()?;
        let keys: Vec<String> = connection.keys(pattern).map_err(|e| format!("{:?}", e))?;
        Ok(keys)
    }
}
