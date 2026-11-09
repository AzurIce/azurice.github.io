// Progressive enhancement only: pages, navigation, descriptions and images are
// already static HTML. No SPA router or client-side Markdown renderer.
import {computeGalleryLayout} from './gallery-layout.mjs';
const $ = (selector, root = document) => root.querySelector(selector);
const $$ = (selector, root = document) => [...root.querySelectorAll(selector)];
const reducedMotion = matchMedia('(prefers-reduced-motion: reduce)').matches;
const progressEnabled = ['article', 'gallery', 'gallery_index'].includes(document.body.dataset.kind);

for (const heading of $$('.markdown :is(h1,h2,h3,h4)[id]')) {
  if (heading.querySelector('a')) continue;
  const anchor = document.createElement('a');
  anchor.className = 'heading-anchor';
  anchor.href = `#${encodeURIComponent(heading.id)}`;
  anchor.append(...heading.childNodes);
  heading.append(anchor);
}

let scrollPending = false;
function updateProgress() {
  scrollPending = false;
  const limit = document.documentElement.scrollHeight - innerHeight;
  const ratio = limit > 0 ? Math.min(1, Math.max(0, scrollY / limit)) : 0;
  $('.scroll-progress-bar').style.transform = `scaleX(${ratio})`;
  $('.scroll-progress').classList.toggle('visible', progressEnabled);
  const pill = $('.scroll-progress-pill');
  pill.textContent = `${Math.round(ratio * 100)}%`;
  pill.style.left = `clamp(1rem, ${ratio * 100}%, calc(100% - 1rem))`;
  pill.classList.toggle('visible', progressEnabled);
  $('.scroll-top').classList.toggle('visible', progressEnabled && ratio > .005);
}
addEventListener('scroll', () => {
  if (!scrollPending) { scrollPending = true; requestAnimationFrame(updateProgress); }
}, {passive: true});
addEventListener('resize', updateProgress);
addEventListener('load', updateProgress);
$('.scroll-top').addEventListener('click', () => scrollTo({top: 0, behavior: reducedMotion ? 'instant' : 'smooth'}));
updateProgress();

// Search only reads text emitted by Tera. User input is never parsed as HTML.
const search = $('#search-input');
if (search) {
  const cards = $$('[data-search-text]');
  const filter = () => {
    const words = search.value.trim().toLocaleLowerCase().split(/\s+/).filter(Boolean);
    let count = 0;
    for (const card of cards) {
      const text = card.dataset.searchText.toLocaleLowerCase();
      card.hidden = !words.every(word => text.includes(word));
      if (!card.hidden) count++;
    }
    $('#search-status').textContent = words.length ? `找到 ${count} 篇文章` : `共 ${count} 篇文章`;
    updateProgress();
  };
  search.value = new URL(location.href).searchParams.get('q') || '';
  search.addEventListener('input', filter);
  filter();
}

const layouts = $$('.gallery-layout').map(element => ({element, cards: $$('.gallery-card', element), width: -1}));
function layoutGallery() {
  for (const layout of layouts) {
    if (layout.element.closest('[hidden]')) continue;
    const width = layout.element.clientWidth;
    const viewport = `${width}:${innerHeight}`;
    if (!width || layout.width === viewport) continue;
    layout.width = viewport;
    const images = layout.cards.map(card => ({width: Number(card.dataset.width), height: Number(card.dataset.height), rating: Number(card.dataset.rating), label: card.dataset.label}));
    const rows = computeGalleryLayout(images, width, innerHeight).map(row => {
      const element = document.createElement('div');
      element.className = `gallery-row${row.feature ? ' gallery-row-feature' : ''}`;
      element.style.gap = width < 640 ? '8px' : '12px';
      for (const item of row.items) {
        const card = layout.cards[item.index];
        card.style.width = `${item.width}px`;
        card.style.height = `${item.height}px`;
        element.append(card);
      }
      return element;
    });
    layout.element.replaceChildren(...rows);
  }
}
const years = $$('.year-tab');
function selectYear(value, updateURL = false) {
  for (const button of years) {
    const active = button.dataset.year === value;
    button.classList.toggle('active', active);
    button.setAttribute('aria-pressed', String(active));
  }
  for (const section of $$('.gallery-date-group')) section.hidden = value !== 'all' && section.dataset.year !== value;
  if (updateURL) {
    const url = new URL(location.href);
    url.searchParams.set('year', value);
    history.replaceState(null, '', url);
  }
  layoutGallery(); updateProgress();
}
for (const button of years) button.addEventListener('click', () => selectYear(button.dataset.year, true));
if (years.length) {
  const requested = new URL(location.href).searchParams.get('year');
  const initial = years.find(button => button.dataset.year === requested) || years.filter(button => /^\d{4}$/.test(button.dataset.year)).at(-1) || years.at(-1);
  selectYear(initial?.dataset.year || 'all');
}
if (layouts.length) {
  const observer = new ResizeObserver(() => { layoutGallery(); updateProgress(); });
  for (const layout of layouts) observer.observe(layout.element);
  addEventListener('resize', layoutGallery);
  layoutGallery();
}

