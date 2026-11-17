// Reduce a saved results page to the markup the ad rules read: no scripts, styles,
// images or handler attributes, and ad links cut down to the parameters search-ads.js uses.
// Save a page with a real browser (for example `google-chrome --headless=new --dump-dom URL > page.html`), then:
//   node scripts/trim-serp.js page.html tests/fixtures/serp/NAME.html 'URL'
const { JSDOM, VirtualConsole } = require('jsdom');
const fs = require('fs');
const [src, out, base] = process.argv.slice(2);
const html = fs.readFileSync(src, 'utf8').replace(/<style[\s\S]*?<\/style>/g, '').replace(/<script[\s\S]*?<\/script>/g, '');
const dom = new JSDOM(html, { virtualConsole: new VirtualConsole(), url: base });
const d = dom.window.document;
d.querySelectorAll('script,style,svg,img,link,meta,noscript,iframe,template,textarea,input,g-img,canvas,video,picture,source,path').forEach(n => n.remove());
const keep = /^(id|class|href|role|aria-level|aria-label|data-(text-ad|pcu|dtld|rw|layout|testid|bm|hveid|partnertag))$/;
const PARAMS = new Set(['q', 'ad_domain', 'ad_provider', 'u', 'adurl', 'sa']);
const short = v => { try { const u = new URL(v, base); const q = [...u.searchParams].filter(([k]) => PARAMS.has(k)); u.search = new URLSearchParams(q).toString(); u.hash = ''; return u.href; } catch { return v; } };
for (const el of d.querySelectorAll('*')) for (const a of [...el.attributes]) {
    if (!keep.test(a.name)) el.removeAttribute(a.name);
    else if (a.name === 'href' || a.name === 'data-rw') el.setAttribute(a.name, short(a.value));
    else if (a.name === 'class') { const c = a.value.split(/\s+/).filter(x => /^(b_|sb_|ads|pa_|react-results)/.test(x)).join(' '); if (c) el.setAttribute('class', c); else el.removeAttribute('class'); }
}
// Drop empty leaves repeatedly and comments.
const w = d.createTreeWalker(d.body, 128); const cs = []; while (w.nextNode()) cs.push(w.currentNode); cs.forEach(c => c.remove());
for (let i = 0; i < 6; i++) for (const el of [...d.body.querySelectorAll('div,span')]) if (!el.children.length && !el.textContent.trim() && !el.id && !el.attributes.length) el.remove();
fs.writeFileSync(out, '<!doctype html>\n<!-- Saved ' + new Date().toISOString().slice(0, 10) + ' from ' + base + ', reduced by scripts/trim-serp.js: scripts, styles, images and most attributes removed. -->\n' + d.body.outerHTML.replace(/\n\s*\n+/g, '\n') + '\n');
console.log(out, fs.statSync(out).size);
