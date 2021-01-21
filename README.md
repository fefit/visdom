# Visdom

一个 Rust 编写的 HTML 文档操作库，与 Nodejs 的 cheerio 类似，它的 API 风格与 jQuery 保持一致。

## 如何开始

第一步：加入 visdom 库

```toml
[depedencies]
visdom = "0.0.1"
```

第二步：

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

| 方法                  |             说明             | 备注 |
| :-------------------- | :--------------------------: | ---: |
| find(selector:&str)   |  查找匹配选择器筛的子孙元素  |      |
| filter(selector:&str) |     筛选匹配选择器的元素     |      |
| not(selector:&str)    |     排除匹配选择器的元素     |      |
| is(selector:&str)     | 判断是否所有元素都匹配选择器 |      |
