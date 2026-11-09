// Optional read-only comparison against the still-deployed aoike post list.
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {mkdir, writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';

const {chromium} = await import(process.env.PLAYWRIGHT_MODULE || 'playwright-core');
const executablePath = process.env.CHROMIUM_PATH || execFileSync('which', ['chromium'], {encoding: 'utf8'}).trim();
const base = process.env.SITE_URL || 'http://127.0.0.1:8780';
const reference = process.env.REFERENCE_URL || 'https://azurice.github.io/posts';
const output = resolve('target/notist-verification/summaries');
await mkdir(output, {recursive: true});
const browser = await chromium.launch({executablePath});
try {
  const groups = [];
  for (const [name, url] of [['deployed', reference], ['notist', base + '/posts/']]) {
    const page = await browser.newPage({viewport: {width: 1440, height: 1000}});
    await page.goto(url);
    await page.locator('.summary').first().waitFor();
    await page.evaluate(() => document.fonts.ready);
    const summaries = await page.locator('.summary').evaluateAll(nodes => nodes.map(node => ({
      text: node.textContent.replace(/\s+/g, ''),
      height: node.getBoundingClientRect().height,
      codeBlocks: node.querySelectorAll('pre').length,
      h1: node.querySelectorAll('h1').length,
    })));
    groups.push(summaries);
    await page.screenshot({path: `${output}/${name}.png`, fullPage: true});
    await page.close();
  }
  const [expected, actual] = groups;
  await writeFile(`${output}/results.json`, JSON.stringify({expected, actual}, null, 2) + '\n');
  assert.ok(expected.length > 0);
  assert.equal(actual.length, expected.length);
  for (let i = 0; i < actual.length; i++) {
    assert.equal(actual[i].text, expected[i].text, `Excerpt ${i}: text differs`);
    assert.equal(actual[i].codeBlocks, expected[i].codeBlocks, `Excerpt ${i}: code blocks differ`);
    assert.equal(actual[i].h1, 0);
    assert.ok(Math.abs(actual[i].height - expected[i].height) < .1, `Excerpt ${i}: height differs`);
  }
  console.log(`PASS rich excerpts: ${actual.length} article texts, code-block counts and desktop summary heights match deployed aoike`);
} finally {
  await browser.close();
}
