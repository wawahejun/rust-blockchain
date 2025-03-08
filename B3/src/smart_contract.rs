use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmartContract {
    pub code: String, // 合约代码
    pub state: HashMap<String, String>, // 合约状态
}

impl SmartContract {
    // 创建一个新的智能合约
    pub fn new(code: String) -> Self {
        SmartContract {
            code,
            state: HashMap::new(),
        }
    }

    // 执行合约方法
    pub fn execute(&mut self, method: &str, args: Vec<String>) -> Result<String, String> {
        match method {
            "set" => {
                if args.len() != 2 {
                    return Err("Invalid arguments for 'set'".to_string());
                }
                self.state.insert(args[0].clone(), args[1].clone());
                Ok("Value set successfully".to_string())
            }
            "get" => {
                if args.len() != 1 {
                    return Err("Invalid arguments for 'get'".to_string());
                }
                match self.state.get(&args[0]) {
                    Some(value) => Ok(value.clone()),
                    None => Err("Key not found".to_string()),
                }
            }
            _ => Err("Unknown method".to_string()),
        }
    }
}
