# 笔趣阁 (BiQuGe) - Rust + Yew

基于 Rust 全栈重写的笔趣阁小说阅读网站，前端严格还原原项目 PC 端设计。

## 技术栈

| 层 | 技术 |
|---|------|
| 后端 | Axum + libsql (SQLite) |
| 前端 | Yew (WASM) + yew-router |
| 样式 | 原版 CSS（716 行，零修改） |
| 存储 | 本地 libsql 数据库，首次运行自动建表并写入演示数据 |

## 架构

```
浏览器 (Yew SPA / WASM)
    │  HTTP fetch
    ▼
Axum API Server (:3000)
    │
    ▼
libsql 本地数据库 (data/biquge.db)
```

## 快速开始

**环境要求：** Rust 1.80+、wasm32-unknown-unknown target、Trunk

```bash
# 安装 wasm 目标
rustup target add wasm32-unknown-unknown

# 安装 Trunk
cargo install trunk
```

**终端 1 — 启动后端：**

```bash
cd biquge-rs
cargo run -p biquge-backend --release
# → http://localhost:3000
```

**终端 2 — 启动前端：**

```bash
cd biquge-rs/frontend
trunk serve --open
# → http://localhost:8080
```

## API 端点

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/home` | 首页数据（推荐、排行、分类、最新更新） |
| GET | `/api/categories` | 全部分类 |
| GET | `/api/novels?category_id=&page=&sort=` | 小说列表（分页） |
| GET | `/api/novels/:id` | 小说详情 + 章节目录 |
| GET | `/api/chapters/:novel_id/:chapter_id` | 章节内容 + 前后导航 |
| GET | `/api/search?q=` | 搜索小说 |
| GET | `/api/rankings?type=` | 排行榜（click / recommend / new） |

## 页面路由

| 路由 | 页面 |
|------|------|
| `/` | 首页 |
| `/category/:slug` | 分类列表 |
| `/novel/:id` | 小说详情 / 章节目录 |
| `/novel/:novel_id/:chapter_id` | 章节阅读页 |
| `/search/:q` | 搜索结果 |
| `/rankings` | 排行榜 |
| `/all` | 全部小说 |

## 阅读器功能

章节阅读页支持以下交互，偏好通过 LocalStorage 持久化：

- 字体切换（默认 / 宋体 / 黑体 / 微软雅黑 / 楷体）
- 文字颜色（9 色可选）
- 字号调整（10pt ~ 30pt）
- 背景色（9 色可选，含护眼模式）
- 页面宽度（50% ~ 95%）
- 夜间模式
- 自动滚动
- 键盘翻页（← →）

## 项目结构

```
biquge-rs/
├── Cargo.toml                  # Workspace
├── backend/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs             # Axum 服务启动
│       ├── db.rs               # libsql 连接 & 建表
│       ├── models.rs           # 数据模型 & 序列化
│       ├── seed.rs             # 演示数据（30 本小说，200+ 章节）
│       └── handlers/
│           ├── home.rs         # GET /api/home
│           ├── novels.rs       # GET /api/novels, /api/novels/:id
│           ├── chapters.rs     # GET /api/chapters/:novel_id/:chapter_id
│           ├── categories.rs   # GET /api/categories
│           ├── search.rs       # GET /api/search
│           └── rankings.rs     # GET /api/rankings
├── frontend/
│   ├── Cargo.toml
│   ├── Trunk.toml
│   ├── index.html              # Trunk 入口
│   ├── style.css               # 原版 PC 端样式（716 行）
│   └── src/
│       ├── main.rs             # Yew 应用入口
│       ├── app.rs              # 路由定义 & 页面切换
│       ├── api.rs              # API 客户端
│       ├── types.rs            # 前端类型定义
│       ├── components/         # 共享组件
│       │   ├── top_bar.rs      # 顶部工具栏
│       │   ├── header.rs       # Logo + 搜索
│       │   ├── nav.rs          # 主导航栏
│       │   ├── footer.rs       # 页脚
│       │   ├── novel_card.rs   # 小说卡片
│       │   ├── ranking_list.rs # 排行榜侧栏
│       │   ├── breadth.rs      # 面包屑
│       │   ├── pagination.rs   # 分页器
│       │   ├── reader_settings.rs  # 阅读设置面板
│       │   └── floating_buttons.rs # 浮动按钮
│       └── pages/              # 页面组件
│           ├── home.rs         # 首页
│           ├── category.rs     # 分类列表页
│           ├── novel.rs        # 小说详情页
│           ├── reader.rs       # 章节阅读页
│           ├── search.rs       # 搜索结果页
│           ├── rankings.rs     # 排行榜页
│           └── all_novels.rs   # 全部小说页
```
