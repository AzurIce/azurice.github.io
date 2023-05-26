'''
fetch_book.py

pip install tqdm loguru pyquery selenium
# 下载安装对应的 chromedriver https://chromedriver.chromium.org/downloads

python fetch_book.py

爬书
Python+TensorFlow机器学习实战
https://lib-nuanxin.wqxuetang.com/read/pdf/3208943

为避免拉黑IP，不时设了较长sleep时间…… 爬完约需6小时——装做人类在读这本书，6小时看完
'''
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
