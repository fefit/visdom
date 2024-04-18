<h1 align="center">

<strong>Visdom</strong>

[![Build status](https://github.com/fefit/visdom/actions/workflows/test.yml/badge.svg)](https://github.com/fefit/visdom/actions)
[![crates.io](https://img.shields.io/crates/v/visdom.svg)](https://crates.io/crates/visdom)
[![tag](https://img.shields.io/github/v/tag/fefit/visdom.svg?sort=semver)](https://github.com/fefit/visdom/tags)
[![codecov](https://codecov.io/gh/fefit/visdom/branch/main/graph/badge.svg)](https://codecov.io/gh/fefit/visdom)
[![Crates download](https://img.shields.io/crates/d/visdom.svg)](https://crates.io/crates/visdom)
[![docs.rs](https://img.shields.io/badge/docs.rs-visdom-green)](https://docs.rs/visdom/latest)
[![GitHub license](https://img.shields.io/github/license/fefit/visdom)](https://github.com/fefit/visdom/blob/main/LICENSE)

</h1>
<h4 align="center">

[API Document](https://github.com/fefit/visdom/wiki/API-Document)&nbsp;&nbsp;&nbsp;&nbsp;
[Performance](https://github.com/fefit/visdom/blob/main/performance/README.md)&nbsp;&nbsp;&nbsp;&nbsp;
[中文 API 文档](https://github.com/fefit/visdom/wiki/%E4%B8%AD%E6%96%87API%E6%96%87%E6%A1%A3)&nbsp;&nbsp;&nbsp;&nbsp;
[更新文档](https://github.com/fefit/visdom/blob/main/CHANGELOG.md)

</h4>
<p>
:house:  A html parsing & node selecting and mutation library written in Rust, using APIs similar to <a href="https://www.jquery.com" target="_blank">jQuery</a>, left off the parts thoes only worked in the browsers(e.g. render and event related methods).

It's not only helpful for the working with html scraping, but also have useful APIs to mutate `text` nodes, so you can use it for mixing your html with dirty html fragement, and keep the web scrapers away. :sparkling_heart:

</p>

## Usage

```rust
use visdom::Vis;
use visdom::types::BoxDynError;

fn main() -> Result<(), BoxDynError>{
  let html = r##"
    <!DOCTYPE html>
    <html>
      <head>
        <meta charset="utf-8" />
      </head>
      <body>
        <nav id="header">
          <ul>
            <li>Hello,</li>
            <li>Vis</li>
            <li>Dom</li>
          </ul>
        </nav>
      </body>
    </html>
  "##;
  // load html
  let root = Vis::load(html)?;
  let lis = root.find("#header li");
  let lis_text = lis.text();
  println!("{}", lis_text);
  // will output "Hello,VisDom"
  Ok(())
}
```

[Try it online](http://visdom.suchjs.com/#hello)

## Feature flags

After version v0.5.0, visdom add some feature flags to support conditional compilation for different usage.

| Feature     | Description                                                                         | API                                                                                                                          | Config                                                |
| :---------- | :---------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------- |
| `destroy`   | When you don't need remove or clear the elements, you can ignore this feature flag. | `.remove()` `.empty()` (IElementTrait) `remove_child()` `clone()`                                                            | `visdom = { version = xxx, features = ["destroy"]}`   |
| `insertion` | When you don't need mutation the DOM, you can ignore this feature flag.             | `append()` `append_to()` `prepend()` `prepend_to()` `insert_after()` `after()` `insert_before()` `before()` `replace_with()` | `visdom = { version = xxx, features = ["insertion"]}` |
| `text`      | When you don't need mutation the TextNode, you can ignore this feature flag.        | `.texts()` `.texts_by()` `texts_by_rec()`                                                                                    | `visdom = { version = xxx, features = ["text"]}`      |
| `full`      | When you need all the API above, you can open this feature flag.                    | -                                                                                                                            | `visdom = { version = xxx, features = ["full"]}`      |

## Depedencies

- Html parser：[https://github.com/fefit/rphtml](https://github.com/fefit/rphtml)
- Html entity encoding and decoding：[https://github.com/fefit/htmlentity](https://github.com/fefit/htmlentity)

## Questions & Advices & Bugs?

Welcome to report [Issue](https://github.com/fefit/visdom/issues) to us if you have any question or bug or good advice.

## License

[MIT License](./LICENSE).
