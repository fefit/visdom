# Visdom

A html DOM operation library written in Rust，like the Nodejs's cheerio library, they used the same API style of jQuery。

## Usage

[中文 API 文档](https://github.com/fefit/visdom/wiki/%E4%B8%AD%E6%96%87API%E6%96%87%E6%A1%A3)

Cargo.toml

```toml
[depedencies]
visdom = {git = "https://github.com/fefit/visdom", tag = "v0.0.6", version = "0.0.6"}
```

main.rs

```rust
use visdom::Vis;
use ntree::selector::interface::KindError;

fn main()-> Result<(), KindError>{
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
  let lis = nodes.find("#header li")?;
  println!("{}", lis.text());
  // will output "Hello,VisDom"
}
```

## Vis

Static method：`load(html: &str) -> Result<NodeList, KindError>`

    Load the `html` string into a document `NodeList`

Static method：`dom(ele: &BoxDynNode) -> Result<NodeList, KindError>`

    Change the `ele` node to single node `NodeList`, this will copy the `ele`, you don't need it if you just need do something with methods of the `BoxDynNode` its'own.

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

The following API are inherited from the library [ntree](https://github.com/fefit/ntree) 。

### Selector Operation

| Selector API                                                                   | Description                                                                                                              |                        Remarks                         |
| :----------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------: |
| The caller `Self` is a `NodeList`, Return `Result<NodeList, ErrorKind>`        | Tha all APIs are same with the jQuery library                                                                            |                                                        |
| <b>`find`</b>(selector: &str)                                                  | Get the descendants of each element in the `Self`, filtered by the `selector`.                                           |                                                        |
| <b>`filter`</b>(selector: &str)                                                | Reduce `Self` to those that match the `selector`.                                                                        |                                                        |
| <b>`filter_by`</b>(handle: &#124;index: usize, ele: &BoxDynNode&#124; -> bool) | Reduce `Self` to those that pass the `handle` function test.                                                             |                                                        |
| <b>`filter_in`</b>(node_list: &NodeList)                                       | Reduce `Self` to those that also in the `node_list`                                                                      |                                                        |
| <b>`not`</b>(selector: &str)                                                   | Remove elements those that match the `selector` from `Self`.                                                             |                                                        |
| <b>`not_by`</b>(handle: &#124;index: usize, ele: &BoxDynNode&#124; -> bool)    | Remove elements those that pass the `handle` function test from `Self`.                                                  |                                                        |
| <b>`not_in`</b>(node_list: &NodeList)                                          | Remove elements those that also in the `node_list` from `Self`.                                                          |                                                        |
| <b>`is`</b>(selector: &str)                                                    | Check at least one element in `Self` is match the `selector`.                                                            |                                                        |
| <b>`is_by`</b>(handle: &#124;index: usize, ele: &BoxDynNode&#124; -> bool)     | Check at least one element call the `handle` function return `true`.                                                     |                                                        |
| <b>`is_in`</b>(node_list: &NodeList)                                           | Check at least one element in `Self` is also in `node_list`.                                                             |                                                        |
| <b>`is_all`</b>(selector: &str)                                                | Check if each element in `Self` are all matched the `selector`.                                                          |                                                        |
| <b>`is_all_by`</b>(handle: &#124;index: usize, ele: &BoxDynNode&#124; -> bool) | Check if each element in `Self` call the `handle` function are all returned `true`.                                      |                                                        |
| <b>`is_all_in`</b>(node_list: &NodeList)                                       | Check if each element in `Self` are all also in `node_list`.                                                             |                                                        |
| <b>`has`</b>(selector: &str)                                                   | Reduce `Self` to those that have a descendant that matches the `selector`.                                               |                                                        |
| <b>`has_in`</b>(node_list: &NodeList)                                          | Reduce `Self` to those that have a descendant that in the `node_list`.                                                   |                                                        |
| <b>`children`</b>(selector: &str)                                              | Get the children of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`.           |                                                        |
| <b>`parent`</b>(selector: &str)                                                | Get the parent of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`.             |                                                        |
| <b>`parents`</b>(selector: &str)                                               | Get the ancestors of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`.          |                                                        |
| <b>`siblings`</b>(selector: &str)                                              | Get the siblings of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`.           |                                                        |
| <b>`next`</b>(selector: &str)                                                  | Get the next sibling of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`.       |                                                        |
| <b>`next_all`</b>(selector: &str)                                              | Get all following siblings of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`. |                                                        |
| <b>`prev`</b>(selector: &str)                                                  | Get the previous sibling of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`.   |                                                        |
| <b>`prev_all`</b>(selector: &str)                                              | Get all preceding siblings of each element in `Self`, when the `selector` is not empty, will filtered by the `selector`. |                                                        |
| <b>`eq`</b>(index: usize)                                                      | Get one element at the specified `index`.                                                                                |                                                        |
| <b>`slice`</b>(range: Range)                                                   | Get a subset specified by a range of indices.                                                                            | e.g.:slice(0..=2), will match the first three element. |

### Helpers

| Helper API                                                                             | Description                                                                                                                                                                                                              |                    Remarks                     |
| :------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------: |
| <b>`length`</b>()                                                                      | Get the number of `Self`'s element.                                                                                                                                                                                      |                                                |
| <b>`is_empty`</b>()                                                                    | Check if `Self` has no element, `length() == 0`.                                                                                                                                                                         |                                                |
| <b>`for_each`</b>(handle: &#124;index: usize, ele: &mut BoxDynNode&#124; -> bool)      | Iterate over the elements in `Self`, when the `handle` return `false`, stop the iterator.                                                                                                                                | You can also use `each` if you like less code. |
| <b>`map`</b>&lt;T&gt;(&#124;index: usize, ele: &BoxDynNode&#124; -> T) -> Vec&lt;T&gt; | Get a collection of values by iterate the each element in `Self` and call the `handle` function.                                                                                                                         |                                                |
| <b>`sort`</b>()                                                                        | Sort each element in `Self` by the appear order in the html document.This should only used when you use a `find` method that the selector is a selector list, e.g. `find(".a,.b")`, which `.a` and `.b` are not ordered. |                                                |

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
| <b>`:header`</b>                       | All title tags，alias of: `h1,h2,h3,h4,h5,h6`.                                                                  |                                                                             |
| <b>`:input`</b>                        | All form input tags, alias of: `input,select,textarea,button`.                                                  |                                                                             |
| <b>`:submit`</b>                       | Form submit buttons, alias of: `input\[type="submit"\],button\[type="submit"\]`.                                |                                                                             |

### Attribute Operation

| Attribute API                                                 | Description                                                                                                                                                                      |                                                       Remarks                                                       |
| :------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-----------------------------------------------------------------------------------------------------------------: |
| <b>`attr`</b>(attr_name: &str) -> Option&lt;IAttrValue&gt;    | Get an atrribute of key `attr_name`                                                                                                                                              | The return value is an Option Enum `IAttrValue`, `IAttrValue` has `is_true()`, `is_str(&str)`, `to_list()` methods. |
| <b>`set_attr`</b>(attr_name: &str, value: Option&lt;&str&gt;) | Set an attribute of key `attr_name`，the value is an `Option<&str>`, when the value is `None`，that means the attribute does'n have a string value, it's a bool value of `true`. |                                                                                                                     |
| <b>`remove_attr`</b>(attr_name: &str)                         | Remove an attribute of key `attr_name`.                                                                                                                                          |                                                                                                                     |
| <b>`has_class`</b>(class_name: &str)                          | Check if `Self`'s ClassList contains `class_name`, multiple classes can be splitted by whitespaces.                                                                              |                                                                                                                     |
| <b>`add_class`</b>(class_name: &str)                          | Add class to `Self`'s ClassList, multiple classes can be splitted by whitespaces.                                                                                                |                                                                                                                     |
| <b>`remove_class`</b>(class_name: &str)                       | Remove class from `Self`'s ClassList, multiple classes can be splitted by whitespaces.名                                                                                         |                                                                                                                     |
| <b>`toggle_class`</b>(class_name: &str)                       | Toggle class from `Self`'s ClassList, multiple classes can be splitted by whitespaces.加                                                                                         |                                                                                                                     |

### Content Operation

| Content API                      | Description                                                                | Remarks |
| :------------------------------- | :------------------------------------------------------------------------- | :-----: |
| <b>`text`</b>()                  | Get the text of each element in `Self`，the html entity will auto decoded. |         |
| <b>`set_text`</b>(content: &str) | Set the `Self`'s text, the html entity in `content` will auto encoded.     |         |
| <b>`html`</b>()                  | Get the first element in `Self`'s html.                                    |         |
| <b>`set_html`</b>(content: &str) | Set the html to `content` of each element in `Self`.                       |         |
| <b>`outer_html`</b>()            | Get the first element in `Self`'s outer html.                              |         |

### DOM Operation

| DOM Insertion and Remove API              | Description                                                                         | Remarks |
| :---------------------------------------- | :---------------------------------------------------------------------------------- | :-----: |
| <b>`append`</b>(nodes: &NodeList)         | Append all `nodes` into `Self`, after the last child<BeforeEnd>                     |         |
| <b>`append_to`</b>(nodes: &mut NodeList)  | The same as the above，but exchange the caller and the parameter target.            |         |
| <b>`prepend`</b>(nodes: &mut NodeList)    | Append all `nodes` into `Self`, befpre the first child<AfterStart>                  |         |
| <b>`prepend_to`</b>(nodes: &mut NodeList) | The same as the above，but exchange the caller and the parameter target.            |         |
| <b>`insert_after`</b>(nodes: &NodeList)   | Insert all `nodes` after `Self`<AfterEnd>                                           |         |
| <b>`after`</b>(nodes: &mut NodeList)      | The same as the above，but exchange the caller and the parameter target.            |         |
| <b>`insert_before`</b>(nodes: &NodeList)  | Insert all `nodes` before `Self`<BeforeStart>                                       |         |
| <b>`before`</b>(nodes: &mut NodeList)     | The same as the above，but exchange the caller and the parameter target.            |         |
| <b>`remove`</b>()                         | Remove the `Self`, it will take the ownership of `Self`, so you can't use it again. |         |
| <b>`empty`</b>()                          | Clear the all childs of each element in `Self`.                                     |         |

#### Example

```rust
let html = r##"
  <div id="container">
    <div class="first-child"></div>
    <div class="second-child"></div>
  </div>
"##;
let root = Vis::load(html)?;
let child = root.find(".child")?;
let mut container = root.find("#container")?;
// append the `child` element to the `container`
container.append(&child);
// then the code become to below
/*
<div id="container">
  <div class="first-child"></div>
  <div class="second-child"></div>
</div>
*/
// create new element by `Vis::load`
let third_child = Vis::load(r##"<div class="third-child"></div>"##)?;
container.append(&third_child);
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

- NodeList API Library：[https://github.com/fefit/ntree](https://github.com/fefit/ntree)
- Html Parser：[https://github.com/fefit/rphtml](https://github.com/fefit/rphtml)
- Html Entity encode and decode：[https://github.com/fefit/htmlentity](https://github.com/fefit/htmlentity)

## Questions & Advices & Bugs?

Welcome to report [Issue](https://github.com/fefit/visdom/issues) to us if you have any question or bug or good advice.

## License

[MIT License](./LICENSE).
