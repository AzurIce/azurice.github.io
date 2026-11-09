// Optional real-Chromium checks; requires playwright-core and a Chromium binary.
// No comment submission, database writes, or logged-in browser profile is used.
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {mkdir, writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';

const {chromium} = await import(process.env.PLAYWRIGHT_MODULE || 'playwright-core');
const executablePath = process.env.CHROMIUM_PATH || execFileSync('which', ['chromium'], {encoding: 'utf8'}).trim();
const base = process.env.SITE_URL || 'http://127.0.0.1:8780';
const output = resolve('target/notist-verification');
await mkdir(output, {recursive: true});
const browser = await chromium.launch({executablePath, headless: true});
const results = [], errors = [];
const record = name => { results.push(name); console.log(`PASS ${name}`); };
try {
  const page = await browser.newPage({viewport: {width: 1440, height: 900}, reducedMotion: 'reduce'});
  page.on('pageerror', error => errors.push(error.message));
  const go = async path => {
    await page.goto(base + path, {waitUntil: 'domcontentloaded'});
    await page.waitForFunction(() => document.querySelector('.site-header') && document.querySelector('.scroll-progress-bar')?.style.transform);
  };
  await go('/');
  assert.equal(await page.locator('.hero-avatar').getAttribute('src'), '/avatar.jpg');
  assert.equal(await page.locator('.recent-posts li').count(), 4);
  assert.equal(await page.locator('.site-header').evaluate(node => node.getBoundingClientRect().height), 56);
  await page.screenshot({path: `${output}/home-desktop.png`});
  record('desktop home, navigation, four recent posts, 56px header');

  await page.locator('.comment-overlay-bar').click();
  await page.waitForFunction(() => document.querySelector('#comment-panel').getBoundingClientRect().height > 100);
  assert.equal(await page.locator('.site-main').evaluate(node => node.inert), true);
  await page.locator('#waline .wl-panel').waitFor({state: 'visible', timeout: 30000});
  assert.equal(await page.locator('#waline').getAttribute('data-path'), '/');
  await page.keyboard.press('Escape');
  assert.equal(await page.locator('#comment-panel').evaluate(node => node.inert), true);
  assert.equal(await page.locator('.site-main').evaluate(node => node.inert), false);
  record('Waline drawer mounts at /, expands, traps background and closes');

  await go('/posts/');
  assert.equal(await page.locator('.post-card').count(), 4);
  await page.screenshot({path: `${output}/posts-desktop.png`});
  await go('/posts/linux-kernel-module.html');
  assert.equal(await page.locator('.article-content h1').count(), 1);
  assert.ok(await page.locator('.article-content pre').count() > 10);
  await page.screenshot({path: `${output}/article-desktop.png`});
  record('post list and converted Typst article, no duplicate title');

  await go('/search/');
  await page.locator('#search-input').fill('launchd');
  assert.equal(await page.locator('[data-search-text]:visible').count(), 1);
  await page.locator('#search-input').fill('<img src=x onerror=alert(1)>');
  assert.equal(await page.locator('[data-search-text]:visible').count(), 0);
  record('search filters text safely, including empty results');

  const earth = '/gallery/' + encodeURIComponent('地球 OL') + '/';
  await go(earth);
  await page.waitForFunction(() => document.querySelector('.year-tab.active')?.dataset.year === '2025');
  assert.equal(await page.locator('.gallery-card:visible').count(), 47);
  await page.locator('.gallery-card:visible img').first().evaluate(image => image.decode());
  await page.screenshot({path: `${output}/gallery-desktop.png`});
  await page.locator('[data-year="2024"].year-tab').click();
  assert.equal(await page.locator('.gallery-card:visible').count(), 56);
  assert.ok(page.url().endsWith('?year=2024'));
  const first = page.locator('.gallery-card:visible').first();
  const firstTitle = await first.getAttribute('data-title');
  await first.click();
  assert.equal(await page.locator('#lightbox').evaluate(node => node.open), true);
  await page.locator('.gallery-lightbox-img').evaluate(image => image.decode());
  await page.locator('.gallery-lightbox-next').click();
  assert.notEqual(await page.locator('.gallery-lightbox-title').textContent(), firstTitle);
  await page.keyboard.press('ArrowLeft');
  assert.equal(await page.locator('.gallery-lightbox-title').textContent(), firstTitle);
  await page.keyboard.press('Escape');
  assert.equal(await page.locator('#lightbox').evaluate(node => node.open), false);
  record('gallery year counts, shareable year URL, lightbox next/keyboard/close');

  await page.setViewportSize({width: 390, height: 844});
  await go('/');
  assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth));
  await page.screenshot({path: `${output}/home-mobile.png`, fullPage: true});
  await go(earth);
  assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth));
  const featured = page.locator('.gallery-card-feature:visible');
  await featured.first().click();
  await page.locator('.gallery-lightbox-img').evaluate(image => image.decode());
  await page.screenshot({path: `${output}/lightbox-mobile.png`});
  await page.keyboard.press('Escape');
  record('390px mobile home and gallery, no horizontal overflow, modal preview');
  assert.deepEqual(errors, []);
  record('no browser page exceptions');

  if (process.env.CAPTURE_REFERENCE === '1') {
    const reference = await browser.newPage({viewport: {width: 1440, height: 900}});
    await reference.goto('https://azurice.github.io/', {waitUntil: 'domcontentloaded'});
    await reference.locator('main img').first().waitFor({state: 'visible'});
    await reference.locator('main img').first().evaluate(image => image.decode());
    await reference.screenshot({path: `${output}/reference-home-desktop.png`});
    await reference.setViewportSize({width: 390, height: 844});
    await reference.screenshot({path: `${output}/reference-home-mobile.png`, fullPage: true});
    record('read-only deployed home reference captured');
  }
  await writeFile(`${output}/results.json`, JSON.stringify({results, errors}, null, 2) + '\n');
} finally {
  await browser.close();
}
