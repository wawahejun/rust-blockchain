# Rust 区块链框架

## 项目概述

本项目是一个模块化的 Rust 区块链框架demo，包含三个版本：B1（教学版）、B2（核心版）和 B3（扩展版）。每个版本针对不同的使用场景进行了优化和扩展，适合从学习区块链基础到开发区块链应用的需求。

---

## 版本对比

| 特性                | B1（教学版）                     | B2（核心版）                     | B3（扩展版）                     |
|---------------------|----------------------------------|----------------------------------|----------------------------------|
| **目标用户**         | 初学者、教学场景                 | 开发者、快速原型                 | 高级开发者、生产环境             |
| **核心功能**         | 区块、交易、挖矿、钱包           | 区块、交易、挖矿                 | 区块、交易、挖矿、智能合约、隐私 |
| **扩展功能**         | 文件存储、CLI 交互               | 无                               | 智能合约、隐私交易、P2P 网络     |
| **性能优化**         | 适中                            | 高                               | 高                               |
| **代码复杂度**       | 中（含详细注释）                 | 低                               | 高                               |

---

## 项目结构

### B1（教学版）
```
src/
├── block.rs          # 区块核心结构（含完整注释）
├── blockchain.rs     # 区块链管理（含文件存储）
├── cli.rs            # 命令行交互界面
├── merkle_tree.rs    # 默克尔树完整实现
├── node.rs           # 简单节点网络功能
├── pow.rs            # 工作量证明模块
├── transaction.rs    # 交易系统（含钱包集成）
├── utils.rs          # 辅助工具集
├── wallet.rs         # 独立钱包模块
└── main.rs           # 命令行主程序入口
```

### B2（核心版）
```
src/
├── block.rs          # 精简区块结构（移除冗余字段）
├── blockchain.rs     # 区块链核心逻辑（移除文件存储）
├── merkle_tree.rs    # 优化默克尔树实现
├── transaction.rs    # 基础交易验证
├── node.rs           # 简单节点网络功能
└── main.rs           # 直接运行示例

```

### B3（扩展版）
```
src/
├── block.rs          # 区块核心结构
├── blockchain.rs     # 区块链管理
├── transaction.rs    # 交易系统
├── merkle_tree.rs    # 默克尔树实现
├── node.rs           # P2P 网络节点
├── cli.rs            # 命令行交互界面
├── wallet.rs         # 钱包管理
├── smart_contract.rs # 智能合约支持
├── privacy.rs        # 隐私交易实现
├── pow.rs            # 工作量证明模块
├── utils.rs          # 辅助工具集
└── main.rs           # 主程序入口
```

---

## 核心功能

### 1. 区块与区块链
- **区块**：包含索引、时间戳、交易列表、哈希值等。
- **区块链**：管理区块的链式结构，支持创世区块、添加新区块、验证链完整性。
- **默克尔树**：用于高效验证交易数据的完整性。

### 2. 交易与钱包
- **交易**：支持发送方、接收方、金额和签名验证。
- **钱包**：生成密钥对、签名数据、验证签名。

### 3. 工作量证明（PoW）
- 通过调整 nonce 值，使区块哈希满足特定条件（前缀包含指定数量的 0）。
- 难度可配置，控制挖矿复杂度。

### 4. 智能合约（B3 独有拓展）
- 支持部署和执行简单的智能合约。
- 合约状态存储在区块链中。

### 5. 隐私交易（B3 独有拓展）
- 使用零知识证明（ZKP）保护交易隐私。
- 支持创建和验证隐私交易。

### 6. P2P 网络（B3 独有拓展）
- 实现节点间的区块链同步。
- 支持对等节点发现和连接。

---

## 使用指南

### 安装与运行
```bash
# 克隆仓库
git clone <仓库>
cd 

# 编译项目
cargo build --release

# 运行 B1 版本
cargo run --bin b1

# 运行 B2 版本
cargo run --bin b2

# 运行 B3 版本
cargo run --bin b3
```

