use confy;
// extern crate serde_derive;
use serde_derive::{Deserialize, Serialize};

use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
    user: String,
    pass: String,
    dbname: String,
    protocol: String,
    dbms: String,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            dbms: "mysql".to_string(),
            protocol: "10.2022.11.1:3306".to_string(),
            user: "hyperx".to_string(),
            pass: "fafle0e90eofjda".to_string(),
            dbname: "hyperx_schema".to_string(),
        }
    }
}

pub fn load_source() -> Result<String, confy::ConfyError> {
    let conf_name = "c01_mysql_client";
    let  conf: Conf = confy::load(conf_name, None)?;
    Ok(format!(
            "{}://{}:{}@{}/{}",
            conf.dbms, conf.user, conf.pass, conf.protocol, conf.dbname
    ))
}
