use mesdoc::interface::IDocumentTrait;
use std::error::Error;
use std::thread;
use std::time::SystemTime;
use visdom::Vis;
fn main() -> Result<(), Box<dyn Error>> {
	let html = r##"
  <!DOCTYPE html>
  <html>
  <head>
      <meta charset="UTF-8">
      <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
  
      <meta name="viewport"
            content="width=device-width, initial-scale=1.0, user-scalable=0, minimum-scale=1.0, maximum-scale=1.0">
      <meta name="apple-mobile-web-app-capable" content="yes">
      <meta name="apple-mobile-web-app-status-bar-style" content="black">
  
      <title>
  Rust语言中文社区-首页
  </title>
      <!--    <script src="https://cdn.bootcss.com/jquery/3.2.1/jquery.min.js"></script>-->
      <link rel="stylesheet" type="text/css" href="/css/base.css">
  </head>
  <body>
  <div id="header">
      <div class="header">
      <div class="logo left">
          <a href="/">
      <img class="left" src="/img/rust-logo.svg"/>
      <div class="logo-title left">Rust语言中文社区</div>
      <div style="clear:both;"></div>
          </a>
          <div style="clear:both;"></div>
      </div>
  
      <div class="signpart right">
          <a href="/search">Search</a> &nbsp;
          <a href="/rss">RSS</a> &nbsp;
          <a href="/account">帐户</a>
    </div>
    <div style="clear:both;"></div>
  </div>
  
  </div>
  <div id="content">
      
  <div class="body-content">
      <div class="action_area">
        <a class="right new-article" href="/p/blogarticle/create">写笔记</a>
        <a class="right new-article" href="/p/article/create">发帖子</a>
    <div style="clear:both;"></div>
      </div>
  
      <div class="article-list-section">
    <div class="article-list-head head">
      <span><a href="/latest_articles_paging">最新帖子</a></span>
    </div>
    
    <div class="article-list-container container article-list">
        <ul>
        
      <li>
          <a href="/article?id=bd131160-52ae-4259-b217-f19d56038192" class="title">
        【Rust日报】2021-01-26 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">洛佳</span>
        <span class="timestamp ">2021-01-26 20:29</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=8cc63f42-67e5-4903-b87f-2556d4b43fbb" class="title">
        Fn所有权可以转移多次吗？ -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">zc58778560</span>
        <span class="timestamp ">2021-01-26 17:33</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=2b168232-ccae-46d3-8452-9cd4b5995ac7" class="title">
        【Rust日报】2021-01-25 从头开始学 Rust -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">gensmusic</span>
        <span class="timestamp ">2021-01-25 19:43</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=7d03eb5d-afcd-406e-86fb-fd815b04980c" class="title">
        关于读取 io流 的边界问题 -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">jojo</span>
        <span class="timestamp ">2021-01-25 12:32</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=e4496ec6-cfe5-40aa-b49d-002304061dd2" class="title">
        【Rust日报】2021-01-24 - infoQ 编程语言排行榜投票 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">lidongjies</span>
        <span class="timestamp ">2021-01-24 21:31</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=f43b1eb0-6b3d-456a-b609-b1cdb5be689f" class="title">
        介绍一下自己的PVSS的密码库 -
        大家的项目
          </a>
          <span class="right">
        <span class="author">AlexiaChen</span>
        <span class="timestamp ">2021-01-24 13:41</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=7b6d130e-8c0e-4682-b835-120abf86d189" class="title">
        【Rust日报】2021-01-23 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">binarytom</span>
        <span class="timestamp ">2021-01-23 22:25</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=939d42c8-20b4-4d56-b145-c6b48b09f04f" class="title">
        周期任务lib（delay-timer）v0.2.0 发布 -
        大家的项目
          </a>
          <span class="right">
        <span class="author">槟橙炮炮</span>
        <span class="timestamp ">2021-01-23 17:24</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=26fdf200-407f-45eb-9f6d-0c39112751a1" class="title">
        【Rust日报】2021-01-22  首份Rust月刊杂志邀请大家一起参与 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">Folyd</span>
        <span class="timestamp ">2021-01-22 20:20</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=2cfc851f-8993-4721-a210-15e15c1251dc" class="title">
        【Rust每周一库】num-bigint - 大整数 -
        综合讨论区
          </a>
          <span class="right">
        <span class="author">黑豆腐</span>
        <span class="timestamp ">2021-01-22 13:49</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=0a9b3aa5-6385-4353-b672-4deeb5dc6458" class="title">
        如何实现最接近多进程的异步多线程？ -
        综合讨论区
          </a>
          <span class="right">
        <span class="author">chaoxi24</span>
        <span class="timestamp ">2021-01-22 03:21</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=75a19b34-c8e3-4cb4-8c9f-e5ce27f632a9" class="title">
        关于RwLock并发读性能的疑问 -
        综合讨论区
          </a>
          <span class="right">
        <span class="author">wfxr</span>
        <span class="timestamp ">2021-01-21 23:22</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=54408c14-5d8c-4681-b7c5-2be5b1c03ec9" class="title">
        【Rust日报】 2021-01-21 Rust 的产品实践：1Password -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">Jancd</span>
        <span class="timestamp ">2021-01-21 22:39</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=0351ea25-a61e-4e2c-bfda-6051a346de9f" class="title">
        Rust FFI 编程 - 其它语言调用 Rust 代码 - Python -
        综合讨论区
          </a>
          <span class="right">
        <span class="author">洋芋</span>
        <span class="timestamp ">2021-01-21 18:59</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=fcdc1407-d8ae-4bab-8fad-5967ee24b60f" class="title">
        阿里创业邦-高级区块链工程师（Rust） -
        Rust 招聘
          </a>
          <span class="right">
        <span class="author">ghaniape</span>
        <span class="timestamp ">2021-01-21 09:57</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=34daa627-1edd-4fe8-af02-a91e6d2693a3" class="title">
        【Rust日报】2021-01-20 rust GUI 编程介绍 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">挺肥</span>
        <span class="timestamp ">2021-01-20 23:36</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=3ed39406-418b-454c-98b8-7ee91127aba7" class="title">
        面向对象问题 -
        综合讨论区
          </a>
          <span class="right">
        <span class="author">TideDlrow</span>
        <span class="timestamp ">2021-01-20 14:36</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=a0202e66-7e6c-4bad-9472-25f59304e0dc" class="title">
        大佬,请教一下异步闭包动态分发? -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">WorldLink</span>
        <span class="timestamp ">2021-01-20 10:43</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=9ca6f6f7-e7b3-47c2-8aab-bd89cb679640" class="title">
        【Rust日报】2021-01-19 中国移动云Rust语言新硬件分布式数据库社招 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">洛佳</span>
        <span class="timestamp ">2021-01-19 21:38</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=80987c68-ffb7-4d63-9603-0f0f84c9c42d" class="title">
        关于trait impl的疑问 -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">fengqi2019</span>
        <span class="timestamp ">2021-01-19 14:02</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=3271f5af-7621-4db7-b36b-eff032cd1b9a" class="title">
        Rust 条件编译请教 -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">WingDust</span>
        <span class="timestamp ">2021-01-19 10:19</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=afb817f3-6ac8-4437-b9e5-25a354f48068" class="title">
        【Rust日报】2021-01-18 split_inclusive 特性已经稳定 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">gensmusic</span>
        <span class="timestamp ">2021-01-18 20:19</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=d60e9f66-1243-4eec-ab96-294152fb775e" class="title">
        【Rust日报】 2021-01-17 Rust 要上太空了！ RocketLab 招聘 Rust 工程师 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">whfuyn</span>
        <span class="timestamp ">2021-01-17 23:37</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=84800718-5db1-4bf8-b96d-4624e0fbeabd" class="title">
        Rust 如何简化编译环境 -
        综合讨论区
          </a>
          <span class="right">
        <span class="author">chaoxi24</span>
        <span class="timestamp ">2021-01-17 17:51</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=809e2fe5-7264-4d2f-8a7f-fba57d8e1589" class="title">
        一个关于返回类型的问题 -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">snylonue</span>
        <span class="timestamp ">2021-01-17 17:20</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=2b7eb30b-61ae-4a3d-96fd-fc897ab7b1e0" class="title">
        请教TcpStream处理任意大小的请求的实现方式 -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">jessun2017</span>
        <span class="timestamp ">2021-01-17 11:20</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=dae635f6-0659-4a46-84de-f63408ffd537" class="title">
        我想把它改成 当 x 等于 4 或者 5 或者 6 或 y 等于 false时，输出 yes。该如何改？ -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">Aaron009</span>
        <span class="timestamp ">2021-01-16 21:53</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=f96277e9-5c50-48f8-8e15-fd327a79e55a" class="title">
        【Rust日报】2021-01-16 Async-std v1.9.0 发布 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">洋芋</span>
        <span class="timestamp ">2021-01-16 21:46</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=2772c148-f43d-410b-aa10-1ca7fb911c36" class="title">
        阿里云消息中间件RocketMQ招人 -
        Rust 招聘
          </a>
          <span class="right">
        <span class="author">fuyou001</span>
        <span class="timestamp ">2021-01-15 21:58</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=28f1d055-0fe1-4f6f-b557-65059f95fd27" class="title">
        【Rust日报】2021-01-15 Nightly的Reference已上线Const Generics的文档 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">Folyd</span>
        <span class="timestamp ">2021-01-15 20:04</span>
          </span>
      </li>
        
        </ul>
    </div>
    
    <div style="clear:both;"></div>
      </div>
  
      <div class="article-list-section">
    <div class="article-list-head head">
      <span><a href="/latest_reply_articles_paging">最新回帖</a></span>
    </div>
    
    <div class="article-list-container container article-list">
        <ul>
        
      <li>
          <a href="/article?id=8cc63f42-67e5-4903-b87f-2556d4b43fbb" class="title">
        Fn所有权可以转移多次吗？ -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">zc58778560</span>
        <span class="timestamp ">2021-01-26 17:33</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=75a19b34-c8e3-4cb4-8c9f-e5ce27f632a9" class="title">
        关于RwLock并发读性能的疑问 -
        综合讨论区
          </a>
          <span class="right">
        <span class="author">wfxr</span>
        <span class="timestamp ">2021-01-21 23:22</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=7d03eb5d-afcd-406e-86fb-fd815b04980c" class="title">
        关于读取 io流 的边界问题 -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">jojo</span>
        <span class="timestamp ">2021-01-25 12:32</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=84800718-5db1-4bf8-b96d-4624e0fbeabd" class="title">
        Rust 如何简化编译环境 -
        综合讨论区
          </a>
          <span class="right">
        <span class="author">chaoxi24</span>
        <span class="timestamp ">2021-01-17 17:51</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=7b6d130e-8c0e-4682-b835-120abf86d189" class="title">
        【Rust日报】2021-01-23 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">binarytom</span>
        <span class="timestamp ">2021-01-23 22:25</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=0a9b3aa5-6385-4353-b672-4deeb5dc6458" class="title">
        如何实现最接近多进程的异步多线程？ -
        综合讨论区
          </a>
          <span class="right">
        <span class="author">chaoxi24</span>
        <span class="timestamp ">2021-01-22 03:21</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=9ca6f6f7-e7b3-47c2-8aab-bd89cb679640" class="title">
        【Rust日报】2021-01-19 中国移动云Rust语言新硬件分布式数据库社招 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">洛佳</span>
        <span class="timestamp ">2021-01-19 21:38</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=3ed39406-418b-454c-98b8-7ee91127aba7" class="title">
        面向对象问题 -
        综合讨论区
          </a>
          <span class="right">
        <span class="author">TideDlrow</span>
        <span class="timestamp ">2021-01-20 14:36</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=a0202e66-7e6c-4bad-9472-25f59304e0dc" class="title">
        大佬,请教一下异步闭包动态分发? -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">WorldLink</span>
        <span class="timestamp ">2021-01-20 10:43</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=94c0faf3-32e5-4a7e-9ec2-a94915e87bd9" class="title">
        rust能在运行时动态创建函数吗？ -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">shanqiang0304</span>
        <span class="timestamp ">2020-12-31 10:05</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=80987c68-ffb7-4d63-9603-0f0f84c9c42d" class="title">
        关于trait impl的疑问 -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">fengqi2019</span>
        <span class="timestamp ">2021-01-19 14:02</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=3271f5af-7621-4db7-b36b-eff032cd1b9a" class="title">
        Rust 条件编译请教 -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">WingDust</span>
        <span class="timestamp ">2021-01-19 10:19</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=809e2fe5-7264-4d2f-8a7f-fba57d8e1589" class="title">
        一个关于返回类型的问题 -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">snylonue</span>
        <span class="timestamp ">2021-01-17 17:20</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=2256fd19-016e-4b1a-8c05-f868be7d28ec" class="title">
        熟悉tokio的tracing的帮忙解答个问题 -
        综合讨论区
          </a>
          <span class="right">
        <span class="author">shanliu</span>
        <span class="timestamp ">2021-01-15 19:30</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=2772c148-f43d-410b-aa10-1ca7fb911c36" class="title">
        阿里云消息中间件RocketMQ招人 -
        Rust 招聘
          </a>
          <span class="right">
        <span class="author">fuyou001</span>
        <span class="timestamp ">2021-01-15 21:58</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=dae635f6-0659-4a46-84de-f63408ffd537" class="title">
        我想把它改成 当 x 等于 4 或者 5 或者 6 或 y 等于 false时，输出 yes。该如何改？ -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">Aaron009</span>
        <span class="timestamp ">2021-01-16 21:53</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=d60e9f66-1243-4eec-ab96-294152fb775e" class="title">
        【Rust日报】 2021-01-17 Rust 要上太空了！ RocketLab 招聘 Rust 工程师 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">whfuyn</span>
        <span class="timestamp ">2021-01-17 23:37</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=5cffed24-f5c4-4e15-b942-d232cd713900" class="title">
        关于引用的问题 -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">shanqiang0304</span>
        <span class="timestamp ">2021-01-06 10:41</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=2b7eb30b-61ae-4a3d-96fd-fc897ab7b1e0" class="title">
        请教TcpStream处理任意大小的请求的实现方式 -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">jessun2017</span>
        <span class="timestamp ">2021-01-17 11:20</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=f96277e9-5c50-48f8-8e15-fd327a79e55a" class="title">
        【Rust日报】2021-01-16 Async-std v1.9.0 发布 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">洋芋</span>
        <span class="timestamp ">2021-01-16 21:46</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=1e2ffcdf-33ef-46fd-8d69-1908fcc6be9b" class="title">
        蚂蚁集团招聘 Rust 技术专家 -
        Rust 招聘
          </a>
          <span class="right">
        <span class="author">killme2008</span>
        <span class="timestamp ">2021-01-15 19:29</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=28f1d055-0fe1-4f6f-b557-65059f95fd27" class="title">
        【Rust日报】2021-01-15 Nightly的Reference已上线Const Generics的文档 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">Folyd</span>
        <span class="timestamp ">2021-01-15 20:04</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=202371f2-f43c-41e8-bc26-01aacd5ef7d4" class="title">
        actix如何立即关闭连接？ -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">lithbitren</span>
        <span class="timestamp ">2021-01-06 11:42</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=9f3d4a33-62f3-439b-9736-fdf65c8132e3" class="title">
        基本类型指针和所有权问题问题 -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">shanqiang0304</span>
        <span class="timestamp ">2021-01-04 10:01</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=e09fe057-2caa-483b-b72c-baca380ce9e5" class="title">
        请教一个rust调用C动态库含C结构体的问题 -
        综合讨论区
          </a>
          <span class="right">
        <span class="author">eweca</span>
        <span class="timestamp ">2021-01-15 13:25</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=f17697cb-a8e7-4a0f-b48b-677d5106da91" class="title">
        【招聘】【阿里巴巴】钉钉文档团队招聘！！ -
        Rust 招聘
          </a>
          <span class="right">
        <span class="author">xz313889179</span>
        <span class="timestamp ">2021-01-12 14:20</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=570ab273-382f-4bbe-abd5-3c9828e7ad20" class="title">
        如何在运行时获取一个对象的类型 -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">huanghu578</span>
        <span class="timestamp ">2021-01-13 17:36</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=2f4b3a15-7874-4c68-b9d8-5a717e9a3af0" class="title">
        rust新手的一个问题：fold和scan。 -
        Rust 问答
          </a>
          <span class="right">
        <span class="author">坚果修补匠</span>
        <span class="timestamp ">2021-01-14 14:47</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=58a22139-4cdb-4141-a069-650b4fdc816a" class="title">
        【Rust日报】2021-01-13 -- Open Source Security, Inc.宣布为Rust的GCC前端提供资金 -
        Rust 新闻&#x2F;聚合
          </a>
          <span class="right">
        <span class="author">mook</span>
        <span class="timestamp ">2021-01-13 23:24</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=15fa209e-b2ec-4bcd-bc71-69749ef0eec6" class="title">
        Rust snmp开源项目自愿者招集倡议 -
        大家的项目
          </a>
          <span class="right">
        <span class="author">efancier-cn</span>
        <span class="timestamp ">2021-01-11 23:18</span>
          </span>
      </li>
        
        </ul>
    </div>
    
    <div style="clear:both;"></div>
      </div>
  
      <!-- Member Blog Planet Space -->
      <div class="planet-space-section">
    <div class="planet-space-head head">
      <span><a href="/latest_blog_articles_paging">学习笔记</a></span>
    </div>
    
    <div class="planet-space-container container article-list">
        <ul>
        
      <li>
          <a href="/article?id=a6118e2e-4bd1-4e77-ae73-33bfed1842e1" class="title">weaming：Rust 入门学习路线</a>
          <span class="right">
        <span class="author">weaming</span>
        <span class="timestamp">2021-01-14 11:17</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=738d8a08-28f0-4de6-b9b8-77ab89b98f75" class="title">Neutron3529：发错位置了……</a>
          <span class="right">
        <span class="author">Neutron3529</span>
        <span class="timestamp">2021-01-01 23:20</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=aa7a04c6-8183-42ff-9bdc-5af83164bc8d" class="title">heymind：Rust China Conf 2020 笔记 1</a>
          <span class="right">
        <span class="author">heymind</span>
        <span class="timestamp">2020-12-26 12:38</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=c0f6595e-fdb6-4c9d-b3b6-1ff371742641" class="title">erihsu：Rust Project Hierachy Organization</a>
          <span class="right">
        <span class="author">erihsu</span>
        <span class="timestamp">2020-12-22 20:32</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=875a47c0-c3df-4340-ad9a-5d768758652c" class="title">lithbitren：actix-websocket 使用 protocol的一点个人理解</a>
          <span class="right">
        <span class="author">lithbitren</span>
        <span class="timestamp">2020-12-20 02:33</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=26d7c070-9836-47fd-8e3b-2cc849d612e1" class="title">老牛：[＊＊招聘＊＊] Rust 工程师</a>
          <span class="right">
        <span class="author">老牛</span>
        <span class="timestamp">2020-12-15 10:45</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=c2cd99aa-e27c-4d01-ae13-32f45c07427a" class="title">WorldLink：大佬   请教一下 rust async的问题？</a>
          <span class="right">
        <span class="author">WorldLink</span>
        <span class="timestamp">2020-12-14 11:38</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=5483b5d9-ca6e-4be1-8f07-c5d7a799cad0" class="title">老牛：招 Rust 工程师</a>
          <span class="right">
        <span class="author">老牛</span>
        <span class="timestamp">2020-12-10 15:39</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=62393b63-26c5-4583-a564-4365de7e87b9" class="title">rdigua：About rust</a>
          <span class="right">
        <span class="author">rdigua</span>
        <span class="timestamp">2020-12-05 19:01</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=ea9d3a37-0324-42b6-ac36-0690324d060d" class="title">gotope：bevy</a>
          <span class="right">
        <span class="author">gotope</span>
        <span class="timestamp">2020-12-02 11:01</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=67ee6d46-62c3-4fe6-9f44-f24cf7ec7a7a" class="title">cisen：测试笔记</a>
          <span class="right">
        <span class="author">cisen</span>
        <span class="timestamp">2020-11-30 17:25</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=1574da85-0157-473a-b5dd-69c1998a4477" class="title">lightsing：DERACTED</a>
          <span class="right">
        <span class="author">lightsing</span>
        <span class="timestamp">2020-11-30 10:47</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=2166fb4d-4812-4a7b-b6f2-b5d9e16b1df4" class="title">rdigua：立个标-gemini</a>
          <span class="right">
        <span class="author">rdigua</span>
        <span class="timestamp">2020-11-18 20:36</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=b3c5ced4-9638-405c-8d0f-a4f02ef84541" class="title">黑豆腐：【Rust每周一库】hex - 处理hex数据</a>
          <span class="right">
        <span class="author">黑豆腐</span>
        <span class="timestamp">2020-11-16 23:30</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=bca2d5b3-b186-496b-844e-f013739d4a93" class="title">rdigua：One day once content</a>
          <span class="right">
        <span class="author">rdigua</span>
        <span class="timestamp">2020-11-15 16:40</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=f4847b14-bf2f-446f-89b8-4935a9e67a01" class="title">netcan：C++&#x2F;Rust 元编程之 BrainFuck 编译器（constexpr&#x2F; 过程宏解法）</a>
          <span class="right">
        <span class="author">netcan</span>
        <span class="timestamp">2020-11-07 10:16</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=1464365e-0331-4e43-81d2-992296b83440" class="title">xhh：Hello, world!</a>
          <span class="right">
        <span class="author">xhh</span>
        <span class="timestamp">2020-10-28 10:20</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=f5188fed-9353-43b9-88cd-133878dbb5fb" class="title">黑豆腐：【Rust每周一库】lazy_static - 动态生成静态变量</a>
          <span class="right">
        <span class="author">黑豆腐</span>
        <span class="timestamp">2020-10-27 23:27</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=1fe4f281-fbd2-4337-a098-b1f74ad54a8a" class="title">老牛：纯 Rust 写的私有云</a>
          <span class="right">
        <span class="author">老牛</span>
        <span class="timestamp">2020-10-21 13:14</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=451a657d-f77f-41a7-9f1c-e4535752d4ff" class="title">Ayawen01：</a>
          <span class="right">
        <span class="author">Ayawen01</span>
        <span class="timestamp">2020-10-18 13:05</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=b13c2f1a-d0a6-4715-8022-b6b4492b20f5" class="title">yuemanxilou：[招聘贴]阿里巴巴在线协同文档团队诚招rust工程师，欢迎自荐或者推荐~</a>
          <span class="right">
        <span class="author">yuemanxilou</span>
        <span class="timestamp">2020-10-14 14:18</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=3923f7b6-0603-474f-8342-d0b23b609090" class="title">rdigua：A question about Rustup update</a>
          <span class="right">
        <span class="author">rdigua</span>
        <span class="timestamp">2020-10-10 21:17</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=a5963c4e-787a-4ff0-8126-ace11b6e9694" class="title">netcan：详解函数式编程之 Monad</a>
          <span class="right">
        <span class="author">netcan</span>
        <span class="timestamp">2020-10-03 09:45</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=d8771ab3-3dcd-4b40-869b-6fd47ebd1174" class="title">zhuxiujia：【Rbatis 数据库ORM文档官网上线】</a>
          <span class="right">
        <span class="author">zhuxiujia</span>
        <span class="timestamp">2020-10-02 14:20</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=25af6b36-e684-486e-8556-fb4151d72c44" class="title">Mike Tang：Rust 宇宙中的三个世界</a>
          <span class="right">
        <span class="author">Mike Tang</span>
        <span class="timestamp">2020-09-18 12:06</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=3893e140-54ce-478d-9399-71dca43ecbe7" class="title">hackerchai：[Rust][权限控制][Casbin] Rust 下成熟好用的权限控制库</a>
          <span class="right">
        <span class="author">hackerchai</span>
        <span class="timestamp">2020-09-01 16:50</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=e89fe8c8-77ef-46ea-8864-9b0ea0586fcc" class="title">binarytom：【Rust日报】2020-07-25 - Rust 新闻&#x2F;聚合 </a>
          <span class="right">
        <span class="author">binarytom</span>
        <span class="timestamp">2020-07-25 18:29</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=63583f31-3016-4a39-b737-fdc39cb07551" class="title">binarytom：test</a>
          <span class="right">
        <span class="author">binarytom</span>
        <span class="timestamp">2020-07-25 18:28</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=4a045817-7980-4c90-830b-8a240d4bafcf" class="title">zhuxiujia：持续提高优质rust语言框架</a>
          <span class="right">
        <span class="author">zhuxiujia</span>
        <span class="timestamp">2020-07-13 23:26</span>
          </span>
      </li>
        
      <li>
          <a href="/article?id=073ba1e1-7b84-4b55-a837-1ca94c1c5b15" class="title">rdigua：One day once content</a>
          <span class="right">
        <span class="author">rdigua</span>
        <span class="timestamp">2020-07-10 21:23</span>
          </span>
      </li>
        
        </ul>
    </div>
    
    <div style="clear:both;"></div>
      </div>
  
      <div class="category-list-section">
    <div class="category-space-head head">
      <span>版块分类</span>
    </div>
    
    <div class="category-list-container container">
        <ul>
        
      <li>
          <a href="/section?id=c2511921-51f7-401f-a0c0-d3abcfa0631c">综合讨论区</a>
      </li>
        
      <li>
          <a href="/section?id=498bfc50-3707-406f-b7ca-ede9cbf8808d">Rust 问答</a>
      </li>
        
      <li>
          <a href="/section?id=39bb9ba5-4349-452a-a9e8-c8932170bb34">Rust 活动</a>
      </li>
        
      <li>
          <a href="/section?id=751ee61a-a1d4-4bd1-87ca-4ca455d24a59">异步io相关讨论区</a>
      </li>
        
      <li>
          <a href="/section?id=522c7491-6a5f-4141-a3a1-3070c0466586">Wasm 相关</a>
      </li>
        
      <li>
          <a href="/section?id=f0234fda-972f-4fb9-a5ac-23ade94442ad">论坛相关</a>
      </li>
        
      <li>
          <a href="/section?id=b5da3eae-a44c-44c2-ab34-bf49e290e257">Web 开发框架</a>
      </li>
        
      <li>
          <a href="/section?id=740a83e5-27e3-469b-82b0-a395cbeeb166">Rust 水区</a>
      </li>
        
      <li>
          <a href="/section?id=bfd278ec-7a6d-4eca-919a-38e608551888">Rust 教程</a>
      </li>
        
      <li>
          <a href="/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f">Rust 新闻&#x2F;聚合</a>
      </li>
        
      <li>
          <a href="/section?id=99d337dc-bae2-4a5a-9f4c-302d4b1df8de">Rust 展望</a>
      </li>
        
      <li>
          <a href="/section?id=12987868-705f-4ce2-b158-4d43db7d3a97">区块链相关</a>
      </li>
        
      <li>
          <a href="/section?id=f38f6ee2-9e28-455a-95a4-f959e9efa02d">机器学习&#x2F;人工智能相关</a>
      </li>
        
      <li>
          <a href="/section?id=66600b12-ad19-4d3f-8f9e-47af67e4a18b">大家的项目</a>
      </li>
        
      <li>
          <a href="/section?id=ad7f4769-63b6-4616-a44d-1a6fd60e0a2e">嵌入式&#x2F;IoT&#x2F;物联网</a>
      </li>
        
      <li>
          <a href="/section?id=fed6b7de-0a74-48eb-8988-1978858c9b35">Rust 招聘</a>
      </li>
        
      <li>
          <a href="/section?id=abef1881-1750-4da7-a2e7-71ab8f7e154b">Web和服务端开发</a>
      </li>
        
      <li>
          <a href="/section?id=fadbbc06-bcc4-4823-8fcf-f2d16ea2c543">Rust文章翻译区</a>
      </li>
        
      <li>
          <a href="/section?id=fde4792c-b6f2-4fb7-804b-74c022119d4f">微服务 Service Mesh 相关</a>
      </li>
        
      <li>
          <a href="/section?id=3c8929e5-37f3-44e0-8d30-3c62898c0e50">Rust Web 前端开发</a>
      </li>
        
      <li>
          <a href="/section?id=b52b5f45-38dc-4c3e-8d9d-ac3f72b4ef4d">Rust兼职&#x2F;外包项目发布</a>
      </li>
        
        </ul>
        <div style="clear:both;"></div>
    </div>
    
      </div>
  
      <div class="most-links-section">
    <div class="links-section-head head">
      <span>常用链接</span>
    </div>
    <div class="container">
        <ul>
          <li><a href="https://mp.weixin.qq.com/s/aRGY6oLXVQzxbb1wb3RGcA">Rust 语言新手指南</a></li>
          <li><a href="https://kaisery.github.io/trpl-zh-cn/foreword.html">Rust 程序设计语言（网络中文翻译）</a></li>
          <li><a href="/article?id=ed7c9379-d681-47cb-9532-0db97d883f62">Rust 语言中文社区公众账号</a></li>
          <li><a href="https://rust-lang.org">rust-lang.org</a></li>
          <li><a href="https://this-week-in-rust.org">This Week in Rust</a></li>
          <li><a href="https://doc.rust-lang.org/std/index.html">Rust 标准库 API 文档</a></li>
          <li><a href="https://doc.rust-lang.org/stable/rust-by-example/">Rust By Example</a></li>
          <li><a href="https://cheats.rs">Rust 单页手册</a></li>
          <li><a href="https://rustcc.cn/article?id=471b7ca6-aa2e-4ea5-b692-6757adc4778a">RustChinaConf2020 资料</a></li>
        </ul>
        <div style="clear:both;"></div>
    </div>
      </div>
  </div>
  
  </div>
  <div id="footer">
      <div class="footer">
      <div class="site-desc">
    <p class="links">
      友情链接：
        <a target="_blank" href="http://ipfs.cn/">IPFS中文社区</a>
        | <a target="_blank" href="http://tinylab.org/">泰晓科技</a>
    </p>
    <p class="links">
        <a href="/acknowledgement">鸣谢：</a>
        <a href="/acknowledgement">迅达云</a>
        <a href="/acknowledgement">赛贝</a>
        <a href="/acknowledgement">LongHash</a>
    </p>
        <p> ©2016~2020 Rust.cc 版权所有 &nbsp;&nbsp;
        <span class="powered">Powered by
      <a href="https://github.com/daogangtang/forustm">Forustm</a> &amp;
      <a href="https://github.com/daogangtang/rusoda">Rusoda</a> &amp;
      <a href="https://github.com/sappworks/sapper">Sapper</a>
        </span>
    </p>
      <p>
          <span><a href="https://beian.miit.gov.cn">蜀ICP备20010673号-1</a></span>
      </p>
  
      </div>
  </div>
  
  <script>
  var _hmt = _hmt || [];
  (function() {
   var hm = document.createElement("script");
   hm.src = "https://hm.baidu.com/hm.js?1fd834970f3ad2bab2cb57d4aa2b2e5a";
   var s = document.getElementsByTagName("script")[0]; 
   s.parentNode.insertBefore(hm, s);
   })();
  </script>
  
  </div>
  
  </body>
  </html>
  
  
  "##;
	let root = Vis::load_catch(
		html,
		Box::new(|e| {
			println!("error:{}", e);
		}),
	);
	println!("root:{}", root.length());
	root.find("a,,");
	let lists = root.find(".article-list-container ul > li a.title");
	let titles = lists
		.filter_by(|_, node| node.text().contains("Rust日报"))
		.map(|_, node| String::from(node.text().trim()));
	println!("{:?}", titles);
	let links = root.find("#footer .links").next_all("").has("span");
	println!("{}", links.length());
	let all_titles = root.find("a.title");
	let not_in_container = all_titles.filter(":not(.article-list-container) > ul > li > *");
	println!(
		"all_titles:{}, not_in_container:{}",
		all_titles.length(),
		not_in_container.length()
	);
	let mut texts = root.find("#footer").texts(0).filter_by(|_, text_node| {
		if text_node.text().trim().is_empty() {
			return false;
		}
		true
	});
	texts.for_each(|_, text_node| {
		println!("text_node:{}", text_node.text());
		let chars = text_node.text().chars().collect::<Vec<char>>();
		let total = chars.len();
		if total > 5 {
			let mut mix = String::with_capacity(total);
			let mut cur_index = 0;
			let mut loop_num = 0;
			while cur_index < total {
				let moved = 5;
				let end = cur_index + moved;
				let end = if end > total { total } else { end };
				let cur_chars = chars[cur_index..end].iter().collect::<String>();
				if loop_num % 2 == 0 || cur_chars.trim().is_empty() {
					mix.push_str(&cur_chars);
				} else {
					let wrapped = format!("<span class='a2xgd5o3k'>{}</span>", cur_chars);
					mix.push_str(&wrapped);
				}
				loop_num += 1;
				cur_index += moved;
			}
			text_node.set_html(&mix);
		}
		true
	});
	texts.remove();
	println!("转换后变成：{}", root.find("#footer").outer_html());
	Ok(())
}