### 命令行工具（B1/B3）
```bash
# 创建钱包
cargo run --bin b1 -- create-wallet

# 发起交易
cargo run --bin b1 -- add-transaction --sender <发送方地址> --receiver <接收方地址> --amount <金额>

# 挖矿
cargo run --bin b1 -- mine-block --miner <挖矿地址>

# 部署智能合约（B3）
cargo run --bin b3 -- deploy-contract --code <合约代码>

# 创建隐私交易（B3）
cargo run --bin b3 -- create-privacy-transaction --amount <金额>
```



## 扩展开发
## B3部分拓展功能代码部分未完全上传仓库，可见文档自行就行完善与解决，以下是部分扩展功能代码完善提示

### 1. 智能合约
当前 `smart_contract.rs` 实现了基础的合约功能，但以下部分需要完善：
- **合约语言支持**：扩展支持 Solidity 或其他高级合约语言。
- **Gas 机制**：为合约执行引入 Gas 费用机制。
- **事件日志**：添加合约事件日志功能。

示例代码：
```rust
// 扩展 Gas 机制
pub struct GasCounter {
    gas_limit: u64,
    gas_used: u64,
}

impl GasCounter {
    pub fn new(gas_limit: u64) -> Self {
        GasCounter { gas_limit, gas_used: 0 }
    }

    pub fn charge(&mut self, amount: u64) -> Result<(), String> {
        self.gas_used += amount;
        if self.gas_used > self.gas_limit {
            Err("Out of gas".to_string())
        } else {
            Ok(())
        }
    }
}
```

### 2. 隐私交易
当前 `privacy.rs` 实现了基础的零知识证明（ZKP）功能，但以下部分需要完善：
- **性能优化**：优化 ZKP 的生成和验证性能。
- **范围证明**：支持金额的范围证明。
- **批量验证**：支持批量验证隐私交易。

示例代码：
```rust
// 扩展范围证明
pub struct RangeProof {
    // 实现范围证明逻辑
}

impl RangeProof {
    pub fn new(amount: u64) -> Self {
        // 初始化范围证明
    }

    pub fn verify(&self) -> bool {
        // 验证范围证明
    }
}
```

### 3. P2P 网络
当前 `node.rs` 实现了基础的节点功能，但以下部分需要完善：
- **节点发现**：实现 Kademlia DHT 节点发现协议。
- **消息协议**：定义标准化的网络消息协议。
- **分片支持**：支持区块链分片技术。


### 4. 发币
*自行见文档进行自我完善与补充，本项目不提供完整发币代码*

示例代码：
```rust
// 扩展节点发现
pub struct NodeDiscovery {
    // 实现节点发现逻辑
}

impl NodeDiscovery {
    pub fn new() -> Self {
        // 初始化节点发现
    }

    pub fn discover_peers(&self) -> Vec<SocketAddr> {
        // 发现对等节点
    }
}
```


### B1 扩展建议
- 添加更详细的日志和调试信息。
- 实现跨链功能。

### B2 优化方向
- 使用更高效的哈希算法（如 Blake3）。
- 优化内存池管理。

### B3 高级功能 
- 发币
- 支持更复杂的智能合约语言（如 Solidity）。
- 增强隐私交易的性能。
- 跨链
- 分布式网络
- 其他拓展功能待完善

---


## 详细文档

- **B1 详细文档**：请进入 [b1/README.md](b1/README.md) 查看。
- **B2 详细文档**：请进入 [b2/README.md](b2/README.md) 查看。
- **B3 详细文档**：请进入 [b3/README.md](b3/README.md) 查看。

项目实际展示结果截图见产品说明pdf

---

## 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。

--- 

通过整合 B1、B2 和 B3，开发者可以根据需求选择合适的版本进行开发和学习。B1 适合初学者，B2 适合快速原型开发，B3 适合高级应用场景。