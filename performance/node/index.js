const cheerio = require("cheerio");
const NODECOUNT = 3000;
const LOOPTIMES = 200;

function execTimesAvg(cb){
  const startTime = Date.now();
  for (let i = 0; i < LOOPTIMES; i++) {
    cb();
  }
  const elapsed = Date.now() - startTime;
  return elapsed / LOOPTIMES;
}

function nthChild(){
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}</ul>`;
  const selector = ":nth-child(2n),:nth-child(3n),:nth-child(5n)";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function(){
      ul.children(selector);
    })
  };
}

function nthLastChild(){
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}</ul>`;
  const selector = ":nth-last-child(2n),:nth-last-child(3n),:nth-last-child(5n)";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function(){
      ul.children(selector);
    })
  };
}

function nthOfType(){
  const html = `<dl>${"<dt></dt><dd></dd>".repeat(NODECOUNT/2)}</dl>`;
  const selector = ":nth-of-type(2n),:nth-of-type(3n)";
  const $ = cheerio.load(html, null, false);
  let dl = $("dl");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dl.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function(){
      dl.children(selector);
    })
  };
}

function nthLastOfType(){
  const html = `<dl>${"<dt></dt><dd></dd>".repeat(NODECOUNT/2)}</dl>`;
  const selector = ":nth-last-of-type(2n),:nth-last-of-type(3n)";
  const $ = cheerio.load(html, null, false);
  let dl = $("dl");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dl.children(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function(){
      dl.children(selector);
    })
  };
}

function nthChildFind(){
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}</ul>`;
  const selector = ":nth-child(2n),:nth-child(3n),:nth-child(5n)";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.find(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function(){
      ul.find(selector);
    })
  };
}

function findId(){
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}<li id='target'></li></ul>`;
  const selector = "#target";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.find(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function(){
      ul.find(selector);
    })
  };
}

function findClass(){
  const html = `<ul>${"<li></li>".repeat(NODECOUNT)}<li class='target'></li></ul>`;
  const selector = ".target";
  const $ = cheerio.load(html, null, false);
  let ul = $("ul");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${ul.find(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function(){
      ul.find(selector);
    })
  };
}

function findAttr(){
  const html = `<dl>${"<dt></dt><dd contenteditable></dd>".repeat(NODECOUNT/2)}</dl>`;
  const selector = "[contenteditable]";
  const $ = cheerio.load(html, null, false);
  let dl = $("dl");
  console.log(`Execute: ${selector}`);
  console.log(`Find: ${dl.find(selector).length}`);
  return {
    selector,
    usedTime: execTimesAvg(function(){
      dl.find(selector);
    })
  };
}


function main(){
  const result = [
    nthChild(),
    nthLastChild(),
    nthOfType(),
    nthLastOfType(),
    nthChildFind(),
    findId(),
    findClass(),
    findAttr(),
  ];
  console.log(result);
}

main();