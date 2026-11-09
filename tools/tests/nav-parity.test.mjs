// Read-only deployed-vs-local comparison. Like browser.test.mjs, this optional
// check needs playwright-core and Chromium; it never submits a comment.
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {mkdir, writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';

const {chromium} = await import(process.env.PLAYWRIGHT_MODULE || 'playwright-core');
const executablePath = process.env.CHROMIUM_PATH || execFileSync('which', ['chromium'], {encoding: 'utf8'}).trim();
const base = process.env.SITE_URL || 'http://127.0.0.1:8780';
const referenceURL = process.env.REFERENCE_URL || 'https://azurice.github.io/';
const output = resolve('target/notist-verification/navigation');
await mkdir(output, {recursive: true});
const properties = ['display', 'position', 'height', 'width', 'paddingTop', 'paddingBottom', 'paddingLeft', 'paddingRight',
  'gap', 'fontSize', 'lineHeight', 'whiteSpace', 'borderRadius', 'outlineWidth', 'outlineColor', 'outlineStyle',
  'backgroundColor', 'transitionProperty', 'transitionDuration', 'transitionTimingFunction', 'transform', 'translate',
  'maxHeight', 'opacity', 'borderBottomWidth', 'borderBottomColor', 'color', 'fontFamily', 'justifyContent', 'alignItems', 'backdropFilter'];

async function capture(page, live) {
  return page.evaluate(({live, properties}) => {
    const header = document.querySelector('body > header');
    const nav = header.querySelector('nav');
    const brand = nav.children[0];
    const nodes = {header, nav, brand, avatar: brand.querySelector('img'), text: brand.children[1],
      title: brand.children[1].children[0], description: brand.children[1].children[1],
      article: nav.children[1], gallery: nav.children[2], search: nav.children[3], repository: nav.lastElementChild,
      backdrop: live ? header.querySelector('div.absolute') : header.querySelector('.site-header-backdrop'),
      progress: header.querySelector('.scroll-progress'), pill: header.querySelector('.scroll-progress-pill')};
    return Object.fromEntries(Object.entries(nodes).map(([name, node]) => {
      const style = getComputedStyle(node), rect = node.getBoundingClientRect();
      return [name, {rect: [rect.x, rect.y, rect.width, rect.height], ...Object.fromEntries(properties.map(property => [property, style[property]]))}];
    }));
  }, {live, properties});
}

const browser = await chromium.launch({executablePath});
const cases = [], differences = [];
try {
  for (const width of [1440, 1024, 768, 480, 390, 360]) {
    const live = await browser.newPage({viewport: {width, height: 900}});
    const local = await browser.newPage({viewport: {width, height: 900}});
    await Promise.all([live.goto(referenceURL), local.goto(base)]);
    for (const page of [live, local]) {
      await page.locator('body > header nav img').waitFor({state: 'visible'});
      await page.locator('body > header nav img').evaluate(image => image.decode());
      await page.evaluate(() => document.fonts.ready);
    }
    for (const state of ['normal', 'brand-hover', 'article-hover']) {
      if (state !== 'normal') {
        const selector = `body > header nav > a:nth-of-type(${state === 'brand-hover' ? 1 : 2})`;
        await Promise.all([live.locator(selector).hover(), local.locator(selector).hover()]);
        // Measure the settled 500ms description/title and 300ms outline states.
        await local.waitForTimeout(650);
      }
      const expected = await capture(live, true), actual = await capture(local, false);
      for (const node of Object.keys(expected)) {
        for (const property of Object.keys(expected[node])) {
          const a = actual[node][property], b = expected[node][property];
          const equal = property === 'rect' ? a.every((value, index) => Math.abs(value - b[index]) < .05) : a === b;
          if (!equal) differences.push({width, state, node, property, expected: b, actual: a});
        }
      }
      const clip = {x: 0, y: 0, width, height: 60};
      await live.screenshot({path: `${output}/${width}-${state}-deployed.png`, clip});
      await local.screenshot({path: `${output}/${width}-${state}-notist.png`, clip});
      cases.push({width, state, nodes: Object.keys(expected).length, properties: properties.length, passed: !differences.some(diff => diff.width === width && diff.state === state)});
    }
    await live.close(); await local.close();
  }
  await writeFile(`${output}/results.json`, JSON.stringify({referenceURL, cases, differences}, null, 2) + '\n');
  assert.deepEqual(differences, [], 'Navigation differs from the deployed reference; see navigation/results.json');
  console.log(`PASS navigation parity: ${cases.length} viewport/state cases, 14 nodes and ${properties.length} style properties plus geometry each`);
} finally {
  await browser.close();
}
