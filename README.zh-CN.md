# Tinkora Markdown Porter

Markdown Porter 是一个本地 Markdown 检查器和安全 HTML 导出工具。Rust
解析器编译为 WebAssembly，在浏览器中处理私密文档，支持预览结果并下载
一个自包含的 HTML 文件。

项目当前处于 **Alpha**。首个公开版本会保持窄范围：不承诺 PDF 生成、完整
GitHub 渲染、内置语法高亮或 MCP transport。

## 解决的问题

在线 Markdown 编辑器很方便，但把私密文档粘贴到未知服务会增加不必要的数据
边界。Markdown Porter 为开发者和 Agent 提供一个小而可审计的本地流程，用于：

- 预览常用 GFM 结构；
- 检查纯文本和 YAML frontmatter 输出；
- 导出不依赖外部资源的自包含 HTML 文件。

## 安全契约

- Markdown 只在浏览器本地处理，没有服务器或上传路径。
- 原始 HTML 会作为文本显示，不会执行。
- 保留 `http`、`https`、`mailto`、相对路径和 fragment 链接；危险 scheme 会替换为本地阻断标记。
- 图片引用显示为文本标签，预览和导出文件不会抓取远程资源。
- Markdown 输入上限为 4 MiB，自定义导出 CSS 上限为 256 KiB。
- 导出文件内嵌 CSS，不包含 JavaScript。

## 支持的 Markdown

当前解析器启用表格、任务列表、删除线、脚注、自动链接、标题、列表、引用和
围栏代码块。代码语言 class 会保留，供使用者自行提供高亮。原始 HTML、远程
图片和任意 URL scheme 不属于当前契约。

## 本地运行

```bash
git clone https://github.com/Tinkora/md_porter.git
cd md_porter
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
wasm-pack build --target web crates/md_porter_web --out-dir static/pkg
python3 -m http.server 8080 --directory crates/md_porter_web/static
```

构建完成后打开 `http://127.0.0.1:8080/`。浏览器页面就是产品，不需要配置后端。

## 开发

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p md_porter_web --target wasm32-unknown-unknown
```

前端变更还必须在真实浏览器中验证 375、768、1024 和 1440 像素视口。具体 UI
流程和安全边界见 [AGENTS.md](AGENTS.md)。

## Agent 集成状态

仓库包含一个描述本地 Markdown 操作的草案 schema，但它**不是 Agent-callable**：
当前没有可运行的 MCP server、注册流程、权限模型或端到端 transport 测试。

## 文档

- [English README](README.md)
- [产品规格](docs/product_spec.zh-CN.md)
- [Security policy](SECURITY.md)
- [Contributing guide](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)

## 许可证

MIT，见 [LICENSE](LICENSE)。
