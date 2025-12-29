# 白开水 (Boiled Water)

[Eng](./README.md) | 中文

**Boiled Water** (工程代号 `boilw`) 是 **仿生智能 (Bionic Intelligence, BI)** 架构的参考实现。

仿生智能即通过符号主义理念仿照人类的思维结构，构建强逻辑机制来实现智能体。它属于严格意义上的**逻辑模型**，旨在通过确定的逻辑路径构建智能，与基于概率的 LLM 完全不同。

## 核心概念

`boilw` 建立在以下基础概念之上：

- **Flag 系统**: 系统的原子单位。所有的概念、事件、类别、物体乃至逻辑本身都被解构为 Flag 节点。
- **拓扑即逻辑**: 逻辑门和推理过程并非通过权重计算，而是直接由 Flag 之间的图谱连接结构决定。
- **记忆架构**: 记忆被明确区分为储存“经历”（场景事件）和储存“知识”（Flag 图谱）两个大类，支持从临时记忆到永久记忆的转化。

## 项目结构

本仓库采用单体仓库 (Monorepo) 结构。

```text
bi-boiled-water/
├── crates/
│   └── (核心组件目前处于私有开发阶段)
└── examples/
    ├── simple-flag/      # 演示基础的 Flag 系统与推理流程
    └── basic-logic/      # 演示通过拓扑结构构建的基础逻辑门
```

> **注意**: 核心运行时 (`boilw-core`) 及标准插件目前仍在开发中，尚未公开。本仓库中的示例项目包含了简化版的内联内核代码，用于演示架构原理。

## 使用方法

运行示例代码前，请确保已安装 Rust 工具链。

```bash
# 克隆仓库
git clone https://github.com/EyKettle/bi-boiled-water.git
cd bi-boiled-water

# 运行 Simple Flag 示例
cargo run -p simple-flag

# 运行 Basic Logic 示例
cargo run -p basic-logic
```

## 许可证 (License)

本仓库当前公开的内容（示例代码及文档）基于 **Apache License, Version 2.0** 发布。

详情请参阅 [LICENSE](./LICENSE) 文件。

---

**Copyright © 2025 Boiled Water Project.**
