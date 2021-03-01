# Visdom

[![Build Status](https://travis-ci.org/fefit/visdom.svg?branch=main)](https://travis-ci.com/github/fefit/visdom)
[![crates.io](https://img.shields.io/crates/v/visdom.svg)](https://crates.io/crates/visdom)
[![tag](https://img.shields.io/github/v/tag/fefit/visdom.svg?sort=semver)](https://github.com/fefit/visdom/tags)
[![GitHub license](https://img.shields.io/github/license/fefit/visdom)](https://github.com/fefit/visdom/blob/main/LICENSE)

A server-side html document syntax and operation library written in Rust, it uses apis similar to [jQuery](https://jquery.com), left off the parts thoes only worked in browser(e.g. render and event related methods), and use names with snake-case instead of camel-case in javasript.

It's not only helpful for the working with web scraping, but also supported useful apis to operate `text` nodes, so you can use it to mix your html with dirty html segement to keep away from web scrapers.

## Usage

[中文 API 文档](https://github.com/fefit/visdom/wiki/%E4%B8%AD%E6%96%87API%E6%96%87%E6%A1%A3)&nbsp;&nbsp;&nbsp;&nbsp;[CHANGELOG](https://github.com/fefit/visdom/blob/main/CHANGELOG.md)&nbsp;&nbsp;&nbsp;&nbsp;[Live Demo](http://visdom.suchjs.com/#selector-id)

main.rs

```rust
use visdom::Vis;
use std::error::Error;

fn main()-> Result<(), Box<dyn Error>>{
  let html = r##"
    <Doctype html>
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
  let nodes = Vis::load(html)?;
  let lis_text = nodes.find("#header li").text();
  println!("{}", lis_text);
  // will output "Hello,VisDom"
  Ok(())
}
```

## Vis

Static method：`load(html: &str) -> Result<Elements, Box<dyn Error>>`

    Load the `html` string into an `Elements` collection.

Static method：`load_catch(html: &str, handle: Box<dyn Fn(Box<dyn Error>)>) -> Elements`

    Load the `html` string into an `Elements` collection, and use the handle to do with the errors such as html parse error, wrong selectors, this is useful if you don't want the process is paniced by the errors.

Static method：`load_options(html: &str, options: html::ParseOptions) -> Result<Elements, Box<dyn Error>>`

    This method allowed you to define the parse options when parsing the `html` string into a document tree, the `load` method is just an alias method of this,  with the most compatible parse options parameter.

```rust
// the `load` and `load_catch` use the parse options as below
// more about the `ParseOptions`, you can see the document of `rphtml` library.
ParseOptions{
  auto_fix_unclosed_tag: true,
  auto_fix_unexpected_endtag: true,
  auto_fix_unescaped_lt: true,
  allow_self_closing: true,
}
```

Static method：`load_options_catch(html: &str, options: html::ParseOptions, handle: Box<dyn Fn(Box<dyn Error>)>) -> Elements`

    It's same as `load` and `load_options` methods, just exposed a parse options parameter so that you can define how to resolve errors when parsing html.

Static method：`dom(ele: &BoxDynElement) -> Elements`

    Change the `ele` node to single node `Elements`, this will copy the `ele`, you don't need it if you just need do something with methods of the `BoxDynElement` its'own.

e.g.：

```rust
// go on the code before
let texts = lis.map(|_index, ele|{
  let ele = Vis::dom(ele);
  return String::from(ele.text());
});
// now `texts` will be a `Vec<String>`: ["Hello,", "Vis", "Dom"]
```

## API

The following API are inherited from the library [mesdoc](https://github.com/fefit/mesdoc) 。

### Trait methods

| Instance                      | Trait          | Inherit    | Document                                                                                            |
| :---------------------------- | :------------- | :--------- | :-------------------------------------------------------------------------------------------------- |
| BoxDynNode                    | INodeTrait     | None       | [INodeTrait Document](https://docs.rs/mesdoc/latest/mesdoc/interface/trait.INodeTrait.html)         |
| BoxDynElement                 | IElementTrait  | INodeTrait | [IElementTrait Document](https://docs.rs/mesdoc/latest/mesdoc/interface/trait.IElementTrait.html)   |
| BoxDynText                    | ITextTrait     | INodeTrait | [ITextTrait Document](https://docs.rs/mesdoc/latest/mesdoc/interface/trait.ITextTrait.html)         |
| Box&lt;dyn IDocumentTrait&gt; | IDocumentTrait | None       | [IDocumentTrait Document](https://docs.rs/mesdoc/latest/mesdoc/interface/trait.IDocumentTrait.html) |

### Collections APIs

| Collections | Document                                                                                 |
| :---------- | :--------------------------------------------------------------------------------------- |
| Elements    | [Elements Document](https://docs.rs/mesdoc/latest/mesdoc/interface/struct.Elements.html) |
| Texts       | [Texts Document](https://docs.rs/mesdoc/latest/mesdoc/interface/struct.Texts.html)       |

### Selector Operation

| Selector API                                                                      | Description                                                                                                                                                                                                                                                                          |                       Remarks                        |
| :-------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------: |
| The caller `Self` is a `Elements`, Return `Elements`                              | Tha all APIs are same with the jQuery library                                                                                                                                                                                                                                        |                                                      |
| <b>`find`</b>(selector: &str)                                                     | Get the descendants of each element in the `Self`, filtered by the `selector`.                                                                                                                                                                                                       |                                                      |
| <b>`filter`</b>(selector: &str)                                                   | Reduce `Self` to those that match the `selector`.                                                                                                                                                                                                                                    |                                                      |
| <b>`filter_by`</b>(handle: &#124;index: usize, ele: &BoxDynElement&#124; -> bool) | Reduce `Self` to those that pass the `handle` function test.                                                                                                                                                                                                                         |                                                      |
| <b>`filter_in`</b>(elements: &Elements)                                           | Reduce `Self` to those that also in the `elements`                                                                                                                                                                                                                                   |                                                      |
| <b>`not`</b>(selector: &str)                                                      | Remove elements those that match the `selector` from `Self`.                                                                                                                                                                                                                         |                                                      |
| <b>`not_by`</b>(handle: &#124;index: usize, ele: &BoxDynElement&#124; -> bool)    | Remove elements those that pass the `handle` function test from `Self`.                                                                                                                                                                                                              |                                                      |
| <b>`not_in`</b>(elements: &Elements)                                              | Remove elements those that also in the `elements` from `Self`.                                                                                                                                                                                                                       |                                                      |
| <b>`is`</b>(selector: &str)                                                       | Check at least one element in `Self` is match the `selector`.                                                                                                                                                                                                                        |                                                      |
| <b>`is_by`</b>(handle: &#124;index: usize, ele: &BoxDynElement&#124; -> bool)     | Check at least one element call the `handle` function return `true`.                                                                                                                                                                                                                 |                                                      |
| <b>`is_in`</b>(elements: &Elements)                                               | Check at least one element in `Self` is also in `elements`.                                                                                                                                                                                                                          |                                                      |
| <b>`is_all`</b>(selector: &str)                                                   | Check if each element in `Self` are all matched the `selector`.                                                                                                                                                                                                                      |                                                      |
| <b>`is_all_by`</b>(handle: &#124;index: usize, ele: &BoxDynElement&#124; -> bool) | Check if each element in `Self` call the `handle` function are all returned `true`.                                                                                                                                                                                                  |                                                      |
| <b>`is_all_in`</b>(elements: &Elements)                                           | Check if each element in `Self` are all also in `elements`.                                                                                                                                                                                                                          |                                                      |
| <b>`has`</b>(selector: &str)                                                      | Reduce `Self` to those that have a descendant that matches the `selector`.                                                                                                                                                                                                           |                                                      |
| <b>`has_in`</b>(elements: &Elements)                                              | Reduce `Self` to those that have a descendant that in the `elements`.                                                                                                                                                                                                                |                                                      |
| <b>`children`</b>(selector: &str)                                                 | Get the children of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`.                                                                                                                                                                       |                                                      |
| <b>`parent`</b>(selector: &str)                                                   | Get the parent of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`.                                                                                                                                                                         |                                                      |
| <b>`parents`</b>(selector: &str)                                                  | Get the ancestors of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`.                                                                                                                                                                      |                                                      |
| <b>`parents_until`</b>(selector: &str, filter: &str, contains: bool)              | Get the ancestors of each element in `Self`, until the ancestor matched the `selector`, when `contains` is true, the matched ancestor will be included, otherwise it will exclude; when the `filter` is not empty, will filtered by the `selector`;                                  |                                                      |
| <b>`closest`</b>(selector: &str)                                                  | Get the first matched element of each element in `Self`, traversing from self to it's ancestors.                                                                                                                                                                                     |                                                      |
| <b>`siblings`</b>(selector: &str)                                                 | Get the siblings of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`.                                                                                                                                                                       |                                                      |
| <b>`next`</b>(selector: &str)                                                     | Get the next sibling of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`.                                                                                                                                                                   |                                                      |
| <b>`next_all`</b>(selector: &str)                                                 | Get all following siblings of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`.                                                                                                                                                             |                                                      |
| <b>`next_until`</b>(selector: &str, filter: &str, contains: bool)                 | Get all following siblings of each element in `Self`, until the sibling element matched the `selector`, when `contains` is true, the matched sibling will be included, otherwise it will exclude; when the `filter` is not empty, will filtered by the `selector`;                   |
| <b>`prev`</b>(selector: &str)                                                     | Get the previous sibling of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`.                                                                                                                                                               |                                                      |
| <b>`prev_all`</b>(selector: &str)                                                 | Get all preceding siblings of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`.                                                                                                                                                             |                                                      |
| <b>`prev_until`</b>(selector: &str, filter: &str, contains: bool)                 | Get all preceding siblings of each element in `Self`, until the previous sibling element matched the `selector`, when `contains` is true, the matched previous sibling will be included, otherwise it will exclude; when the `filter` is not empty, will filtered by the `selector`; |
| <b>`eq`</b>(index: usize)                                                         | Get one element at the specified `index`.                                                                                                                                                                                                                                            |                                                      |
| <b>`first`</b>()                                                                  | Get the first element of the set,equal to `eq(0)`.                                                                                                                                                                                                                                   |                                                      |
| <b>`last`</b>()                                                                   | Get the last element of the set, equal to `eq(len - 1)`.                                                                                                                                                                                                                             |                                                      |
| <b>`slice`</b><T: RangeBounds>(range: T)                                          | Get a subset specified by a range of indices.                                                                                                                                                                                                                                        | e.g.:slice(..3), will match the first three element. |
| <b>`add`</b>(eles: Elements)                                                      | Get a concated element set from `Self` and `eles`, it will generate a new element set, take the ownership of the parameter `eles`, but have no sence with `Self`                                                                                                                     |                                                      |

### Helpers

| Helper API                                                                                | Description                                                                                      |                    Remarks                     |
| :---------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------- | :--------------------------------------------: |
| <b>`length`</b>()                                                                         | Get the number of `Self`'s element.                                                              |                                                |
| <b>`is_empty`</b>()                                                                       | Check if `Self` has no element, `length() == 0`.                                                 |                                                |
| <b>`for_each`</b>(handle: &#124;index: usize, ele: &mut BoxDynElement&#124; -> bool)      | Iterate over the elements in `Self`, when the `handle` return `false`, stop the iterator.        | You can also use `each` if you like less code. |
| <b>`map`</b>&lt;T&gt;(&#124;index: usize, ele: &BoxDynElement&#124; -> T) -> Vec&lt;T&gt; | Get a collection of values by iterate the each element in `Self` and call the `handle` function. |                                                |

### Supported Selectors

| Selectors                              | Description                                                                                                     |                                   Remarks                                   |
| :------------------------------------- | :-------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------: |
| <b>`*`</b>                             | [MDN Universal Selectors](https://developer.mozilla.org/en-US/docs/Web/CSS/Universal_selectors)                 |                                                                             |
| <b>`#id`</b>                           | [MDN Id Selector](https://developer.mozilla.org/en-US/docs/Web/CSS/ID_selectors)                                |                                                                             |
| <b>`.class`</b>                        | [MDN Class Selector](https://developer.mozilla.org/en-US/docs/Web/CSS/Class_selectors)                          |                                                                             |
| <b>`p`</b>                             | [MDN Type Selectors](https://developer.mozilla.org/en-US/docs/Web/CSS/Type_selectors)                           |                                                                             |
| <b>`[attr]`</b>                        | [MDN Attribute Selectors](https://developer.mozilla.org/en-US/docs/Web/CSS/Attribute_selectors)                 |                                                                             |
| <b>`[attr=value]`</b>                  | See the above.                                                                                                  |                                                                             |
| <b>`[attr*=value]`</b>                 | See the above.                                                                                                  |                                                                             |
| <b><code>[attr&#124;=value]</code></b> | See the above.                                                                                                  |                                                                             |
| <b>`[attr~=value]`</b>                 | See the above.                                                                                                  |                                                                             |
| <b>`[attr^=value]`</b>                 | See the above.                                                                                                  |                                                                             |
| <b>`[attr$=value]`</b>                 | See the above.                                                                                                  |                                                                             |
| <b>`[attr!=value]`</b>                 | jQuery supported, match the element that has an attribute of `attr`，but it's value is not equal to `value`.    |                                                                             |
| <b>`span > a`</b>                      | [MDN Child Combinator](https://developer.mozilla.org/en-US/docs/Web/CSS/Child_combinator)                       |           match the element of `a` that who's parent is a `span`            |
| <b>`span a`</b>                        | [MDN Descendant Combinator](https://developer.mozilla.org/en-US/docs/Web/CSS/Descendant_combinator)             |                                                                             |
| <b>`span + a`</b>                      | [MDN Adjacent Sibling Combinator](https://developer.mozilla.org/en-US/docs/Web/CSS/Adjacent_sibling_combinator) |                                                                             |
| <b>`span ~ a`</b>                      | [MDN Generic Sibling Combinator](https://developer.mozilla.org/en-US/docs/Web/CSS/General_sibling_combinator)   |                                                                             |
| <b>`span,a`</b>                        | [MDN Selector list](https://developer.mozilla.org/en-US/docs/Web/CSS/Selector_list)                             |                                                                             |
| <b>`span.a`</b>                        | Adjoining Selectors                                                                                             | match an element that who's tag type is `span` and also has a class of `.a` |
| <b>`:empty`</b>                        | [MDN `:empty`](https://developer.mozilla.org/en-US/docs/Web/CSS/:empty)                                         |                              Pseudo Selectors                               |
| <b>`:first-child`</b>                  | [MDN `:first-child`](https://developer.mozilla.org/en-US/docs/Web/CSS/:first-child)                             |                                                                             |
| <b>`:last-child`</b>                   | [MDN `:last-child`](https://developer.mozilla.org/en-US/docs/Web/CSS/:last-child)                               |                                                                             |
| <b>`:only-child`</b>                   | [MDN `:only-child`](https://developer.mozilla.org/en-US/docs/Web/CSS/:only-child)                               |                                                                             |
| <b>`:nth-child(nth)`</b>               | [MDN `:nth-child()`](https://developer.mozilla.org/en-US/docs/Web/CSS/:nth-child)                               |                   `nth` support keyword `odd` and `even`                    |
| <b>`:nth-last-child(nth)`</b>          | [MDN `:nth-last-child()`](https://developer.mozilla.org/en-US/docs/Web/CSS/::nth-last-child)                    |                                                                             |
| <b>`:first-of-type`</b>                | [MDN `:first-of-type`](https://developer.mozilla.org/en-US/docs/Web/CSS/:first-of-type)                         |                                                                             |
| <b>`:last-of-type`</b>                 | [MDN `:last-of-type`](https://developer.mozilla.org/en-US/docs/Web/CSS/:last-of-type)                           |                                                                             |
| <b>`:only-of-type`</b>                 | [MDN `:only-of-type`](https://developer.mozilla.org/en-US/docs/Web/CSS/:only-of-type)                           |                                                                             |
| <b>`:nth-of-type(nth)`</b>             | [MDN `:nth-of-type()`](https://developer.mozilla.org/en-US/docs/Web/CSS/:nth-of-type)                           |                                                                             |
| <b>`:nth-last-of-type(nth)`</b>        | [MDN `:nth-last-of-type()`](https://developer.mozilla.org/en-US/docs/Web/CSS/:nth-last-of-type)                 |                                                                             |
| <b>`:not(selector)`</b>                | [MDN `:not()`](https://developer.mozilla.org/en-US/docs/Web/CSS/:not)                                           |                                                                             |
| <b>`:contains(content)`</b>            | Match the element who's `text()` contains the content.                                                          |                                                                             |
| <b>`:header`</b>                       | All title tags，alias of: `h1,h2,h3,h4,h5,h6`.                                                                  |                                                                             |
| <b>`:input`</b>                        | All form input tags, alias of: `input,select,textarea,button`.                                                  |                                                                             |
| <b>`:submit`</b>                       | Form submit buttons, alias of: `input\[type="submit"\],button\[type="submit"\]`.                                |                                                                             |

### Attribute Operation

| Attribute API                                                 | Description                                                                                                                                                                      |                                                       Remarks                                                       |
| :------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-----------------------------------------------------------------------------------------------------------------: |
| <b>`attr`</b>(attr_name: &str) -> Option&lt;IAttrValue&gt;    | Get an atrribute of key `attr_name`                                                                                                                                              | The return value is an Option Enum `IAttrValue`, `IAttrValue` has `is_true()`, `is_str(&str)`, `to_list()` methods. |
| <b>`set_attr`</b>(attr_name: &str, value: Option&lt;&str&gt;) | Set an attribute of key `attr_name`，the value is an `Option<&str>`, when the value is `None`，that means the attribute does'n have a string value, it's a bool value of `true`. |                                                                                                                     |
| <b>`remove_attr`</b>(attr_name: &str)                         | Remove an attribute of key `attr_name`.                                                                                                                                          |                                                                                                                     |
| <b>`has_class`</b>(class_name: &str) -> bool                  | Check if `Self`'s ClassList contains `class_name`, multiple classes can be splitted by whitespaces.                                                                              |                                                                                                                     |
| <b>`add_class`</b>(class_name: &str)                          | Add class to `Self`'s ClassList, multiple classes can be splitted by whitespaces.                                                                                                |                                                                                                                     |
| <b>`remove_class`</b>(class_name: &str)                       | Remove class from `Self`'s ClassList, multiple classes can be splitted by whitespaces.                                                                                           |                                                                                                                     |
| <b>`toggle_class`</b>(class_name: &str)                       | Toggle class from `Self`'s ClassList, multiple classes can be splitted by whitespaces.                                                                                           |                                                                                                                     |

### Content Operation

| Content API                               | Description                                                                                                                                                                                                                                                                                                                                     | Remarks |
| :---------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-----: |
| <b>`text`</b>() -> &str                   | Get the text of each element in `Self`，the html entity will auto decoded.                                                                                                                                                                                                                                                                      |         |
| <b>`set_text`</b>(content: &str)          | Set the `Self`'s text, the html entity in `content` will auto encoded.                                                                                                                                                                                                                                                                          |         |
| <b>`html`</b>()                           | Get the first element in `Self`'s html.                                                                                                                                                                                                                                                                                                         |         |
| <b>`set_html`</b>(content: &str)          | Set the html to `content` of each element in `Self`.                                                                                                                                                                                                                                                                                            |         |
| <b>`outer_html`</b>()                     | Get the first element in `Self`'s outer html.                                                                                                                                                                                                                                                                                                   |         |
| <b>`texts`</b>(limit_depth: u32) -> Texts | Get the text node of each element in `Self`, if `limit_depth` is `0`, will get all the descendant text nodes; if `1`, will just get the children text nodes.`Texts` not like `Elements`, it doesn't have methods by implemented the `IElementTrait` trait, but it has `append_text` and `prepend_text` methods by implemented the `ITextTrait`. |         |

### DOM Operation

| DOM Insertion and Remove API                    | Description                                                                         | Remarks |
| :---------------------------------------------- | :---------------------------------------------------------------------------------- | :-----: |
| <b>`append`</b>(elements: &Elements)            | Append all `elements` into `Self`, after the last child<BeforeEnd>                  |         |
| <b>`append_to`</b>(elements: &mut Elements)     | The same as the above，but exchange the caller and the parameter target.            |         |
| <b>`prepend`</b>(elements: &mut Elements)       | Append all `elements` into `Self`, befpre the first child<AfterStart>               |         |
| <b>`prepend_to`</b>(elements: &mut Elements)    | The same as the above，but exchange the caller and the parameter target.            |         |
| <b>`insert_after`</b>(elements: &mut Elements)  | Insert all `elements` after `Self`<AfterEnd>                                        |         |
| <b>`after`</b>(elements: &mut Elements)         | The same as the above，but exchange the caller and the parameter target.            |         |
| <b>`insert_before`</b>(elements: &mut Elements) | Insert all `elements` before `Self`<BeforeStart>                                    |         |
| <b>`before`</b>(elements: &mut Elements)        | The same as the above，but exchange the caller and the parameter target.            |         |
| <b>`remove`</b>()                               | Remove the `Self`, it will take the ownership of `Self`, so you can't use it again. |         |
| <b>`empty`</b>()                                | Clear the all childs of each element in `Self`.                                     |         |

#### Example

```rust
let html = r##"
  <div class="second-child"></div>
  <div id="container">
    <div class="first-child"></div>
  </div>
"##;
let root = Vis::load(html)?;
let mut container = root.find("#container");
let mut second_child = root.find(".second-child");
// append the `second-child` element to the `container`
container.append(&mut second_child);
// then the code become to below
/*
<div id="container">
  <div class="first-child"></div>
  <div class="second-child"></div>
</div>
*/
// create new element by `Vis::load`
let mut third_child = Vis::load(r##"<div class="third-child"></div>"##)?;
container.append(&mut third_child);
// then the code become to below
/*
<div id="container">
  <div class="first-child"></div>
  <div class="second-child"></div>
  <div class="third-child"></div>
</div>
*/
```

## Depedencies

- Elements API Library：[https://github.com/fefit/mesdoc](https://github.com/fefit/mesdoc)
- Html Parser：[https://github.com/fefit/rphtml](https://github.com/fefit/rphtml)
- Html Entity encode and decode：[https://github.com/fefit/htmlentity](https://github.com/fefit/htmlentity)

## Questions & Advices & Bugs?

Welcome to report [Issue](https://github.com/fefit/visdom/issues) to us if you have any question or bug or good advice.

## License

[MIT License](./LICENSE).
