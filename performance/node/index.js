const cheerio = require("cheerio");
const fs = require("fs");
const NODECOUNT = 3000;
const LOOPTIMES = 200;


function getFileContent(file) {
  return fs.readFileSync(file, 'utf8').toString()
}

function loadHtml() {
  const content = getFileContent("../data/index.html");
  return {
    selector: '',
    usedTime: execTimesAvg(function () {
      cheerio.load(content, null, false)
    })
  };
}

function execTimesAvg(cb) {
  const startTime = Date.now();
  for (let i = 0; i < LOOPTIMES; i++) {
    cb();
  }
  const elapsed = Date.now() - startTime;
  return elapsed / LOOPTIMES;
}

function findId() {
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}<li id='target'></li></ul>`;
  const selector = "#target";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.find(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      ul.find(selector);
    })
  };
}

function findClass() {
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}<li class='target'></li></ul>`;
  const selector = ".target";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.find(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      ul.find(selector);
    })
  };
}

function findAttr() {
  const html = `<dl>${"<dt></dt><dd contenteditable></dd>".repeat(NODECOUNT / 2)}</dl>`;
  const selector = "[contenteditable]";
  const $ = cheerio.load(html, null, false);
  let dl = $("dl");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dl.find(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      dl.find(selector);
    })
  };
}

function findName() {
  const html = `<dl>${"<dt></dt><dd></dd>".repeat(NODECOUNT / 2)}</dl>`;
  const selector = "dt";
  const $ = cheerio.load(html, null, false);
  let dl = $("dl");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dl.find(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      dl.find(selector);
    })
  };
}

function findPrev() {
  const html = `<dl>${"<dt></dt><dd></dd>".repeat(NODECOUNT / 2)}</dl>`;
  const selector = "dd";
  const $ = cheerio.load(html, null, false);
  let dt = $("dl dt");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dt.prev(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      dt.prev(selector);
    })
  };
}

function findPrevAll() {
  const html = `<dl>${"<dt></dt><dd></dd>".repeat(NODECOUNT / 2)}</dl>`;
  const selector = "dd";
  const $ = cheerio.load(html, null, false);
  let dt = $("dl dt");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dt.prevAll(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      dt.prevAll(selector);
    })
  };
}

function findNext() {
  const html = `<dl>${"<dt></dt><dd></dd>".repeat(NODECOUNT / 2)}</dl>`;
  const selector = "dd";
  const $ = cheerio.load(html, null, false);
  let dt = $("dl dt");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dt.next(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      dt.next(selector);
    })
  };
}

function findNextAll() {
  const html = `<dl>${"<dt></dt><dd></dd>".repeat(NODECOUNT / 2)}</dl>`;
  const selector = "dd";
  const $ = cheerio.load(html, null, false);
  let dt = $("dl dt");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dt.nextAll(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      dt.nextAll(selector);
    })
  };
}

function empty() {
  const html = `<ul>${"<li></li><li>a</li>".repeat(NODECOUNT/2)}</ul>`;
  const selector = ":empty";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      ul.children(selector);
    })
  };
}

function contains() {
  const html = `<ul>${"<li></li><li>a</li>".repeat(NODECOUNT/2)}</ul>`;
  const selector = ":contains('a')";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      ul.children(selector);
    })
  };
}

function firstChild() {
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}</ul>`;
  const selector = ":first-child";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      ul.children(selector);
    })
  };
}

function lastChild() {
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}</ul>`;
  const selector = ":last-child";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      ul.children(selector);
    })
  };
}

function firstOfType() {
  const html = `<dl>${"<dt></dt><dd></dd>".repeat(NODECOUNT / 2)}</dl>`;
  const selector = ":first-of-type";
  const $ = cheerio.load(html, null, false);
  let dl = $("dl");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dl.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      dl.children(selector);
    })
  };
}

function lastOfType() {
  const html = `<dl>${"<dt></dt><dd></dd>".repeat(NODECOUNT / 2)}</dl>`;
  const selector = ":last-of-type";
  const $ = cheerio.load(html, null, false);
  let dl = $("dl");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dl.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      dl.children(selector);
    })
  };
}

