// Port of aoike-sycamore's weighted editorial layout. Pure numeric functions
// keep the deployed row sizing/portrait treatment testable without a DOM.
const clamp = (value, min, max) => Math.max(min, Math.min(max, value));
const aspect = image => image.width > 0 && image.height > 0 ? clamp(image.width / image.height, .25, 4) : 4 / 3;
const rating = image => clamp(image.rating || 3, 3, 5);
const pressure = image => clamp((.9 - aspect(image)) / .5, 0, 1);
function layoutAspect(image) {
  const value = aspect(image);
  if (value < .85) return Math.min(value + (.85 - value) * .32, value / .86);
  if (value > 1.9) return Math.max(value - (value - 1.9) * .28, value * .88);
  return value;
}
function rowPressure(images) {
  if (!images.length) return 0;
  const values = images.map(pressure);
  const attenuation = images.length <= 2 ? 1 : Math.max(.64, 1 - .12 * (images.length - 2));
  return (Math.max(...values) * .75 + values.reduce((a, b) => a + b, 0) / values.length * .25) * attenuation;
}
function limits(width, height) {
  return {
    base: width < 640 ? clamp(width / 2.35, 145, 175) : clamp(width / 5.2, 185, 260),
    feature: width < 640 ? Math.min(height * .48, 420) : width < 1000 ? Math.min(height * .56, 520) : Math.min(height * .62, 620),
    single: width < 640 ? Math.min(height * .46, 360) : width < 1000 ? Math.min(height * .5, 420) : Math.min(height * .54, 480),
  };
}
function rowTarget(images, base, contrast) {
  if (!contrast || !images.length) return base;
  const sum = images.reduce((sum, image) => sum + aspect(image), 0);
  const priority = images.reduce((sum, image) => sum + (rating(image) - 4) * aspect(image), 0);
  return base * (1 + clamp(priority / sum, -1, 1) * .08);
}
function standardCandidate(images, indices, width, gap, limit, contrast, last, segmentLength) {
  const count = images.length;
  const available = width - gap * (count - 1);
  const sum = images.reduce((sum, image) => sum + layoutAspect(image), 0);
  if (!count || available <= 0 || sum <= 0) return null;
  const p = rowPressure(images);
  const target = rowTarget(images, limit.base, contrast) * (count === 1 ? 1 : 1 + p * .45);
  const maxHeight = (width < 640 ? 220 : Math.min(limit.base * 1.32, 330)) * (1 + p * .55);
  const ideal = available / sum;
  const normalMin = width < 640 ? 92 : limit.base * .68;
  const floor = target * (.9 + p * .12);
  const minHeight = normalMin + (Math.max(floor, normalMin) - normalMin) * clamp(p / .35, 0, 1);
  if (count > 1 && ideal < minHeight) return null;
  let height;
  if (count === 1) {
    const portrait = sum < .85;
    const scale = (sum < 1.15 ? 1.45 : 1.3) + p * .7;
    const fraction = width < 640 ? (portrait ? .72 : 1) : portrait ? .46 : .84;
    height = Math.min(target * scale, limit.single * (1 + p * .15), width * fraction / sum);
  } else if (last && ideal > target * 1.08) height = Math.min(target, maxHeight);
  else height = Math.min(ideal, maxHeight);
  const items = indices.map((index, n) => ({index, width: layoutAspect(images[n]) * height, height}));
  const used = items.reduce((sum, item) => sum + item.width, 0) + gap * (count - 1);
  if (used > width + .5) return null;
  const error = (height - target) / limit.base;
  const slack = Math.max(0, (width - used) / width);
  let cost = error ** 2 * 4 + slack ** 2 * (last ? .45 : 2.5);
  if (count === 1 && segmentLength > 1) cost += 7;
  for (let n = 0; contrast && n < count; n++) {
    const share = items[n].width / width;
    if (rating(images[n]) === 5) {
      if (share < .18) cost += (.18 - share) ** 2 * 16;
      if (count > 4) cost += .18 * (count - 4);
    } else if (rating(images[n]) === 3 && share > .48) cost += (share - .48) ** 2 * 12;
  }
  return {items, cost, feature: false};
}
function featureCandidate(images, indices, width, gap, limit) {
  const available = width - gap * (images.length - 1);
  const sum = images.reduce((sum, image) => sum + layoutAspect(image), 0);
  const max = limit.feature * (1 + rowPressure(images) * .18);
  const height = Math.min(available / sum, max);
  const items = indices.map((index, n) => ({index, width: layoutAspect(images[n]) * height, height}));
  const used = items.reduce((sum, item) => sum + item.width, 0) + gap * (items.length - 1);
  const slack = Math.max(0, (width - used) / width);
  return {items, feature: true, cost: slack ** 2 * 2 + ((height - max) / Math.max(1, max)) ** 2 * .35};
}
function segment(all, indices, width, height, feature) {
  if (!indices.length) return [];
  const gap = width < 640 ? 8 : 12;
  const limit = limits(width, height);
  const contrast = new Set(indices.map(index => rating(all[index]))).size > 1;
  const maxItems = feature ? (width < 640 ? 1 : 2) : (width < 640 ? 4 : 6);
  const costs = new Array(indices.length + 1).fill(Infinity);
  const previous = new Array(indices.length + 1);
  costs[0] = 0;
  for (let start = 0; start < indices.length; start++) {
    if (!Number.isFinite(costs[start])) continue;
    for (let end = start + 1; end <= Math.min(start + maxItems, indices.length); end++) {
      const slice = indices.slice(start, end);
      const images = slice.map(index => all[index]);
      const row = feature ? featureCandidate(images, slice, width, gap, limit) : standardCandidate(images, slice, width, gap, limit, contrast, end === indices.length, indices.length);
      if (!row) continue;
      const cost = row.cost + (feature && end - start === 1 && indices.length > 1 ? .08 : 0);
      if (costs[start] + cost < costs[end]) {
        costs[end] = costs[start] + cost;
        previous[end] = {start, row};
      }
    }
  }
  const rows = [];
  for (let cursor = indices.length; cursor > 0;) {
    const step = previous[cursor];
    if (!step) throw new Error('Gallery layout has no single-image fallback');
    rows.push(step.row);
    cursor = step.start;
  }
  return rows.reverse();
}
export function computeGalleryLayout(images, width, height) {
  if (!images.length || width <= 0) return [];
  const featured = [], standard = [];
  images.forEach((image, index) => (String(image.label || '').toLowerCase() === 'red' ? featured : standard).push(index));
  return [...segment(images, featured, width, height, true), ...segment(images, standard, width, height, false)];
}
