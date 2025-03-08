use serde::Serialize;
use serde_json;

// 将数据格式化为 JSON 字符串
pub fn to_json<T: Serialize>(data: &T) -> String {
    serde_json::to_string_pretty(data).unwrap()
}

// 计算数据的 SHA-256 哈希值
pub fn calculate_hash<T: Serialize>(data: &T) -> String {
    let input = serde_json::to_string(data).unwrap();
    let mut hasher = sha2::Sha256::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}
