use anyhow::anyhow;

pub enum DatabaseType {
    Postgresql,
}

pub struct Connection {}

impl Connection {
    pub fn connect(dbtype: DatabaseType, conn_str: impl Into<String>) -> anyhow::Result<Self> {
        Err(anyhow!("not implement"))
    }
}
