# 种养基地生产档案系统

## 项目简介

种养基地生产档案系统是一套面向农业种养基地的综合管理平台，实现了农作物播种管理、施肥用药记录、出栏/收成台账和农产品溯源等核心功能。系统采用 Rust 语言开发，基于 Axum Web 框架，使用 SQLite 数据库存储数据，Tera 模板引擎渲染页面。

## 技术栈

| 技术 | 版本 | 说明 |
|------|------|------|
| Rust | 1.77+ | 编程语言 |
| Axum | 0.7 | Web 框架 |
| SQLite | - | 嵌入式数据库（通过 rusqlite） |
| Tera | 1.x | 模板引擎 |
| bcrypt | 0.15 | 密码加密 |
| tower-http | 0.5 | 静态文件服务 |

## 功能模块

### 1. 农作物播种管理
- 播种记录的增删改查
- 支持作物名称、品种、种植面积、播种日期、预计收获日期、状态、基地编号等字段
- 播种详情页关联展示施肥用药和收成记录

### 2. 施肥用药记录
- 记录每次施肥或施药的详细信息
- 区分肥料和农药两种类型
- 支持用量、单位、施用日期、操作人、备注等字段
- 与播种记录关联

### 3. 出栏/收成台账
- 记录收获日期、产量、品质等级、买家、单价等信息
- 支持按播种记录查询关联的收成数据
- 品质等级分为优等、一等、二等、合格

### 4. 农产品溯源
- 通过唯一溯源码查询产品完整信息链
- 展示从种植到收获的全过程
- 包含种植信息、施肥用药记录和收获检验结果
- 支持公开查询（无需登录）

### 5. 用户认证
- 用户注册与登录
- bcrypt 密码加密
- Cookie 会话管理
- 登录状态中间件验证

## 项目结构

```
repo/
├── Cargo.toml              # 项目配置与依赖
├── src/
│   ├── main.rs             # 程序入口，路由配置
│   ├── config/             # 配置模块
│   │   ├── app.rs          # 应用配置
│   │   ├── database.rs     # 数据库初始化与迁移
│   │   └── seed.rs         # 种子数据
│   ├── handlers/           # 处理器（控制器）
│   │   ├── auth.rs         # 认证相关
│   │   ├── planting.rs     # 播种管理
│   │   ├── chemical.rs     # 施肥用药
│   │   ├── harvest.rs      # 收成台账
│   │   ├── traceability.rs # 产品溯源
│   │   └── home.rs         # 首页与工作台
│   ├── middleware/          # 中间件
│   │   └── auth.rs         # 认证中间件
│   ├── models/             # 数据模型
│   │   ├── user.rs         # 用户模型
│   │   ├── planting.rs     # 播种模型
│   │   ├── chemical.rs     # 施肥用药模型
│   │   ├── harvest.rs      # 收成模型
│   │   └── traceability.rs # 溯源模型
│   └── services/           # 业务逻辑层
│       ├── user_service.rs
│       ├── planting_service.rs
│       ├── chemical_service.rs
│       ├── harvest_service.rs
│       └── traceability_service.rs
├── templates/              # Tera 模板
│   ├── base.html           # 基础布局
│   ├── index.html          # 首页
│   ├── partials/           # 公共组件
│   ├── auth/               # 认证页面
│   ├── plantings/          # 播种页面
│   ├── chemicals/          # 施肥用药页面
│   ├── harvests/           # 收成页面
│   ├── traceability/       # 溯源页面
│   └── dashboard/          # 工作台
└── static/                 # 静态资源
    ├── css/style.css
    └── js/main.js
```

## 快速开始

### 本地开发

```bash
cd repo
cargo run
```

服务器将在 `http://localhost:3000` 启动。

### Docker 部署

```bash
cd farm-archive
docker build -t farm-archive .
docker run -d -p 3000:3000 -p 2222:22 farm-archive
```

