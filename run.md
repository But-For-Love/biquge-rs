两条命令，两个终端：

**终端 1 — 启动后端：**
```bash
cd biquge-rs
cargo run -p biquge-backend --release
```
后端启动后监听 `http://localhost:3000`，会自动创建数据库并写入演示数据。

**终端 2 — 启动前端：**
```bash
cd biquge-rs/frontend
trunk serve --open
```
前端启动后自动打开浏览器 `http://localhost:8080`，API 请求通过 Trunk 代理转发到后端。

---

如果不想每次重新编译后端，可以直接运行已编译好的二进制：
```bash
cd biquge-rs
target/release/biquge-backend.exe
```
