const cheerio = require("cheerio");
const count = 3000;
const html = `<ul>${"<li></li>".repeat(count)}</ul>`;
const loopNum = 200;
const selector = ":nth-child(2n),:nth-child(3n),:nth-child(5n)";
const $ = cheerio.load(html, null, false);
let ul = $("ul");
let items = ul.children(selector);
console.log(`Html: <ul>\${"<li></li>".repeat(${count})}</ul>`);
console.log(`Query: ul.children("${selector}")`);
console.log(`Find matched: ${items.length}`);
console.log(`Execute ${loopNum} times to get average time:`);
const startTime = Date.now();
for (let i = 0; i < loopNum; i++) {
  let items = ul.children(selector);
}
const elapsed = Date.now() - startTime;
console.log(`Elapsed: ${elapsed / 1e3}s, Average Time：${elapsed / loopNum}ms`);
