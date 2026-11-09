# Notist 原目录迁移试验

这是可重复构建的功能试验，不是最终主题或生产迁移。`docs/`、`gallery/`、`static/`、`build.rs` 和部署工作流未修改。所有新增实现都在本目录，生成文件在已忽略的 `target/notist-trial/`。

使用 `flake.lock` 中的 Notist（`85ab69e`），另需 Python 3.11+ 和 Git。以下命令在仓库根运行。

## 复现

```sh
# 纯配置与模板：没有任何 extension
nix develop --command notist --config experiments/notist/native.toml build

# 相同输入目录，加站点 prepare extension
nix develop --command notist --config experiments/notist/enriched.toml build

# 产物断言及转换测试
python3 experiments/notist/test_trial.py

# 浏览生成的增强样站（测试静态发布路径，而不是 SPA fallback）
python3 -m http.server 8766 --bind 127.0.0.1 --directory target/notist-trial/enriched
```

打开 http://127.0.0.1:8766/posts/ 和 http://127.0.0.1:8766/gallery/ 。实时预览也可以使用：

```sh
nix develop --command notist --config experiments/notist/enriched.toml serve --port 8767
```

## 实测结果

| 试验 | 结果 |
| --- | --- |
| 原生博客 target，`content = "docs"` | 3 页：首页和两篇 Markdown；两个 `.typ` 原样复制为资源，不是文章。 |
| 原生相册 target，`content = "gallery"` | 3 页：现有的分类/分组 Markdown 描述；不会扫描 Shadow refs，也不会生成图片卡片或不存在的相册首页。 |
| 增强 target | 7 页：首页、文章列表、4 篇正文、相册试验页。 |
| Shadow 接入 | 5 类、306 张图片：VRChat 18、ffxiv 153、地球 OL 103、异环 19、终末地 13。只读 ref，不恢复原图。 |
| 自动测试 | 8 项通过：页数、原始 Typst 资源复制、转换防护、4 篇文章的代码块渲染内容、评论路径、静态资源及本地链接、ref 全覆盖，以及旧路由移除和首页评论身份。 |
| 浏览器 | 已打开列表和转换后的文章；TOS 图片实际加载，点击图片打开原生 dialog 灯箱。抽查一个 TOS 对象 HEAD 返回 200 / image/jpeg。 |
| Waline 浏览器接入 | 初版在 `/posts/launchd-md/` 点击加载后，成功挂载客户端并显示服务端空评论列表；未填写或提交评论。随后按下述公开数据调查放弃旧 slug，现改用 `/posts/launchd.html`。 |

后续按用户决定取消旧 slug 兼容。Waline 的只读公开接口调查见 [WALINE-AUDIT.md](WALINE-AUDIT.md)：当前公开评论只发现首页 `/` 的 1 条，四篇旧文章地址均为 0。增强样站现保持 Markdown 的原路径，采用 Notist 默认 `.html` 输出；首页评论路径仍为 `/`。

## 哪些直接有，哪些需要加工

### 直接使用 Notist 配置、模板、CSS / JS 即可

- **现有 Markdown 正文**：原文件直接编译；标题、代码块、代码高亮、引用、强调、链接等不需要 extension。
- **模板和样式**：Tera 页面外壳、`site.pages` 导航、静态头像、Sass 编译、明暗样式。当前只做简单验证主题，没有复刻 aoike。
- **已有元数据的展示/排序**：若属性或 `site.data` 已有日期和摘要，可以由模板呈现；“获取 git 时间”不是模板职责。
- **文章导航列表**：模板直接遍历 `site.pages` 即可。独立 `/posts/` 列表页之所以在试验中生成，是因为原目录没有 `docs/posts/index.md`，不是 Notist 不会列页面。
- **Waline 客户端**：模板加普通浏览器 JS；不需要文档 extension。试验为点击后加载，不自动发请求，不提交测试评论。
- **图片网格与基础灯箱**：普通模板、CSS 和 JS 即可。试验已验证 dialog 打开/关闭；前后切换、原布局、时间线联动还未复刻。
- **静态发布**：产物就是 HTML / CSS / JS，可由普通 HTTP 服务器提供，不需要 WASM 应用。