function nthChild() {
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}</ul>`;
  const selector = ":nth-child(10)";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      ul.children(selector);
    })
  };
}

function nthChild10() {
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}</ul>`;
  const selector = ":nth-child(10)";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      ul.children(selector);
    })
  };
}

function nthChild2n5() {
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}</ul>`;
  const selector = ":nth-child(2n + 5)";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      ul.children(selector);
    })
  };
}


function nthLastChild() {
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}</ul>`;
  const selector = ":nth-last-child(2n),:nth-last-child(3n),:nth-last-child(5n)";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      ul.children(selector);
    })
  };
}

function nthLastChild10() {
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}</ul>`;
  const selector = ":nth-last-child(10)";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      ul.children(selector);
    })
  };
}

function nthLastChild2n5() {
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}</ul>`;
  const selector = ":nth-last-child(2n + 5)";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      ul.children(selector);
    })
  };
}

function nthOfType() {
  const html = `<dl>${"<dt></dt><dd></dd>".repeat(NODECOUNT / 2)}</dl>`;
  const selector = ":nth-of-type(2n),:nth-of-type(3n)";
  const $ = cheerio.load(html, null, false);
  let dl = $("dl");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dl.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      dl.children(selector);
    })
  };
}

function nthOfType10() {
  const html = `<dl>${"<dt></dt><dd></dd>".repeat(NODECOUNT / 2)}</dl>`;
  const selector = ":nth-of-type(10)";
  const $ = cheerio.load(html, null, false);
  let dl = $("dl");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dl.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      dl.children(selector);
    })
  };
}

function nthOfType2n5() {
  const html = `<dl>${"<dt></dt><dd></dd>".repeat(NODECOUNT / 2)}</dl>`;
  const selector = ":nth-of-type(2n + 5)";
  const $ = cheerio.load(html, null, false);
  let dl = $("dl");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dl.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      dl.children(selector);
    })
  };
}

function nthLastOfType() {
  const html = `<dl>${"<dt></dt><dd></dd>".repeat(NODECOUNT / 2)}</dl>`;
  const selector = ":nth-last-of-type(2n),:nth-last-of-type(3n)";
  const $ = cheerio.load(html, null, false);
  let dl = $("dl");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dl.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      dl.children(selector);
    })
  };
}

function nthLastOfType10() {
  const html = `<dl>${"<dt></dt><dd></dd>".repeat(NODECOUNT / 2)}</dl>`;
  const selector = ":nth-last-of-type(10)";
  const $ = cheerio.load(html, null, false);
  let dl = $("dl");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dl.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      dl.children(selector);
    })
  };
}

function nthLastOfType2n5() {
  const html = `<dl>${"<dt></dt><dd></dd>".repeat(NODECOUNT / 2)}</dl>`;
  const selector = ":nth-last-of-type(2n+5)";
  const $ = cheerio.load(html, null, false);
  let dl = $("dl");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dl.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      dl.children(selector);
    })
  };
}


function nthChildFind() {
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}</ul>`;
  const selector = ":nth-child(2n),:nth-child(3n),:nth-child(5n)";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.find(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function () {
      ul.find(selector);
    })
  };
}



function main() {
  const result = [
    // loadHtml(),
    // findId(),
    // findClass(),
    // findName(),
    // findAttr(),
    // findPrev(),
    // findPrevAll(),
    // findNext(),
    // findNextAll(),
    // empty(),
    contains(),
    // firstChild(),
    // lastChild(),
    // firstOfType(),
    // lastOfType(),
    // nthChild(),
    // nthChild10(),
    // nthChild2n5(),
    // nthLastChild(),
    // nthLastChild10(),
    // nthLastChild2n5(),
    // nthOfType(),
    // nthOfType10(),
    // nthOfType2n5(),
    // nthLastOfType(),
    // nthLastOfType10(),
    // nthLastOfType2n5(),
    // nthChildFind(),
  ];
  console.log(result);
}

main();