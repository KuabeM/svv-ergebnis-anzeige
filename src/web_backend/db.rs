use mysql::Pool;
use std::env::var;
use std::fmt::Display;

use super::super::errors::*;

pub struct MysqlDB {
    pool: Pool,
    name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DbRow {
    no: u8,
    pub team: String,
    pub score: u8,
}

impl MysqlDB {
    pub fn create_db(name: String) -> Result<Self> {
        let cred = var("DB_CREDS").unwrap_or("Set_env_var_DB_CREDS:a".to_string());
        let pool = Pool::new(format!("mysql://{}@localhost/svv_backend", cred)).map_err(|e| {
            BackendError::DatabaseError.context(format!("Failed to get db instance: {:?}", e))
        })?;
        let stat = format!(r"DROP TABLE IF EXISTS {}", name);
        pool.prep_exec(stat, ()).map_err(|e| {
            BackendError::DatabaseError.context(format!("Failed to create db: {:?}", e))
        })?;
        let stat = format!(r"CREATE TABLE {} (no int, team text, score int)", name);
        pool.prep_exec(stat, ()).map_err(|e| {
            BackendError::DatabaseError.context(format!("Failed to init db: {:?}", e))
        })?;

        Ok(MysqlDB { pool, name })
    }

    pub fn db_instance(name: String) -> Result<Self> {
        let cred = var("DB_CREDS").unwrap_or("Set_env_var_DB_CREDS:a".to_string());
        let pool = Pool::new(format!("mysql://{}@localhost/svv_backend", cred)).map_err(|e| {
            BackendError::DatabaseError.context(format!("Failed to get db instance: {:?}", e))
        })?;
        Ok(MysqlDB { pool, name })
    }

    pub fn insert(&self, key: u8, name: String, score: u8) -> Result<()> {
        let stat = format!(
            r"INSERT INTO {} (no, team, score) VALUES ('{}', '{}', {})",
            self.name, key, name, score
        );
        self.pool.prep_exec(stat, ()).map_err(|e| {
            BackendError::DatabaseError.context(format!("Failed to insert into db: {:?}", e))
        })?;

        Ok(())
    }

    pub fn update<T>(&self, key: u8, col: String, val: T) -> Result<()>
    where
        T: Display,
    {
        let stat = format!(
            r"UPDATE {} SET {} = '{}' WHERE no = {}",
            self.name, col, val, key
        );
        self.pool.prep_exec(stat, ()).map_err(|e| {
            BackendError::DatabaseError.context(format!("Failed to update db: {:?}", e))
        })?;

        Ok(())
    }

    pub fn select(&self, key: u8) -> Result<Vec<DbRow>> {
        let stat = format!(
            r"SELECT no, team, score FROM {} WHERE no = '{}'",
            self.name, key
        );

        let cont: Vec<DbRow> = self.pool.prep_exec(stat, ()).map(|answer| {
            answer
                .map(|x| x.unwrap())
                .map(|row| {
                    let (no, team, score) = mysql::from_row(row);
                    DbRow { no, team, score }
                })
                .collect()
        })?;

        Ok(cont)
    }
}