### 保留当前源文件不变时，需要 Site prepare extension 或等价预处理

- **两篇 Typst 变成正文**：`prepare.py` 在内存中将当前两篇使用的标题、章节和 fenced code 子集转换为 Markdown；不是调用 Typst 编译器，也不是通用 Typst 前端。两篇原文件从未改写，代码块内容已逐块比对。更复杂的 Typst 仍需真正的转换适配/自定义前端。
- **git 创建/更新时间**：构建时调用 `git log`，将结果写入虚拟文档根属性。不需要在正文逐篇手填。
- **摘要**：试验生成近似的 200 字符文本摘要，和 aoike 的 HTML 摘要算法不同；要完全一致应复用旧算法。
- **旧 slug 兼容已移除**：Markdown 的文档路径不再改变，例如 `posts/launchd.md` 直接输出 `/posts/launchd.html`。Typst 转换后的虚拟文档采用同目录同 stem 的 `.md` 路径，例如 `posts/linux-kernel-module.md`，输出 `/posts/linux-kernel-module.html`。不生成旧地址别名或跳转页，评论使用新页面路径。
- **补不存在的入口页**：虚拟生成 `/posts/`、`/gallery/`，不需要在原目录创建 index 文件。
- **读取 Shadow refs 并解析 TOS URL**：扫描 ref、校验 SHA-256、按现有对象命名规则拼 URL，输出 `site.data.trial.gallery`，模板只负责展示。此步骤同样可以在构建前导出 JSON/TOML，不必依赖 Notist 专有协议。
- **组合不同内容目录**：原生 Site 只配置一个 content，本试验用两个原生 target 证明独立发现，再在增强 target 中通过 prepare 生成相册入口。若要分类和分组描述合并进同一站点，也应由 prepare 提供虚拟文档；本次尚未合并描述。

本次只用了 `[[site.extensions]]` 的一次性 **prepare**；没有使用文档级 `before_check` / `after_check` / `after_evaluate` extension。以上需求主要是输入发现、元数据和页面生成，不需要修改正文 AST。

### 还需要原图/额外实现，不能据此算已完成

- **EXIF、尺寸、拍摄日期、评分、颜色标签及旧时间线**：refs 只有哈希和文件大小。本机 gallery 没有恢复图片，不能凭文件名杜撰。需恢复原图后提取，或从旧构建导出可信 JSON 缓存。
- **相册旧分类 slug、评论身份及分组行为**：当前只做一个相册验证页，没有逐类兼容旧路由，也没有验证旧相册评论串。
- **生产 URL/锚点**：当前明确放弃旧 slug，外部旧链接不再由样站承接；新地址与标题锚点仍须预发布验收。Markdown 不再移动，保留原相对目录基准；Typst 转换如包含指向其它 `.typ` 的本地链接，仍需相应适配。
- **Waline 历史数据一致性**：生成的旧 path 有自动断言，但这不证明线上历史评论实际使用同一身份。不能仅凭挂载成功判定评论迁移完成。
- **原始 `.typ` 仍会被复制**：Site prepare 只能修改 `documents` / `data`，不能删除发现阶段的资源。因此增强 target 同样带原始 `.typ` 下载文件；如果不希望发布源码，需要隔离的 staging 内容目录/预处理资源清单，或扩展 Notist 的资源过滤能力，不能只靠当前 prepare 协议解决。
- **最终主题和交互**：当前摘要带原始 Markdown 标记、页面标题可能与正文标题重复、导航未做产品化筛选；只证明管线通，不代表视觉或行为等价。

## 建议

保留现在的目录结构完全可行。优先将“git 元数据 + 虚拟入口 + 相册数据适配”收敛成一个 Site prepare；不再维护旧 slug 路由。模板和前端交互保持普通 Tera/CSS/JS。若希望控制源文件公开范围、或未来 Typst 复杂化，则用独立 staging 预处理比继续扩大这一份试验转换器更稳妥。暂时不必新增文档级 extension。
