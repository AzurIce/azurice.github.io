# Notist 主题与站点适配

## 文件边界

- `../Notist.toml`：生产 Site target；`content = "docs"`，不移动原始内容。
- `templates/page.tera`：首页、文章、列表、搜索、画廊与 404；只插入 Notist 编译的 `page.html`。
- `styles/`：统一的样式源码目录，包含原 SCSS、UnoCSS 快照与 Notist 适配，不依赖被 Git 忽略的 `../static/css/`。`main.scss` 是唯一编译入口，导入 `_legacy.scss` 后应用适配，由 Notist 内置 Sass 编译成 CSS。
- `static/site.js`：标题锚点、搜索、滚动进度、年份筛选、灯箱与按需 Waline。
- `static/avatar.jpg`：头像与 favicon，直接发布为 `/avatar.jpg`；根目录不再保留 `static/`。
- `static/gallery-layout.mjs`：移植 aoike 的动态规划布局，包括竖图压力、评分对比、红色精选置顶、桌面双图精选和移动端单图精选。图片在各分区内按确定的 ref 路径排序，不依赖操作系统的目录迭代顺序。
- `data/config.toml`：个人信息及 Waline 服务配置。
- `../target/gallery-metadata/metadata.json`：生成的单文件元数据缓存，按图片内容哈希索引，不进 Git。`data/` 只保存手写站点配置。
- `../tools/site_prepare.py`：git 时间、摘要、Typst 子集转换、虚拟入口、相册描述/时间线；不改源文件。
- `../tools/site_summaries.py`：用同一 Notist SSG 无主题渲染文章，再提取保留 HTML 结构的卡片摘要；搜索纯文本与摘要分离。
- `../tools/build_site.py`：先暂存独立 XIV 页面，再运行 Notist。仅在带所有权标记的 `target/site-static/` 内替换生成资源；主题静态资源由 Notist 直接从 `site/static/` 收集。
- `../tools/site_assets.py`：Site prepare extension，每次构建/重建将 UnoCSS 快照原子更新到 `target/site-static/css/uno.css`，再由普通 static 管线发布。

## 临时复用旧样式

当前采用旧主题快照，不在构建时重新运行 UnoCSS：`styles/uno.css` 包含原 preflight、工具类、图标和 `nav-btn` shortcut。模板先加载 `/css/uno.css`，再加载 `/main.css`，最后加载 Notist 语法高亮。新增工具类仍需后续接入 UnoCSS 生成器，不能假定任意类名都已存在。

`styles/_legacy.scss` 是重命名为 partial 的旧主题入口，原样导入颜色变量、正文、评论、滚动和相册样式；`styles/main.scss` 保留当前语义类布局、Notist 标题锚点、评论状态及 dialog 灯箱等适配。SCSS 必须编译后才能供浏览器使用，前导 `_` 防止被 Notist 当成独立入口额外输出。快照进入版本管理；干净 checkout / CI 不需要先运行旧 Rust 构建生成 `static/css`。不改写旧目录，也不从该目录隐式刷新快照。修改 `site/styles/` 会触发 serve 重建；extension 监听的是源目录而不是 `target/` 产物。

## 渐进增强

内容、文章链接、分类入口和图片都在静态 HTML 中。禁用 JS 时仍能阅读和浏览；JS 只增强搜索、年份筛选、精确排布、灯箱和评论。默认选择最新年份，可用 `?year=2024` 分享年份状态。灯箱支持按钮、左右方向键、触摸滑动、Escape、焦点返回，并联动底下的图片位置。

底部评论抽屉保持线上外观，但改为第一次展开才加载固定版本 Waline；不主动发送测试评论。弹层有键盘焦点约束和背景 inert。首页 `/` 的评论身份保留；放弃旧文章 slug，详见 `../experiments/notist/WALINE-AUDIT.md`。

## 相册数据的真实性

`tools/gallery_metadata.py` 根据 `.shadow/refs/gallery` 管理一个聚合缓存：

```json
{
  "version": 2,
  "extractor_version": 3,
  "objects": {
    "sha256:…": {"size": 1234, "format": "jpeg", "width": 1920, "height": 1080}
  }
}
```

