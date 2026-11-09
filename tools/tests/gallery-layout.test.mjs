import {test} from 'node:test';
import assert from 'node:assert/strict';
import {computeGalleryLayout} from '../../site/static/gallery-layout.mjs';

const image = (width, height, rating = 3, label = '') => ({width, height, rating, label});

test('empty and zero-width layouts are empty', () => {
  assert.deepEqual(computeGalleryLayout([], 900, 900), []);
  assert.deepEqual(computeGalleryLayout([image(100, 100)], 0, 900), []);
});
test('red images move to the feature region and pair on desktop', () => {
  const images = [image(1600, 900), image(1600, 900, 5, 'Red'), image(1600, 900, 5, 'red')];
  const desktop = computeGalleryLayout(images, 1200, 900);
  assert.equal(desktop[0].feature, true);
  assert.deepEqual(desktop[0].items.map(item => item.index), [1, 2]);
  const mobile = computeGalleryLayout(images, 350, 800);
  assert.equal(mobile.filter(row => row.feature).length, 2);
});
test('mixed landscape and portrait layouts stay within width and cover every image', () => {
  const images = Array.from({length: 47}, (_, index) => index % 3 ? image(1600, 900, index % 6) : image(600, 900, 5));
  for (const width of [240, 350, 640, 960, 1440]) {
    const rows = computeGalleryLayout(images, width, 900);
    const indices = rows.flatMap(row => row.items.map(item => item.index));
    assert.deepEqual(indices, images.map((_, index) => index));
    for (const row of rows) {
      const used = row.items.reduce((sum, item) => sum + item.width, 0) + (row.items.length - 1) * (width < 640 ? 8 : 12);
      assert.ok(used <= width + .5);
      assert.ok(row.items.every(item => item.width > 0 && item.height > 0));
    }
  }
});
test('single portraits receive more height than single landscapes', () => {
  const portrait = computeGalleryLayout([image(600, 900)], 960, 900)[0].items[0];
  const landscape = computeGalleryLayout([image(1600, 900)], 960, 900)[0].items[0];
  assert.ok(portrait.height > landscape.height);
});
test('feature heights respect viewport caps', () => {
  const item = computeGalleryLayout([image(1600, 900, 5, 'Red')], 1400, 900)[0].items[0];
  assert.ok(item.height <= 900 * .62);
});
