# 站点迁移到 Notist 的可行性评估

评估日期：2026-11-06。结论：**可行，但属于重做展示层，不是替换一个依赖即可完成。**

迁移约束：不兼容旧文章 slug，文章采用 `.html` 路径，首页 Waline path 保留 `/`。2026-11-06 的只读公开接口查询发现首页 `/` 有 1 条公开评论，四篇旧文章路径均为 0；该查询不覆盖待审核等非公开记录，不代表数据库全量调查。下文旧 URL 兼容建议不作为当前目标。

正式实现已完成：根 `Notist.toml`、`site/` 主题、`tools/site_prepare.py` 及新的 Just/CI 发布链。默认构建为 `just build`，输出 `target/notist-site/`；旧 Rust 实现保留。实际使用说明以 [README.md](README.md) 和 [site/README.md](site/README.md) 为准，以下为迁移前评估记录。

建议使用 Notist 的 `[[site]]` / SSG，而非面向章节文档的 Book。先迁移博客，再迁移相册；不必先把所有 Markdown 改写成 `.not`。

## 评估依据与边界

- 本站：`build.rs`、`src/main.rs`、`docs/`、`gallery/`、`shadow.toml` 和 `.github/workflows/deploy.yml`。
- Notist：以本次 `flake.lock` 固定的远端提交 `85ab69e13bb99cb4016538accdd55c65f0cb6d7c` 为准；本机 checkout 更新，不能用它的未发布能力作为承诺。
- 核心依据：Notist 的 `docs/ssg.not`、`crates/notist-ssg/src/{config,build,routes}.rs`、`crates/notist-pipeline/src/frontend.rs`。
- aoike 功能实现参考本机相邻仓库；正式迁移时仍须对照本站 `Cargo.lock` 和线上行为验收。
- 本次只添加开发工具并做源码级评估，没有转换正文、改部署或验证整站迁移后的浏览器效果。

## 当前规模

- 首页 1 个 Markdown 文件。
- 文章 4 篇：Markdown 2 篇（204 行）、Typst 2 篇（414 行）。
- 相册有分类和分组描述，图片由 Shadow / 火山 TOS 管理；复杂度不能仅按仓库中 Markdown 文件数量估计。
- 当前展示框架为 Sycamore SPA；实际评论系统是 Waline，首页文字中的 Giscus 描述已过时。

## 功能匹配

| 功能 | 可行性与迁移工作 |
| --- | --- |
| 首页、导航、个人信息 | 可用 Tera 模板和 `site.data` 重建；现有 Rust 配置不直接复用。 |
| Markdown 正文 | Notist 原生支持 `.md`；先保持原格式，检查表格、删除线、代码块、HTML 和链接的渲染差异。 |
| Typst 正文 | 默认前端只有 Notist 和 Markdown，不会把 `.typ` 编译成文章，而是按普通资源复制。建议将这两篇转换为 `.not` 或 Markdown；其中主要是标题、段落和代码块，不值得为此新增整套 Typst 前端。Typst 数学/组件支持不等于支持完整 `.typ` 文档。 |
| 文章列表、摘要、时间 | `site.pages` / `page.attrs` 提供基础数据；文章筛选、日期排序和摘要需模板/预处理补齐。aoike 的 git 创建/更新时间不会自动继承，重命名前应导出并固定。 |
| 样式与主题 | 支持主题、静态资源、Sass 和代码高亮；需移植 aoike 源样式并调整 DOM。不要把本站自动生成的 `static/css` 当成长期样式源码；UnoCSS 工具类也不能假设会自动生成。 |
| Waline 评论 | 可在文章/相册模板加载 Waline 客户端，继续使用现有服务端。必须显式保留旧 `path`，不能直接采用新页面 URL。 |
| 相册数据 | 没有发现等价的内置相册设施。需保留/提取 EXIF、尺寸、日期、评分、颜色标签、分组和排序，以及 Shadow ref 到 TOS URL 的解析。 |
| 相册交互 | 现有 Sycamore 灯箱、前后切换和联动滚动不能直接搬到 Tera；需独立 JS / Web Component，或暂时保留旧相册应用。 |
| GitHub Pages | 可直接发布 Notist 生成的静态 HTML，无须 Rust WASM/Trunk 运行时；CI 应继续保留所需 Shadow 恢复和密钥流程。 |

## 关键风险

### 旧 URL 和评论身份

当前路由为 `/posts`、`/posts/<slug>`、`/gallery`、`/gallery/<slug>`。Notist 默认把文档扩展名替换为 `.html`，目录 `index` 的公开 URL 为尾斜杠形式。

建议将文章放到 `posts/<原 slug>/index.md`（或 `.not`），相册同理，用真实目录入口承接 Pages 的目录访问。不能假设 `/posts/x`、`/posts/x/` 和 `/posts/x.html` 完全等价；需要在 Pages 环境验证无斜杠访问、跳转、中文编码和旧标题锚点。无法自然承接的地址需要兼容页；Pages 本身不提供任意服务端重写规则。

Waline 的 `path` 应独立于新路由存储，例如继续使用 `/posts/<原 slug>`。同样要核对相册和首页评论身份。不要直接沿用当前 CI 的“复制首页为 404”做法，那是 SPA fallback，不适用于多页静态站。

### 相册是最大工作量

建议将相册处理拆成两个独立部分：构建前导出 JSON/TOML 元数据，Tera 渲染相册页并用轻量 JS 实现灯箱。初期可复用 aoike 的构建期解析以降低行为差异，后续再决定是否移出依赖。

Notist 可以通过实验性 `site.extensions` 在构建时生成文档和数据，但不建议把第一版迁移全部绑定在实验协议上；普通预处理脚本加 `data/` 更容易排错和固定版本。外链 TOS 图片可以继续使用，不需要把原图打进 Pages 产物；若构建依赖原图中的 EXIF，仍需恢复原图或提前缓存元数据。

### 工具链与成熟度

本次让 Notist 保留其上游锁定的 nixpkgs、Rust overlay 和 nightly，不强行跟随本站较旧的工具链。代价是额外下载和首次 Rust 编译开销。上游 flake 当前只提供 x86_64-linux、aarch64-linux、aarch64-darwin 包；本站仅在上游支持的平台添加 Notist，x86_64-darwin 保留原开发环境但不含 Notist，需要上游补包后才能使用。

Notist 的 SSG 有主题/模板/数据能力，但不提供完整博客产品层；迁移会重新引入本站首页提到过的模板维护成本。建议固定提交，在样站验证后再升级。

## 建议迁移顺序与验收

1. 在独立预览目录建立最小 Site：先接入首页、一篇原始 Markdown 和一篇转换后的 Typst；暂不改生产部署。
2. 补首页、列表、文章模板和样式；固定标题、slug、摘要、创建/更新时间以及 Waline 评论路径。
3. 导出相册数据并重建交互，或在过渡期将原相册作为独立应用保留。后者不是“全站完成迁移”。
4. 验证全部旧 URL、锚点、评论串、移动端布局、明暗主题、代码高亮、图片和相册元数据；再切换 Pages 工作流，保留旧构建链作为回退。

**推荐判断：** 若目标是统一 Notist 内容管线、减少 SPA/WASM 依赖，这次迁移值得做；若目标只是降低短期维护工作量，保留 aoike 更省事。主要投入在主题与相册，不在这 4 篇正文。未做样站前，不应承诺精确工期或性能提升。