格式版本与提取器版本独立，变更提取语义时必须升级提取器版本。缓存条目只保存图片自身的信息；文件名日期、路径、分类和分组从当前 refs/描述文件每次重新组合，因此移动、重命名及重复引用复用同一个条目。

构建的 Site prepare 自动检查并补齐缓存，`just gallery-sync` 仅是独立预热入口。Shadow CLI 当前没有按文件 restore 参数，因此缓存缺失时按 ref 的同一对象命名规则直接获取公共 TOS 对象，不调用全量 `shadow restore`。优先选择相同哈希的任一已恢复本地原图；原图与 ref 不一致会失败，不静默使用修改后的文件。

所有获取的对象都完整校验大小与 SHA-256。以最多 1 MiB 的块写入临时文件，解析后删除；每次只处理有限并发，不把整个相册放进内存或还原进 `gallery/`。JPEG 扫描全部标准头部元数据，PNG 遍历全部 chunk（跳过像素载荷而不解码），能读取 IDAT 之后或文件中部的 EXIF/XMP。元数据块和解压结果有 16 MiB 上限，遇到非法结构/CRC、错误对象或提取失败会阻止发布。

日期优先使用 EXIF，然后使用旧实现同样支持的文件名日期格式；未知日期显示“日期未知”，不使用本机恢复时间或伪造拍摄时间。JPEG EXIF 日期同时接受冒号和连字符格式；评分/颜色没有值就留空。

单个 JSON 用临时文件 + 原子替换写入，每完成一个对象即持久化；网络/解析失败或进程中断不要求重做已经完成的对象。进程锁防止并行构建重复下载或覆盖缓存。未变化的暖缓存不联网，也不重写 JSON；格式/提取器版本不兼容或整个 JSON 损坏会重新生成，单条目损坏仅修复该条。

CI 只缓存这个 JSON，不缓存临时图片。键包含运行平台、提取脚本/工具链摘要、refs 摘要及唯一运行标识；恢复先匹配当前 refs，再退回同提取脚本的较旧缓存，以补齐而非全量重算。唯一运行标识使“同 refs 但修复了缓存”和失败构建的有效部分也可保存，避免 Actions Cache 不可变键导致修复无法持久化。缓存不是事实来源：丢失只会重新读取原图。

分组描述交给 Notist 编译为额外文档，模板复用其已编译 HTML，因此没有另一套 Markdown 正文渲染器。原相册用系统目录遍历顺序，本版使用确定的文件路径顺序，图片先后可能与旧产物不同，但分类、分组、年份、评分/颜色语义保持。

## 文章摘要

列表和搜索卡片不再使用源码正则清理后的前 200 个字符。摘要从 Notist 渲染的 HTML 中提取：移除 h1 及其内容、保留 h2 等其他标题、段落、链接、强调、行内代码、代码块及高亮，截取 200 个非空白文字字符，超出后加 `...` 并补齐闭合标签。HTML 实体按解码后的字符计数，不复刻旧版可能切断实体/错误计算实体长度的行为；截断点不受两个渲染器的引号转义写法影响。

Site prepare 尚拿不到当前构建的 HTML，因此每次用同一个 `notist` 可执行文件做一次仅含文章、无主题的 Site 构建，输入来自当前内存里的 Markdown/已转换 Typst，root 与 content 路径不变。临时配置和输出仅位于 `target/`，成功或失败后都会清理；不写入原始内容，也不递归调用主 prepare。`build` 与 `serve` 走同一条处理路径，不依赖上次构建的摘要缓存。若以后增加文档级全局 extensions，需要同步这份无主题渲染配置。

摘要中的相对链接/图片基于原文章 URL 转为站点绝对路径，去除重复 id；代码块取消浏览器默认外边距并沿用旧卡片的等宽字体，超宽内容可横向滚动，不设统一卡片高度。文章标题提取规则不在这次调整范围内。搜索索引继续使用原来的纯文本逻辑。