const lightbox = $('#lightbox');
let visibleCards = [], current = 0, lastCard = null;
function showImage(index) {
  current = (index + visibleCards.length) % visibleCards.length;
  const card = visibleCards[current];
  const image = $('.gallery-lightbox-img', lightbox);
  image.src = card.dataset.src;
  image.alt = card.dataset.title;
  $('.gallery-lightbox-title', lightbox).textContent = card.dataset.title;
  $('.gallery-lightbox-desc', lightbox).textContent = card.dataset.description;
  $('.gallery-lightbox-rating', lightbox).textContent = '★'.repeat(Number(card.dataset.rating));
  const label = $('.gallery-lightbox-label', lightbox);
  label.textContent = card.dataset.label;
  label.style.setProperty('--label-color', ({red: '#fb7185', yellow: '#facc15', green: '#4ade80', blue: '#60a5fa', purple: '#c084fc'})[card.dataset.label.toLowerCase()] || '#94a3b8');
  $('time', lightbox).textContent = card.dataset.created.replace('T', ' ');
  $('.gallery-lightbox-counter', lightbox).textContent = `${current + 1} / ${visibleCards.length}`;
  if (lightbox.open) card.scrollIntoView({block: 'nearest', behavior: 'instant'});
}
for (const card of $$('.gallery-card')) card.addEventListener('click', () => {
  if (card.closest('[hidden]')) return;
  visibleCards = $$('.gallery-card').filter(card => !card.closest('[hidden]'));
  lastCard = card;
  showImage(visibleCards.indexOf(card));
  lightbox.showModal();
  document.body.style.overflow = 'hidden';
});
$('.gallery-lightbox-prev', lightbox).addEventListener('click', () => showImage(current - 1));
$('.gallery-lightbox-next', lightbox).addEventListener('click', () => showImage(current + 1));
$('.gallery-lightbox-close', lightbox).addEventListener('click', () => lightbox.close());
lightbox.addEventListener('click', event => { if (event.target === lightbox) lightbox.close(); });
lightbox.addEventListener('keydown', event => {
  if (event.key === 'ArrowLeft') { event.preventDefault(); showImage(current - 1); }
  if (event.key === 'ArrowRight') { event.preventDefault(); showImage(current + 1); }
});
lightbox.addEventListener('close', () => {
  document.body.style.overflow = '';
  (visibleCards[current] || lastCard)?.focus({preventScroll: true});
});
let touchX;
lightbox.addEventListener('touchstart', event => { touchX = event.changedTouches[0].clientX; }, {passive: true});
lightbox.addEventListener('touchend', event => {
  const distance = event.changedTouches[0].clientX - touchX;
  if (Math.abs(distance) > 50) showImage(current + (distance > 0 ? -1 : 1));
}, {passive: true});

const commentBar = $('.comment-overlay-bar');
if (commentBar) {
  const overlay = $('.comment-overlay');
  const panel = $('#comment-panel');
  const backdrop = $('.comment-overlay-backdrop');
  let loading = null;
  async function loadComments() {
    if (loading) return loading;
    $('#comment-error').hidden = true;
    loading = (async () => {
      if (!$('#waline-css')) {
        const link = document.createElement('link');
        link.id = 'waline-css'; link.rel = 'stylesheet';
        link.href = 'https://unpkg.com/@waline/client@3.16.2/dist/waline.css';
        document.head.append(link);
      }
      const {init} = await import('https://unpkg.com/@waline/client@3.16.2/dist/waline.js');
      const host = $('#waline');
      init({el: host, serverURL: host.dataset.server, path: host.dataset.path, lang: 'zh-CN', login: 'enable', pageSize: 10});
    })().catch(() => { loading = null; $('#comment-error').hidden = false; });
    return loading;
  }
  function setComments(open) {
    commentBar.setAttribute('aria-expanded', String(open));
    $('.comment-overlay-chevron').textContent = open ? '↓' : '↑';
    panel.classList.toggle('expanded', open);
    panel.inert = !open;
    backdrop.hidden = !open;
    $('.site-header').inert = open;
    $('.site-main').inert = open;
    document.body.style.overflow = open ? 'hidden' : '';
    if (open) { overlay.setAttribute('role', 'dialog'); overlay.setAttribute('aria-modal', 'true'); loadComments(); }
    else { overlay.removeAttribute('role'); overlay.removeAttribute('aria-modal'); commentBar.focus({preventScroll: true}); }
  }
  commentBar.addEventListener('click', () => setComments(commentBar.getAttribute('aria-expanded') !== 'true'));
  backdrop.addEventListener('click', () => setComments(false));
  $('#retry-comments').addEventListener('click', loadComments);
  overlay.addEventListener('keydown', event => {
    if (event.key === 'Escape') setComments(false);
    if (event.key === 'Tab' && commentBar.getAttribute('aria-expanded') === 'true') {
      const controls = $$('button:not([disabled]), input, textarea, a[href]', overlay).filter(element => element.getClientRects().length);
      const first = controls[0], last = controls.at(-1);
      if (event.shiftKey && document.activeElement === first) { event.preventDefault(); last.focus(); }
      else if (!event.shiftKey && document.activeElement === last) { event.preventDefault(); first.focus(); }
    }
  });
}
