# Visdom

一个 Rust 编写的 HTML 文档操作库，与 Nodejs 的 cheerio 类似，它的 API 风格基本与 jQuery 保持一致。

## 使用方式

Cargo.toml

```toml
[depedencies]
visdom = "0.0.1"
```

main.rs

```rust
use visdom::Vis;

fn main()-> Result<(), &'static str>{
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
  let nodes = Vis::load(html)?;
  let lis = nodes.find("#header li")?;
  println!("{}", lis.text());
  // 将输出 "Hello,VisDom"
}
```

## API

以下 API 接口由 [ntree](https://github.com/fefit/ntree) 接口库实现。

### 选择器操作

| 选择器方法及其它                 | 说明                                           |              备注              |
| :------------------------------- | :--------------------------------------------- | :----------------------------: |
| <b>`find`</b>(selector:&str)     | 查找匹配选择器的子孙元素                       | 选择器方法均可参见 jQuery 文档 |
| <b>`filter`</b>(selector:&str)   | 筛选出匹配选择器的元素                         |                                |
| <b>`not`</b>(selector:&str)      | 排除匹配选择器的元素                           |                                |
| <b>`is`</b>(selector:&str)       | 判断是否所有元素都匹配选择器                   |                                |
| <b>`children`</b>(selector:&str) | 从子元素开始，查找匹配选择器的元素             |                                |
| <b>`parent`</b>(selector:&str)   | 从父元素开始，查找匹配选择器的元素             |                                |
| <b>`parents`</b>(selector:&str)  | 从父元素及祖先元素开始，查找匹配选择器的元素   |                                |
| <b>`siblings`</b>(selector:&str) | 从兄弟元素开始，查找匹配选择器的元素           |                                |
| <b>`next`</b>(selector:&str)     | 从后一个紧挨兄弟元素开始，查找匹配选择器的元素 |                                |
| <b>`next_all`</b>(selector:&str) | 从后面所有兄弟元素开始，查找匹配选择器的元素   |                                |
| <b>`prev`</b>(selector:&str)     | 从前一个紧挨兄弟元素开始，查找匹配选择器的元素 |                                |
| <b>`prev_all`</b>(selector:&str) | 从前面所有兄弟元素开始，查找匹配选择器的元素   |                                |
| <b>`eq`</b>(index:usize)         | 获取元素列表中第 index 个                      |                                |
| <b>`length`</b>()                | 元素集合长度                                   |                                |
| <b>`is_empty`</b>()              | 元素集合是否为空                               |                                |

### 支持选择器

| 选择器                                 | 说明                                                                                                                         |                   备注                    |
| :------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------: |
| <b>`*`</b>                             | 所有元素                                                                                                                     |    元素选择器可参见 MDN css 选择器文档    |
| <b>`#id`</b>                           | id 选择器                                                                                                                    |                                           |
| <b>`.class`</b>                        | 类选择器                                                                                                                     |                                           |
| <b>`p`</b>                             | 标签名选择器                                                                                                                 |                                           |
| <b>`[attr]`</b>                        | 含有{attr}的元素                                                                                                             |                                           |
| <b>`[attr=value]`</b>                  | {attr}值为{value}的元素                                                                                                      |                                           |
| <b>`[attr*=value]`</b>                 | {attr}包含{value}值的元素                                                                                                    |                                           |
| <b><code>[attr&#124;=value]</code></b> | {attr}包含{value}值或者{value}-的元素                                                                                        |                                           |
| <b>`[attr~=value]`</b>                 | {attr}包含{value}值，且值是以空格作为分隔的元素                                                                              |                                           |
| <b>`[attr^=value]`</b>                 | {attr}以{value}值开头的元素                                                                                                  |                                           |
| <b>`[attr$=value]`</b>                 | {attr}以{value}值结尾的元素                                                                                                  |                                           |
| <b>`[attr!=value]`</b>                 | 包含{attr}，且值不为{value}的元素                                                                                            |                                           |
| <b>`span > a`</b>                      | 子元素选择器                                                                                                                 |        匹配父元素为 span 的 a 元素        |
| <b>`span a`</b>                        | 子孙元素选择器                                                                                                               |     匹配 span 元素下的所有子孙 a 元素     |
| <b>`span + a`</b>                      | 相邻元素选择器                                                                                                               |      匹配 span 后面紧邻的兄弟 a 元素      |
| <b>`span ~ a`</b>                      | 后面兄弟元素选择器                                                                                                           |      匹配 span 后面所有的兄弟 a 元素      |
| <b>`span.a`</b>                        | 多条件筛选选择器                                                                                                             |    匹配 span 且 class 名包含 a 的元素     |
| <b>`:empty`</b>                        | 没有子元素的元素                                                                                                             |            以下都为伪类选择器             |
| <b>`:first-child`</b>                  | 第一个子元素                                                                                                                 |                                           |
| <b>`:last-child`</b>                   | 最后一个子元素                                                                                                               |                                           |
| <b>`:only-child`</b>                   | 唯一子元素                                                                                                                   |                                           |
| <b>`:nth-child(nth)`</b>               | nth 表示为 a'n + b'，a'和 b'为整数<零及正负>，n 从 0 开始计数，和为 1 则表示第一个子元素，最终将获取所有符合该数列值的子元素 | nth 形式的选择器都支持 odd 和 even 关键字 |
| <b>`:nth-last-child(nth)`</b>          | 同上，但从最后一个子元素开始计数算作第一个子元素                                                                             |                                           |
| <b>`:first-of-type`</b>                | 子元素中第一个出现的标签元素<按标签名>                                                                                       |                                           |
| <b>`:last-of-type`</b>                 | 子元素中最后一个出现的标签元素<按标签名>                                                                                     |                                           |
| <b>`:only-of-type`</b>                 | 子元素中只出现一次的标签元素<按标签名>                                                                                       |                                           |
| <b>`:nth-of-type(nth)`</b>             | 子元素中标签<按标签名>出现顺序符合数列值的元素                                                                               |                                           |
| <b>`:nth-last-of-type(nth)`</b>        | 同上，但出现顺序从最后一个元素往前数                                                                                         |                                           |
| <b>`:not(selector)`</b>                | 匹配不符合 selector 选择器的元素                                                                                             |                                           |
| <b>`:header`</b>                       | 所有标题元素，h1,h2,h3,h4,h5,h6 的别名                                                                                       |                                           |
| <b>`:input`</b>                        | 所有表单元素，input,select,textarea,button 的别名                                                                            |                                           |
| <b>`:submit`</b>                       | 表单提交按钮，input\[type="submit"\],button\[type="submit"\] 的别名                                                          |                                           |

### 属性操作

| 属性方法                                               | 说明                                                                 | 备注 |
| :----------------------------------------------------- | :------------------------------------------------------------------- | :--: |
| <b>`attr`</b>(attr_name:&str)                          | 获取属性                                                             |      |
| <b>`set_attr`</b>(attr_name:&str, value: Option<&str>) | 设置属性值，当 value 为 None 时，表示设置布尔 true，没有字符串属性值 |      |
| <b>`remove_attr`</b>(attr_name:&str)                   | 删除属性                                                             |      |
| <b>`add_class`</b>(class_name:&str)                    | 增加 class 类名，多个 class 用空格隔开                               |      |
| <b>`remove_class`</b>(class_name:&str)                 | 删除 class 类名                                                      |      |
| <b>`toggle_class`</b>(class_name:&str)                 | 切换 class 类名，存在则删除，不存在则添加                            |      |

### 文本操作

| 文本方法                        | 说明                                            | 备注 |
| :------------------------------ | :---------------------------------------------- | :--: |
| <b>`text`</b>()                 | 获取所有元素的文本内容，实体将会自动 decode     |      |
| <b>`set_text`</b>(content:&str) | 设置元素的内容为 content 文本<自动 encode 实体> |      |
| <b>`html`</b>()                 | 获取第一个元素的 html 文档内容                  |      |
| <b>`set_html`</b>(content:&str) | 设置元素的子节点为 content 解析后的子节点       |      |
| <b>`outer_html`</b>()           | 获取第一个元素的 html 文档内容，包含节点本身    |      |

### 节点操作

| dom 节点操作方法                          | 说明                                     | 备注 |
| :---------------------------------------- | :--------------------------------------- | :--: |
| <b>`append`</b>(nodes: &NodeList)         | 将所有节点插入节点子元素最后<BeforeEnd>  |      |
| <b>`append_to`</b>(nodes: &mut NodeList)  | 同上，但交换参数与调用者                 |      |
| <b>`prepend`</b>(nodes: &mut NodeList)    | 将所有节点插入节点子元素开始<AfterStart> |      |
| <b>`prepend_to`</b>(nodes: &mut NodeList) | 同上，但交换参数与调用者                 |      |
| <b>`insert_after`</b>(nodes: &NodeList)   | 将所有节点插入该元素之后<AfterEnd>       |      |
| <b>`after`</b>(nodes: &mut NodeList)      | 同上，但交换参数与调用者                 |      |
| <b>`insert_before`</b>(nodes: &NodeList)  | 将所有节点插入该元素之前<BeforeStart>    |      |
| <b>`before`</b>(nodes: &mut NodeList)     | 同上，但交换参数与调用者                 |      |
| <b>`remove`</b>()                         | 删除节点，删除后持有的变量将不能再使用   |      |

#### 示例代码

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
// 将child元素转移到container
container.append(&child);
// 代码将变成
/*
<div id="container">
  <div class="first-child"></div>
  <div class="second-child"></div>
</div>
*/
let third_child = Vis::load(r##"<div class="third-child"></div>"##)?;
container.append(&third_child);
// 代码将变成
/*
<div id="container">
  <div class="first-child"></div>
  <div class="second-child"></div>
  <div class="third-child"></div>
</div>
*/
```

## 依赖

- 接口库：[https://github.com/fefit/ntree](https://github.com/fefit/ntree)
- html 解析库：[https://github.com/fefit/rphtml](https://github.com/fefit/rphtml)
- html 实体解析：[https://github.com/fefit/htmlentity](https://github.com/fefit/htmlentity)

## 问题 & 建议 & Bugs?

如果您在使用过程中遇到任何问题，或者有好的建议，欢迎提供 Issue. [Issue](https://github.com/fefit/visdom/issues)

## License

[MIT License](./LICENSE).