可选命令 `just check-summary-parity` 使用与浏览器测试相同的 Chromium / playwright-core 环境，只读比较部署版与本地四篇文章的摘要非空白文本、代码块数量和桌面摘要区域高度。此次四篇均匹配，结果/截图写入 `target/notist-verification/summaries/`；不表示 Notist 与旧 Typst 渲染器的高亮颜色或全部 HTML 字节完全相同。

Typst 转换保留 `shell-unix-generic`，Notist 能直接识别。主题补齐默认高亮 CSS 未覆盖的 shell command / tilde scope，沿用旧版蓝色 `#4b69c6` 和红色 `#d73948`；其他语法仍使用 Notist 调色板。`node tools/tests/highlight.test.mjs`（同样使用 `SITE_URL`、`PLAYWRIGHT_MODULE` 和 `CHROMIUM_PATH`）验证列表、搜索与正文的实际 computed color，而不只检查 span 是否存在。

## 运行与检查

在根目录的 Nix shell 中执行 `just build`、`just check`、`just serve`。`just serve` 可热更新正文、模板、样式、脚本、`site/static/` 资源和相册数据；独立 `xiv-market-lite/` 的变化需要重启一次 serve 以重新暂存资源。旧版 `just build-legacy` 仍会自动生成根 `static/css/`，但主构建不会创建或读取它；旧 Trunk 入口从 `site/static/` 复制资源到旧产物的 `/static/`。

浏览器验收应对照线上首页、文章列表、正文、画廊及移动端；静态检查覆盖代码块保留、内部链接、首页评论路径、缓存过期/冲突阻止构建、所有图片和独立 XIV 页面。生成目录只在 `target/`，不覆盖旧的 `dist/`。

可选的真实 Chromium 验收脚本为 `tools/tests/browser.test.mjs`，额外需要 Chromium 和 `playwright-core`（不属于站点运行依赖）。在静态预览服务 `http://127.0.0.1:8780` 运行时，用 `PLAYWRIGHT_MODULE` 指定已安装模块的绝对 file URL，`CHROMIUM_PATH` 指定浏览器，再执行 `just check-browser`；也可用 `SITE_URL` 指向其他本地端口。结果和桌面/移动端截图写入 `target/notist-verification/`。设置 `CAPTURE_REFERENCE=1` 会额外只读打开线上首页，保存同尺寸参考截图。

此前已验证：桌面首页/列表/正文、搜索命中与空结果、2024/2025 年切换及图片计数、灯箱前后切换与键盘关闭、390px 无横向溢出、评论抽屉展开/背景 inert/关闭；首页旧评论仍可读取。未提交评论、未推送或触发线上发布。旧照片的目录遍历顺序不作等价承诺。

## 导航与部署版的精确对照

初次移植曾引入灰底 hover、2px 标题上移、55px 导航内容高度、移动端收紧间距/禁止换行等偏差，已撤销。导航现在使用部署版的背景分层、OKLCH 颜色、描边 hover、4px 标题位移、56px 内容高度和原始移动端收缩/换行；GitHub 图标使用同一 Font Awesome Free SVG 和 CSS mask（Font Awesome Free，CC BY 4.0）。首页不再显示原版没有启用的阅读进度条。同时撤销了首页移动端头像/字号缩小、额外底部留白和文章列表标题放大的规则。

`just check-nav-parity`（同样需要可选 Chromium / playwright-core 环境）会只读访问部署版，在 1440、1024、768、480、390、360px 下比较普通、品牌悬停、文章按钮悬停：共 18 组，对照导航/背景/进度控件的 14 个节点、31 个计算样式属性及几何位置。结果与成对截图保存在 `target/notist-verification/navigation/`。这是指定状态下的导航对照，不等于已证明全站所有页面、动画中间帧或不同浏览器逐像素一致。

随后 `tools/tests/nav-pixels.py` 比较这 18 对顶部 60px 截图。此次同一 Chromium 中的 18 对截图均为 0 差异像素，结果见该目录的 `pixel-diff.json`。初次实现只验证整体布局与功能，不能据此声称全站完全对应；导航的严格对照是本次补上的单独验证范围。
