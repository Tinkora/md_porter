# md_porter 产品规格

## 产品一句话

让用户在浏览器中编写 Markdown，实时预览安全的 HTML，并导出一个不依赖
外部资源的自包含 HTML 文件。所有处理在浏览器内完成，由 Rust WASM 驱动。

## 目标用户

- 需要快速检查 README、变更说明和技术文档渲染结果的开发者。
- 不希望把私密 Markdown 粘贴到在线服务的个人和团队。
- 需要先由 Agent 生成草稿、再由人确认并导出的文档工作流。

## 核心体验

1. 用户打开编辑器或粘贴已有 Markdown。
2. 左侧编辑 Markdown，右侧实时显示安全预览。
3. 工具栏插入常见 Markdown 语法，快捷键支持加粗、斜体和链接。
4. 自动读取 YAML frontmatter 并在状态栏显示键名。
5. 通过导出操作下载自包含 HTML 文件。
6. 预览和导出不抓取 Markdown 中的远程图片或其他外部资源。

## 技术架构

- **解析引擎**：Rust `pulldown-cmark` 0.11，编译为 WASM。
- **GFM 扩展**：表格、任务列表、删除线、脚注、自动链接和标题属性。
- **前端**：纯 HTML/CSS/JS，无框架依赖。
- **导出**：浏览器内 Blob 下载，无服务器交互。

## Markdown 支持

| 特性 | 语法 | 状态 |
| --- | --- | --- |
| 标题、强调、删除线 | CommonMark/GFM | 支持 |
| 行内代码和围栏代码块 | `` `code` `` / ````` | 支持，保留语言 class |
| 引用、列表、分割线 | CommonMark | 支持 |
| 任务列表 | `- [ ]` / `- [x]` | 支持，预览中不可交互 |
| 表格 | GFM table | 支持 |
| 脚注和自动链接 | GFM | 支持 |
| 链接 | `http`、`https`、`mailto`、相对和 fragment | 危险 scheme 会被阻断 |
| 图片 | `![alt](url)` | 显示文本，不抓取资源 |
| 原始 HTML | `<span>` | 转义为文本 |
| YAML Frontmatter | `---` 包裹 | 支持键值提取 |

## 安全与资源边界

- Markdown 输入上限为 4 MiB。
- 自定义导出 CSS 上限为 256 KiB。
- 原始 HTML 作为文本显示，不执行脚本或事件属性。
- `javascript:`、`data:`、`vbscript:`、协议相对 URL 和控制字符 URL 会被阻断。
- 图片引用转为文本标签，避免浏览器或导出文件发起外部请求。
- 导出 HTML 不包含 JavaScript、外部 CSS、字体或图片资源。

## 非目标

- 不做在线托管、分享或协作编辑。
- 不做 PDF 直接导出；用户可以使用浏览器打印为 PDF。
- 不做图片上传、图片代理或远程资源抓取。
- 不承诺完整 GitHub Markdown 渲染或内置语法高亮。
- 不提供可运行的 MCP server 或隐式 Agent 权限。

## 验收标准

- 空输入返回 `EMPTY_INPUT`，超限输入返回 `INPUT_TOO_LARGE`。
- GFM 表格、任务列表、删除线和脚注生成稳定 HTML。
- 原始 HTML 被转义，危险链接不包含原始 scheme，图片不生成 `<img>`。
- YAML Frontmatter 正确解析为键值对。
- 导出标题和自定义 CSS 不能突破 HTML 文档边界。
- WASM 编译通过 `wasm32-unknown-unknown` target。
- 浏览器在 375、768、1024 和 1440 像素视口完成关键流程验证。