### 默认账号

| 用户名 | 密码 | 姓名 | 角色 |
|--------|------|------|------|
| admin | admin123 | 管理员 | admin |
| zhangwei | zhang123 | 张伟 | operator |
| lina | li123 | 李娜 | operator |

## 数据库设计

### users 表
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键 |
| username | TEXT | 用户名（唯一） |
| password_hash | TEXT | bcrypt 密码哈希 |
| real_name | TEXT | 真实姓名 |
| role | TEXT | 角色（admin/operator） |
| created_at | TEXT | 创建时间 |

### plantings 表
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键 |
| crop_name | TEXT | 作物名称 |
| variety | TEXT | 品种 |
| area | REAL | 面积（亩） |
| planting_date | TEXT | 播种日期 |
| expected_harvest_date | TEXT | 预计收获日期 |
| status | TEXT | 状态（growing/harvested） |
| base_id | TEXT | 基地编号 |
| created_by | INTEGER | 创建人 |
| created_at | TEXT | 创建时间 |

### chemicals 表
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键 |
| planting_id | INTEGER | 关联播种ID |
| chem_type | TEXT | 类型（fertilizer/pesticide） |
| name | TEXT | 名称 |
| dosage | TEXT | 用量 |
| unit | TEXT | 单位 |
| application_date | TEXT | 施用日期 |
| operator | TEXT | 操作人 |
| notes | TEXT | 备注 |
| created_by | INTEGER | 创建人 |
| created_at | TEXT | 创建时间 |

### harvests 表
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键 |
| planting_id | INTEGER | 关联播种ID |
| harvest_date | TEXT | 收获日期 |
| quantity | REAL | 产量 |
| unit | TEXT | 单位 |
| quality_grade | TEXT | 品质等级 |
| buyer | TEXT | 买家 |
| price | REAL | 单价 |
| notes | TEXT | 备注 |
| created_by | INTEGER | 创建人 |
| created_at | TEXT | 创建时间 |

### traceability 表
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键 |
| code | TEXT | 溯源码（唯一） |
| planting_id | INTEGER | 关联播种ID |
| harvest_id | INTEGER | 关联收成ID |
| product_name | TEXT | 产品名称 |
| origin | TEXT | 产地 |
| harvest_date | TEXT | 收获日期 |
| inspection_result | TEXT | 检验结果 |
| created_at | TEXT | 创建时间 |

## API 路由

| 方法 | 路径 | 说明 | 需要登录 |
|------|------|------|----------|
| GET | / | 首页 | 否 |
| GET | /login | 登录页面 | 否 |
| POST | /login | 登录 | 否 |
| GET | /register | 注册页面 | 否 |
| POST | /register | 注册 | 否 |
| POST | /logout | 退出 | 是 |
| GET | /dashboard | 工作台 | 是 |
| GET | /plantings | 播种列表 | 是 |
| GET | /plantings/new | 新增播种页面 | 是 |
| POST | /plantings | 新增播种 | 是 |
| GET | /plantings/{id} | 播种详情 | 是 |
| GET | /plantings/{id}/edit | 编辑播种页面 | 是 |
| POST | /plantings/{id} | 更新播种 | 是 |
| GET | /chemicals | 施肥用药列表 | 是 |
| GET | /chemicals/new | 新增施肥用药页面 | 是 |
| POST | /chemicals | 新增施肥用药 | 是 |
| GET | /chemicals/{id} | 施肥用药详情 | 是 |
| GET | /harvests | 收成列表 | 是 |
| GET | /harvests/new | 新增收成页面 | 是 |
| POST | /harvests | 新增收成 | 是 |
| GET | /harvests/{id} | 收成详情 | 是 |
| GET | /traceability | 溯源查询 | 否 |
| GET | /traceability/list | 溯源记录管理 | 是 |
| GET | /traceability/{code} | 溯源详情 | 否 |

## 许可证

MIT License
