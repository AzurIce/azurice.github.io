// Check rendered scopes AND computed colors: spans alone can still look plain.
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';

const {chromium} = await import(process.env.PLAYWRIGHT_MODULE || 'playwright-core');
const executablePath = process.env.CHROMIUM_PATH || execFileSync('which', ['chromium'], {encoding: 'utf8'}).trim();
const base = process.env.SITE_URL || 'http://127.0.0.1:8780';
const browser = await chromium.launch({executablePath});
try {
  const page = await browser.newPage();
  for (const [path, container] of [
    ['/posts/', '.summary'],
    ['/search/', '.summary'],
    ['/posts/linux-kernel-module.html', 'main'],
  ]) {
    await page.goto(base + path);
    const shell = `${container} pre > code .z-source.z-shell`;
    const command = page.locator(`${shell} .z-variable.z-function`).filter({hasText: 'root@ecs-3c5a'}).first();
    const tilde = page.locator(`${shell} .z-variable.z-language.z-tilde`).first();
    await command.waitFor();
    assert.equal(await command.evaluate(node => getComputedStyle(node).color), 'rgb(75, 105, 198)', `${path}: command color`);
    assert.equal(await tilde.evaluate(node => getComputedStyle(node).color), 'rgb(215, 57, 72)', `${path}: tilde color`);
    const art = page.locator(`${shell} .z-variable.z-function`).filter({hasText: '.-/+oossssoo+/-.'}).first();
    assert.equal(await art.evaluate(node => getComputedStyle(node).color), 'rgb(75, 105, 198)', `${path}: ASCII art color`);
  }
  console.log('PASS shell highlight scopes and computed colors in list, search and article');
} finally {
  await browser.close();
}
