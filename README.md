# AzurIce's Blog

冰弦のBlog，使用 [Notist](https://github.com/AzurIce/notist) SSG 发布。

## 开发与构建

```sh
nix develop
just build          # 产物：target/notist-site/
just check          # 内容、链接、元数据和画廊布局测试
just serve          # Notist 实时预览，默认 http://127.0.0.1:8000
```

直接运行：`nix develop --command just build`。首次需要下载/构建 flake.lock 固定的 Notist。上游 Nix 包支持 x86_64-linux、aarch64-linux、aarch64-darwin；Intel Mac 的旧开发环境仍保留，但不含 Notist。

`Notist.toml` 声明站点。原有 `docs/`、`gallery/` 和 `xiv-market-lite/` 目录不移动；静态资源统一放在 `site/static/`（头像为 `avatar.jpg`），样式在 `site/styles/`，根 `static/` 已移除。构建适配在 `tools/site_prepare.py`。不兼容旧文章 slug，文章按原相对路径生成 `.html`。

## 相册

图片继续使用 Shadow refs 对应的 TOS 公共对象，不打包进 Pages。元数据是中间构建产物，保存在被 Git 忽略的 **`target/gallery-metadata/metadata.json`**，不再提交 `site/data/gallery-metadata.json`。

`just build` / `just serve` 会自动检查每个 ref：同一内容哈希直接复用缓存，仅获取并完整解析缺失、损坏或版本过期的对象。图片移动、重命名或重复引用不需要重新下载。也可以单独预热缓存：

```sh
just gallery-sync
just build
```

缓存采用 `version: 2`（按哈希索引的聚合格式）和独立的 `extractor_version: 3`（完整对象提取逻辑）。优先使用已恢复的本地原图，否则按 Shadow ref 定位公共 TOS 对象；完整校验大小及 SHA-256 后，扫描 JPEG 元数据和 PNG 全部 chunk，不再使用头尾 Range 近似。

下载流式写入临时文件，解析后立即删除，不恢复到原始 `gallery/`，也不把原图放入缓存。冷缓存首次会读取当前约 668 MB 图片；暖缓存可以离线构建。失败时构建停止，但成功提取的条目会原子保存，重试只补缺失部分。`python3 tools/gallery_metadata.py --refresh` 可强制重新提取。

CI 使用 Actions Cache 恢复/保存这一个 JSON。缓存键区分提取器/工具链和 refs，通过恢复前缀复用旧 refs 的缓存；即使构建失败也保留已完成的条目。缓存服务不可用只影响性能，不改变构建结果。当前对象为公开访问，无需 TOS 密钥；冷缓存构建需要访问对象存储。

## 发布与主题

GitHub Actions 使用同一 Nix/Just 构建和测试，发布 `target/notist-site/`。`404.html` 是真实 404 页面，不再复制首页充当 SPA fallback。`xiv-market-lite/` 原样包含在产物中；其自身的远程依赖没有改写。

主题以 2026-11-06 的线上 aoike 版本为参考：粉色强调色 `#f67c91`、白底、顶栏、首页头像区、正文、文章卡片、画廊时间线/年份筛选、评分布局/红色精选和底部评论抽屉。布局算法由 aoike 的 Rust 实现移植为可测试的纯 JS；不依赖 `static/css/` 的旧生成文件。见 [site/README.md](site/README.md)。

正文 Markdown 由 Notist 直接解析；两篇 Typst 仅转换当前使用的标题、段落和 raw code 子集，不是通用 Typst 支持。原 `.typ` 目前也作为可下载资源发布。Waline 服务不变，首页评论路径仍为 `/`，文章采用新路径；没有修改评论数据库。

旧 Rust/Sycamore 实现保留，可用 `just build-legacy` 构建；不会被当前部署工作流调用。先前的探索记录在 `experiments/notist/`。
