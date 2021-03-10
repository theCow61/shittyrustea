use std::collections::HashMap;
use super::Commandment;
use serde_json::{Value as JsonValue, json, /*to_string_pretty*/};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Bank {
    // username: String,
    // balance: f64,
    // tele_user_executer: String,
    pub json_object: JsonValue,
    db_path: String,
    pub msg_info: HashMap<String, String>,
}

impl Bank {
    pub fn new(/*tele_user_executer: String*/) -> Result<Bank, Box<dyn std::error::Error>> {
        // let bruha = tele_user_executer.to_owned();
        dotenv::dotenv().unwrap();
        let contents = std::fs::read_to_string(std::env::var("STAT_PATH").expect("Env not set..."))?;
        let res = serde_json::from_str(&contents)?;
        let msg_info = HashMap::new();
        Ok(
            Bank {
                // tele_user_executer: bruha,
                json_object: res,
                db_path: std::env::var("STAT_PATH").unwrap(),
                msg_info,
            }
        )
        // match res {
        //     Ok(resa) => {
        //         let p: Bank = resa;
        //         Ok(p)
        //     }
        //     Err(e) => {
        //         println!("Could not parse json...");
        //         Err(Box::new(e))
        //     }
        // }
        // Ok(
        //     Bank {
        //         username: "lul".to_owned(),
        //         balance: 3f64,

    }
    pub async fn get_balance(&mut self, commandent: &Commandment) -> Result<u64, std::io::Error> {
        // if self.json_object["CowSheckles"][commandent.cap_map["Executer"]] == JsonValue::Null {
        //     self.json_object["CowSheckles"][commandent.cap_map["Executer"]] = json!(0);
        //     let new_contents = serde_json::to_string_pretty(&self.json_object)?;
        //     tokio::fs::write(&self.db_path, new_contents).await?;
        //     return Ok(0)
        // }
	    self.null_nullifier(&commandent.cap_map["Executer"]).await?;
        let balance: u64 = self.json_object["CowSheckles"][&commandent.cap_map["Executer"]].as_u64().unwrap().to_owned();
        Ok(balance)
    }
    pub async fn transfer(&mut self, commandent: &Commandment, amount: u64) -> Result<Option<()>, Box<dyn std::error::Error>> {
        // if self.json_object["CowSheckles"][&commandent.cap_map["Executer"]] == JsonValue::Null {
        //     self.json_object["CowSheckles"][&commandent.cap_map["Executer"]] = json!(0);
        //     let new_contents = serde_json::to_string_pretty(&self.json_object)?;
        //     tokio::fs::write(&self.db_path, new_contents).await?;
        //     return Ok(None)
        // }
        // if self.json_object["CowSheckles"][&commandent.cap_map[""]]
        self.null_nullifier(&commandent.cap_map["Executer"]).await?;
        self.null_nullifier(&commandent.cap_map["Recep"]).await?;
        if self.json_object["CowSheckles"][&commandent.cap_map["Executer"]].as_u64().unwrap() < amount {
            return Ok(None)
        }
        if self.json_object["CowSheckles"][&commandent.cap_map["Executer"]].as_u64().unwrap() >= amount {
            let to_inc = self.json_object["CowSheckles"][&commandent.cap_map["Recep"]].as_u64().unwrap() + amount;
            let to_dec = self.json_object["CowSheckles"][&commandent.cap_map["Executer"]].as_u64().unwrap() - amount;
            self.json_object["CowSheckles"][&commandent.cap_map["Recep"]] = json!(to_inc);
            self.json_object["CowSheckles"][&commandent.cap_map["Executer"]] = json!(to_dec);
            let new_contents = serde_json::to_string_pretty(&self.json_object)?;
            tokio::fs::write(&self.db_path, new_contents).await?;
            return Ok(Some(()))
        }
        Ok(None)
    }
    async fn null_nullifier(&mut self, test_if_null: &str) -> Result<(), tokio::io::Error> {
        if self.json_object["CowSheckles"][test_if_null] == JsonValue::Null {
            self.json_object["CowSheckles"][test_if_null] = json!(0);
            let new_contents = serde_json::to_string_pretty(&self.json_object)?;
            tokio::fs::write(&self.db_path, new_contents).await?;
        }
        Ok(())
    }
}
