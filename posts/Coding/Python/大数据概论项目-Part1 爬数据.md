---
date: 2023-05-27
---

通过 Python 获取网页数据的方法一般分为两大类：

- 基于 HTML 正则匹配

- 基于 API 请求

前者很简单，其实就是字符串匹配，不过缺点就是较为繁琐复杂；后者也很简单，就是伪装浏览器发请求即可。

下面简单讲一下。



## 零、安装Python

假设你已经安装好 Python。

如何安装可以百度。不过我这里推荐一个工具，叫做 **Scoop**。它是一个 Windows 下的包管理器，可以在终端通过简单的一行命令完成一些软件包的安装、升级、卸载等，而且可以免掉图形化的安装界面，不必下一步下一步的点，此外也用了特殊的方法来管理环境变量，不必再配置为环境变量发愁。具体可以见 [aoike - 告别繁琐安装界面，使用Scoop管理Windows软件 (azurice.github.io)](https://azurice.github.io/posts/告别繁琐安装界面，使用Scoop管理Windows软件.html)。

## 一、基于 HTML 正则匹配

### 1. 有关 HTML

众所周知，每一个网页都是一个 `.html` 文件，一个标准的 HTML 文档的结构大概长这样：

```html
<html>
    <head>
        ...
    </head>
    <body>
        ...
    </body>
</html>
```

其中尖括号扩起来的一个个东西叫做 **标签**，标签成对出现，如 `<sometag></sometag>`，当然如果某些标签中不包含任何内容，也会写作 `<sometag />`。

标签可以携带一些属性，比如 `<script type="text/javascript"></script>`，它有一个值为 `"text/javascript"` 的 `type` 属性。

> 介绍一个重要的网站，上面包含一切 web 技术的文档：[MDN Web Docs (mozilla.org)](https://developer.mozilla.org/zh-CN/)
>
> <img src="大数据概论项目-Part1 爬数据.assets/image-20230526225418855.png" alt="image-20230526225418855" style="zoom:50%;" />

在一般的浏览器中按 F12 选择 **元素** 一栏，便可以看到网页整个的 HTML 代码。

也可以通过在目标元素处右键 -> 检查，来快速定位到其对应的 HTML 代码位置。

<img src="大数据概论项目-Part1 爬数据.assets/image-20230526225745841.png" alt="image-20230526225745841" style="zoom:50%;" />

### 2. 引入

比如对于这个页面：https://space.bilibili.com/46452693

![image-20230526225556972](大数据概论项目-Part1 爬数据.assets/image-20230526225556972.png)

我想爬取他的关注、粉丝、获赞等信息。

通过 F12 我们发现，这部分对应的代码是这样的：

```html
<div class="n-statistics">
    <a href="/46452693/fans/follow" class="n-data n-gz" title="1,596">
        <p class="n-data-k">关注数</p>
        <p id="n-gz" class="n-data-v space-attention">1596</p>
    </a>
    <a href="/46452693/fans/fans" class="n-data n-fs" title="80">
        <p class="n-data-k">粉丝数</p>
        <p id="n-fs" class="n-data-v space-fans">80</p>
    </a>
    <div title="视频、动态、专栏累计获赞319" class="n-data n-bf">
        <p class="n-data-k">获赞数</p>
        <p id="n-bf" class="n-data-v">319</p>
    </div>
    <div title="截止昨天，播放数总计为4,271" class="n-data n-bf">
        <p class="n-data-k">播放数</p>
        <p id="n-bf" class="n-data-v">4271</p>
    </div>
    <div title="截止昨天，阅读数总计为275" class="n-data n-bf">
        <p class="n-data-k">阅读数</p>
        <p id="n-bf" class="n-data-v">275</p>
    </div>
</div>
```

这一部分内容位于一个 `class` 为 `n-statistics` 的 `div` 块中，也就是说只要我们在整篇 html 中找到这一部分，就可以从中分离出我们想要的数据。

但是怎么找？简单的字符串匹配么？

### 3. 正则表达式

正则表达式可以用于描述一组字符串。

比如 `<div class="n-statistics">.*</div>` 即可匹配上面的内容。

再进一步，`<p id="n-.*>(.*)</p>` 即可匹配出五个数据。

详细内容可以再查一查。

可以看看这个：[Python 正则表达式 | 菜鸟教程 (runoob.com)](https://www.runoob.com/python/python-reg-expressions.html)

### 4. 码

简单搓了段码：

```python
import re
import requests

AZURICE = 46452693

def url(id):
    return f'https://space.bilibili.com/{id}'


def get_data(id):
    page = requests.get(url(id)).text
    res = re.findall(r'<div class="n-statistics">.*</div>', page, flags=re.S)

    block = res[0]

    res = re.findall(r'<p id="n-.*>(.*)</p>', block)
    return [int(e) for e in res]


if __name__ == "__main__":
    data = get_data(AZURICE)
    print(data)

```

但是，直接这样运行并不能得到想要的结果，如果将 `page` 打印一下会发现只有如下的内容：

```html
<!DOCTYPE html><html><head><title>验证码_哔哩哔哩</title><meta name="viewport" content="width=device-width,user-scalable=no,initial-scale=1,maximum-scale=1,minimum-scale=1,viewport-fit=cover"><meta name="spm_prefix" content="333.1291"><script type="text/javascript" src="//www.bilibili.com/gentleman/polyfill.js?features=Promise%2CObject.assign%2CString.prototype.includes%2CNumber.isNaN"></script>
    <script>
    window._riskdata_ = {
      'v_voucher': 'voucher_bad3755a-dcf8-4544-91d4-87d4f5b09c07'
    }
    </script>
    <script type="text/javascript" src="//s1.hdslb.com/bfs/seed/log/report/log-reporter.js"></script><link href="//s1.hdslb.com/bfs/static/jinkela/risk-captcha/css/risk-captcha.0.4e3ed2119997a8315e1c9a96a1e93f5569d9fb5a.css" rel="stylesheet"></head><body><div id="biliMainHeader"></div><div id="risk-captcha-app"></div><script src="//s1.hdslb.com/bfs/seed/jinkela/risk-captcha-sdk/CaptchaLoader.js"></script><script type="text/javascript" src="//s1.hdslb.com/bfs/static/jinkela/risk-captcha/1.risk-captcha.4e3ed2119997a8315e1c9a96a1e93f5569d9fb5a.js"></script><script type="text/javascript" src="//s1.hdslb.com/bfs/static/jinkela/risk-captcha/risk-captcha.4e3ed2119997a8315e1c9a96a1e93f5569d9fb5a.js"></script></body></html>
```

首先，大多数网站都有反爬机制，一种常见的反爬机制就是通过请求的 headers 中的 User-Agent 来判断是否是一个真正的浏览器发送的请求。那么绕过这个机制也很简单，我们将一个真正的浏览器的 headers 中的 User-Agent 设置给 python：

```diff
import re
import requests

+ HEADERS = {
+     "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36 Edg/113.0.1774.50"
+ }

AZURICE = 46452693

def url(id):
    return f'https://space.bilibili.com/{id}'


def get_data(id):
-     page = requests.get(url(id)).text
+     page = requests.get(url(id), headers=HEADERS).text
    res = re.findall(r'<div class="n-statistics">.*</div>', page, flags=re.S)

    block = res[0]

    res = re.findall(r'<p id="n-.*>(.*)</p>', block)
    return [int(e) for e in res]


if __name__ == "__main__":
    data = get_data(AZURICE)
    print(data)

```

现在，确实发现获取到的内容发现了改变，但是依旧不是一个完整的网页：

```html
<!DOCTYPE html><html><head><meta name="spm_prefix" content="333.999"><meta charset="UTF-8"><meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1"><meta name="renderer" content="webkit|ie-comp|ie-stand"><meta name="referrer" content="no-referrer-when-downgrade"><meta name="applicable-device" content="pc"><meta http-equiv="Cache-Control" content="no-transform"><meta http-equiv="Cache-Control" content="no-siteapp"><script type="text/javascript" src="//s1.hdslb.com/bfs/seed/jinkela/short/config/biliconfig.js"></script><script type="text/javascript">var ua=window.navigator.userAgent,agents=["Android","iPhone","SymbianOS","Windows Phone","iPod"],pathname=/\d+/.exec(window.location.pathname),getCookie=function(e){return decodeURIComponent(document.cookie.replace(new RegExp("(?:(?:^|.*;)\\s*"+encodeURIComponent(e).replace(/[\-\.\+\*]/g,"\\$&")+"\\s*\\=\\s*([^;]*).*$)|^.*$"),"$1"))||null},DedeUserID=getCookie("DedeUserID"),mid=pathname?+pathname[0]:null===DedeUserID?0:+DedeUserID;if(mid<1)window.location.href="https://passport.bilibili.com/login?gourl=https://space.bilibili.com";else{window._bili_space_mid=mid,window._bili_space_mymid=null===DedeUserID?0:+DedeUserID;for(var prefix=/^\/v/.test(pathname)?"/v":"",i=0;i<agents.length;i++)if(-1<ua.indexOf(agents[i])&&!/\sVR\s/g.test(ua)){window.location.href="https://m.bilibili.com/space/"+mid;break}}</script><script type="text/javascript">function getIEVersion(){var e=99;if("Microsoft Internet Explorer"==navigator.appName){var t=navigator.userAgent;null!=new RegExp("MSIE ([0-9]{1,}[.0-9]{0,})").exec(t)&&(e=parseFloat(RegExp.$1))}return e}getIEVersion()<11&&(window.location.href="https://www.bilibili.com/blackboard/activity-I7btnS22Z.html")</script><link rel="prefetch" as="script" href="//s1.hdslb.com/bfs/static/player/main/video.js?v=2023525"><script type="text/javascript" src="//s1.hdslb.com/bfs/static/jinkela/long/js/sentry/sentry-5.2.1.min.js"></script><script type="text/javascript" src="//s1.hdslb.com/bfs/static/jinkela/long/js/sentry/sentry.vue.js"></script><link rel="stylesheet" href="//at.alicdn.com/t/font_438759_d66lkuno6c9.css"><script id="abtest" type="text/javascript">window.abtest={"in_new_ab":true,"ab_version":{},"ab_split_num":{}}</script></body><link href="//s1.hdslb.com/bfs/static/jinkela/space/css/space.9.89c88a9b06d39e34331a447c5eb1e139e95fd3b2.css" rel="stylesheet"><link href="//s1.hdslb.com/bfs/static/jinkela/space/css/space.8.89c88a9b06d39e34331a447c5eb1e139e95fd3b2.css" rel="stylesheet"><title>Azur冰弦的个人空间-Azur冰弦个人主页-哔哩哔哩视频</title><meta name="keywords" content="Azur冰弦的个人空间,Azur冰弦个人主页"/><meta name="description" content="哔哩哔哩Azur冰弦的个人空间，提供Azur冰弦分享的视频、音频、文章、动态、收藏等内容，关注Azur冰弦账 号，第一时间了解UP注动态。这个人不是很懒于是写了一点话。"/><meta name="referrer" content="no-referrer-when-downgrade"><link rel="apple-touch-icon" href="//i0.hdslb.com/bfs/face/ec1b401b2a4caeff3c0de8536294008431ceaec7.jpg"></head><body><div id="biliMainHeader" token-support="true" disable-sticky style="height:56px"></div><div id="space-app"></div><script type="text/javascript">//日志上报
    window.spaceReport = {}
    window.reportConfig = {
      sample: 1,
      scrollTracker: true,
      msgObjects: 'spaceReport'
    }
    var reportScript = document.createElement('script')
    reportScript.src = '//s1.hdslb.com/bfs/seed/log/report/log-reporter.js'
    document.getElementsByTagName('body')[0].appendChild(reportScript)
    reportScript.onerror = function () {
      console.warn('log-reporter.js加载失败，放弃上报')
      var noop = function () { }
      window.reportObserver = {
        sendPV: noop,
        forceCommit: noop
      }
    }

    // webp支持
    function webSupportCheck() {
      const img = new Image()
      img.onload = function () {
        window.supportWebP = (img.width > 0) && (img.height > 0)
      }
      img.onerror = function () {
        window.supportWebP = false
      }
      img.src = 'data:image/webp;base64,UklGRiIAAABXRUJQVlA4IBYAAAAwAQCdASoBAAEADsD+JaQAA3AAAAAA'
    }
    webSupportCheck()</script><script src="//s1.hdslb.com/bfs/seed/laputa-entry-header/bili-entry-header.umd.js"></script><script>var el=document.getElementById("biliMainHeader"),header=new BiliEntryHeader({config:{headerType:"mini",disableSticky:!0,disableChannelEntry:!1,forceVersion:3,tokenSupport:!0}});header.init(el)</script><script src="//s1.hdslb.com/bfs/static/jinkela/long/js/jquery/jquery1.7.2.min.js"></script><div style="display:none"><a href="https://www.bilibili.com/v/game/match/">赛事库</a> <a href="https://www.bilibili.com/cheese/">课堂</a> <a href="https://www.bilibili.com/festival/2021bnj">2021拜年纪</a></div><script type="text/javascript" src="//s1.hdslb.com/bfs/seed/jinkela/short/auto-append-spmid.js"></script><script type="text/javascript" src="//s1.hdslb.com/bfs/static/jinkela/space/9.space.89c88a9b06d39e34331a447c5eb1e139e95fd3b2.js"></script><script type="text/javascript" src="//s1.hdslb.com/bfs/static/jinkela/space/space.89c88a9b06d39e34331a447c5eb1e139e95fd3b2.js"></script></body></html>
```

这就是另一种反爬机制，一些数据是动态加载或延迟加载的，并不会直接出现在网页上，要想获取完全加载完毕的网页，可能需要借助 **Selenium** 库（这个一会会提到）。

不过这里为了演示，就直接手动将页面的部分内容赋给了 `page`：

```diff
import re
import requests

HEADERS = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36 Edg/113.0.1774.50"
}

AZURICE = 46452693

def url(id):
    return f'https://space.bilibili.com/{id}'


def get_data(id):
-     page = requests.get(url(id), headers=HEADERS).text
+     page = '''
+         一大堆一大堆一大堆东西。。。。。
+     ...
+     asdasdjasdjaksdasldsajd
+     Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.
+     <div class="n-statistics">
+     <a href="/46452693/fans/follow" class="n-data n-gz" title="1,596">
+         <p class="n-data-k">关注数</p>
+         <p id="n-gz" class="n-data-v space-attention">1596</p>
+     </a>
+     <a href="/46452693/fans/fans" class="n-data n-fs" title="80">
+         <p class="n-data-k">粉丝数</p>
+         <p id="n-fs" class="n-data-v space-fans">80</p>
+     </a>
+     <div title="视频、动态、专栏累计获赞319" class="n-data n-bf">
+         <p class="n-data-k">获赞数</p>
+         <p id="n-bf" class="n-data-v">319</p>
+     </div>
+     <div title="截止昨天，播放数总计为4,271" class="n-data n-bf">
+         <p class="n-data-k">播放数</p>
+         <p id="n-bf" class="n-data-v">4271</p>
+     </div>
+     <div title="截止昨天，阅读数总计为275" class="n-data n-bf">
+         <p class="n-data-k">阅读数</p>
+         <p id="n-bf" class="n-data-v">275</p>
+     </div>
+ 
+     '''
    res = re.findall(r'<div class="n-statistics">.*</div>', page, flags=re.S)

    block = res[0]

    res = re.findall(r'<p id="n-.*>(.*)</p>', block)
    return [int(e) for e in res]


if __name__ == "__main__":
    data = get_data(AZURICE)
    print(data)

```

现在就可以得到输出：

```
[1596, 80, 319, 4271, 275]
```

### 5. Selenium

刚才提到有很多反爬机制会使得直接对网页的获取并不能得到我们实际在浏览器看到的网页，这时候就需要借助 Selenium 库。

Selenium 是一个浏览器自动化库，可以通过浏览器对网页各个元素进行访问以及操作。具体可以查一查或啃一啃官方文档。

这里给一个我爬取文泉书局电子书的例子：

https://wqbook.wqxuetang.com/read/pdf?bid=2135236

上面这个网页中就包含我想要爬取的 pdf，这个网站做了很多层反爬，一年前我爬大学物理教材的时候它的反爬还没这么厉害（），下面简单讲一下。



首先，第一层反爬，在网页按 F12 无法调出开发者工具。

这是因为网页代码屏蔽了相关的按键。

解决办法：先打开其他网页，调出开发者工具，再修改地址栏回到这个网页即可。



然后我们可以发现，pdf的内容都被显示在 `<img>` 标签中，而且还是 base64 编码的，这意味着我们直接获取字符串进行解码即可得到图片数据：

<img src="大数据概论项目-Part1 爬数据.assets/image-20230527001646983.png" alt="image-20230527001646983" style="zoom: 67%;" />

但是。。等一下。这并不是真实的页面，这是一张清晰度极低的预览图，而页面真正的图片被纵向切分成了 6 份：

<img src="大数据概论项目-Part1 爬数据.assets/image-20230527001809857.png" alt="image-20230527001809857" style="zoom: 50%;" />

这就意味着，我们需要分辨出这六分的顺序，分别解码对应的图片，再进行拼接才能得到一张完整的页面。

这就是第二层反爬。



第三层反爬，很显然，不用试，这里的数据也是动态加载的，无法直接通过 get 网页地址来获取完整的网页，这就需要我们使用 Selenium 操纵浏览器模拟人的行为一页一页翻页。



第四层，这个输入页码的地方。

<img src="大数据概论项目-Part1 爬数据.assets/image-20230527002104678.png" alt="image-20230527002104678" style="zoom:67%;" />

它只有在被点击后才会显示出要输入页码的元素：

<img src="大数据概论项目-Part1 爬数据.assets/image-20230527002137350.png" alt="image-20230527002137350" style="zoom:67%;" />

而且还被折叠在 `div` 块中，想要将其展开看一看里面的标签长什么样又会由于这个点击使得这个输入页码的地方隐藏。

---

不过，还是被我爬了（

码如下

```python
import base64
import os
import re
from time import sleep
from random import random, randint

import requests
from pyquery import PyQuery as pq
from loguru import logger
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.common.by import By
from tqdm import tqdm

from seleniumwire import webdriver

# import chromedriver_binary  # chrome 76.x

from selenium.webdriver.chrome.options import Options

UA = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_8_2) AppleWebKit/537.17 (KHTML, like Gecko) Chrome/24.0.1309.0 Safari/537.17'  # noqa


def get_chrome_driver(timeout=120, headless=True):
    '''
    start a Selenium Chrome driver

    timeout=120; headless=True
    '''
    chrome_options0 = Options()
    chrome_options0.add_argument(f'user-agent={UA}')
    chrome_options0.add_argument("--headless")

    chrome_options1 = Options()
    chrome_options1.add_argument(f'user-agent={UA}')

    driver_ = ''
    try:
        # driver_ = webdriver.PhantomJS(exe, desired_capabilities=dcap)
        if headless:
            driver_ = webdriver.Chrome(
                service=Service(executable_path='./chromedriver.exe'),
                options=chrome_options0,
            )
        else:
            driver_ = webdriver.Chrome(
                service=Service(executable_path='./chromedriver.exe'),
                options=chrome_options1,
            )
        driver_.set_page_load_timeout(timeout + 2)
    except Exception as exc:
        logger.warning(f"webdriver.Chrome Exception: {exc}")

    return driver_


SAVE_PATH = './算法/'
BID = 2135236
url = 'https://wqbook.wqxuetang.com'
book_url = f'{url}/read/pdf?bid={BID}'
# 大学物理（第三版）上: 3221081
# 大学物理（第三版）下: 3224900
# 算法 2135236


def save_webp_from_site():
    # to rid of the browser, set headless to True
    driver = get_chrome_driver(headless=False)
    assert driver, 'Get chrome driver failed.'

    intercepted_imgs = {}

    def response_interceptor(request, response):
        t = response.headers['Content-Type']
        if t and 'image/webp' in t:
            intercepted_imgs[request.url] = response.body

    driver.response_interceptor = response_interceptor

    driver.get(book_url)

    logger.info("Waiting for login")
    sleep(10)
    logger.info("Starting")
    # class_name = 'page-head-right'  # full screen
    # driver.find_element_by_class_name(class_name).click()

    class_name = 'page-head-tol'
    doc = pq(driver.page_source)
    # Page count
    tol = doc(f'.{class_name}').text()
    total = tol.split('/')
    assert len(total) == 2, ' need to finetune '
    total = total[1].strip()

    try:
        tot_page = int(total)
    except Exception as exc:
        logger.error(exc)
        raise SystemError(' Something is wrong, need fine tune')


    logger.info('Saving webp images...')
    # tot_page = 1
    for page in tqdm(range(1, tot_page + 1)):

        # Goto page
        driver.find_element(By.CLASS_NAME, 'page-head-tol').click()
        driver.find_element(By.CLASS_NAME, 'el-input').find_element(By.TAG_NAME, 'input').send_keys(f'{page}\n')

        sleep_ = 4  # + randint(25, 45) + random()
        logger.info(' Sleeping %.2f s' % sleep_)
        sleep(sleep_)

        imgs = driver.find_element(By.ID, f'pageImgBox{page}').find_elements(By.TAG_NAME, 'img')

        picList = []

        for img in imgs:
            url = img.get_attribute('src')

            # print(len(url))
            if url in intercepted_imgs:
                res = re.match('.*left: (.*)px', img.get_attribute('style'))
                left = res.group(1)

                filename = f'{page:03d}-{left}.webp'
                with open(f'webp/{filename}', "wb") as f:
                    # b64_data = pic.split(';base64,')[1]
                    # data = base64.b64decode(b64_data)
                    f.write(intercepted_imgs[url])
            else:
                logger.error('Not intercepted')
    driver.quit()


from PIL import Image

def convert():
    SRC_PATH = './webp'
    DEST_PATH = './jpg'
    for f in tqdm(os.scandir(SRC_PATH)):
        if f.is_file():
            im = Image.open(f'{SRC_PATH}/{f.name}')
            if im.mode == "RGBA":
                im.load()  # required for png.split()
                background = Image.new("RGB", im.size, (255, 255, 255))
                background.paste(im, mask=im.split()[3])
            save_name = f.name.replace('webp', 'jpg')
            im.save(f'{DEST_PATH}/{save_name}', 'JPEG')



from PIL import Image


def concat():
    SRC_PATH = './jpg'
    DEST_PATH = './jpg-concat'

    path_list = os.listdir(SRC_PATH)

    pre_data = []

    for i in path_list:
        res = re.match(r'.*(...)-(.*).jpg', i)
        pre_data.append((int(res.group(1)), float(res.group(2)), i))

    pre_data.sort()
    # print(pre_data)

    data = []
    for i in range(0, len(pre_data), 6):
        data.append([pre_data[i+j][2] for j in range(6)])

    print(data)

    for page in tqdm(data):
        images = [Image.open(f'{SRC_PATH}/{page[i]}') for i in range(6)]
        # image = image.resize((200, 200))
        # images.append(image)
        h = images[0].height
        w = 0
        for image in images:
            w += image.width

        new_image = Image.new('RGB', (w, h), 'white')

        acc = 0
        for i in range(6):
            new_image.paste(images[i], (acc, 0))
            acc += images[i].width

        # 将最终图像保存到磁盘上
        new_image.save(f'{DEST_PATH}/{page[0][:3]}.jpg')

if __name__ == '__main__':
    # save_webp_from_site()
    # convert()
    concat()

```

### 6. 总结

因此这种方式一般为下策，十分繁琐且复杂。

## 二、基于 API 请求

还是 B 站的那几个数据，既然它是动态加载的那么它一定会向服务端发送网络请求来获取数据，只要我模拟浏览器，向相同的 URL，用相同的参数发送请求，不久也可以得到相同的数据了么。

在网络这一栏中我们可以寻找一下数据的请求：

<img src="大数据概论项目-Part1 爬数据.assets/image-20230527002524494.png" alt="image-20230527002524494" style="zoom:67%;" />

于是我们很快的就找到了（这里也是有一些技巧，比如一般是属于 Fetch/XHR 类型的，选上它可以排除掉大部分请求）：

<img src="大数据概论项目-Part1 爬数据.assets/image-20230527002654812.png" alt="image-20230527002654812" style="zoom:67%;" />

可以发现是这样的一个请求：

<img src="大数据概论项目-Part1 爬数据.assets/image-20230527002802911.png" alt="image-20230527002802911" style="zoom:67%;" />

于是事情变得简单了起来：

```python
import requests
import json

headers = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36 Edg/113.0.1774.50",
}

res = requests.get('https://api.bilibili.com/x/relation/stat?vmid=46452693', headers=headers)

json_data = res.content
data = json.loads(json_data)
print(data)

```

得到：

```
{'code': 0, 'message': '0', 'ttl': 1, 'data': {'mid': 46452693, 'following': 1596, 'whisper': 0, 'black': 0, 'follower': 80}}
```

但是这里只有关注数和粉丝数，这是因为其他在另一个接口中：

<img src="大数据概论项目-Part1 爬数据.assets/image-20230527003617804.png" alt="image-20230527003617804" style="zoom:67%;" />

但是如果我们直接请求：

```diff
import requests
import json

headers = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36 Edg/113.0.1774.50",
}

- res = requests.get('https://api.bilibili.com/x/relation/stat?vmid=46452693', headers=headers)
+ res = requests.get('https://api.bilibili.com/x/space/upstat?mid=46452693', headers=headers)

json_data = res.content
data = json.loads(json_data)
print(data)

```

得到的会是空数据：

```
{'code': 0, 'message': '0', 'ttl': 1, 'data': {}}
```

这是没有登陆导致的，很多接口会设计为对登录与否返回不同的数据，或者只有登录才能访问。

可以通过在浏览器登陆后将浏览器的 cookie 设置给 python 来做到伪装登录：

<img src="大数据概论项目-Part1 爬数据.assets/image-20230527003353978.png" alt="image-20230527003353978" style="zoom:67%;" />

```diff
import requests
import json

headers = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36 Edg/113.0.1774.50",
+     "cookie": "buvid3=DCD7DCA7-946A-873B-F086-BBD113D7B55B71803infoc; b_nut=1684650871; i-wanna-go-back=-1; _uuid=564E41097-B827-95A6-B426-AAB746EAEF6470634infoc; FEED_LIVE_VERSION=V8; nostalgia_conf=-1; buvid4=41861768-A85B-36DA-8BA5-DF158A877ED373582-023010820-aN5fltImCgRQWCnsP2i7D%2FJIwXV6ACOcIEqJxVQq467iPa2cgehVRg%3D%3D; CURRENT_FNVAL=4048; rpdid=|(k|~u|k)mkY0J'uY)RYl|)mk; fingerprint=5677c4085fd61a28872087c5575d0f58; buvid_fp_plain=undefined; b_ut=5; header_theme_version=CLOSE; bp_video_offset_46452693=800047030483288200; PVID=2; SESSDATA=4b7f4832%2C1700670810%2C3c842%2A52; bili_jct=62d33772c95b5dd980daf902d3e9bd48; DedeUserID=46452693; DedeUserID__ckMd5=254848859dbd9bdd; buvid_fp=1645a9fad4c3183b3f729cb6147fd8bf; sid=ef1mejq7; home_feed_column=4; browser_resolution=893-989; b_lsid=1033CB7410_18858E871D9"
}

res = requests.get('https://api.bilibili.com/x/space/upstat?mid=46452693', headers=headers)

json_data = res.content
data = json.loads(json_data)
print(data)

```

现在就好了：

```
{'code': 0, 'message': '0', 'ttl': 1, 'data': {'archive': {'view': 4271}, 'article': {'view': 275}, 'likes': 319}}
```

## 三、总结

大概就是这两大类方法，写得比较简陋，可以简单看看。

如果有问题可以随时讨论。