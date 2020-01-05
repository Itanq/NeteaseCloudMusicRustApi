# 网易云音乐 API
网易云音乐 Rust API Service

## 实现说明
此实现是为了学习Rust和actix-web框架而写的，是对[Binaryify](https://github.com/Binaryify/NeteaseCloudMusicApi)大神Node.js版的网易云音乐API的Rust实现，
因为本人对actix-web还不是很熟悉，只是在实现之初简单快速过了一遍actix-web的官方文档，加之本人之前从未有过写过web网络相关经历，故此实现还存在很多待优化的地方，整个代码结构亦是如此；
开始写的时候，actix-web好像是刚发布1.0不久，后面因为工作比较忙，写的比较慢，到现在actix-web已经到2.0了（actix-web发展实在太快了），所以后面本人也会持续更新。

目前所有API都已完成，并且使用github上的网易云音乐App的Flutter实现测试其中一部分Api,提供的API接口与原Node.js版的接口保持一致，大概率会一直保持一致。

## 环境要求

需要 Rust1.39+, actix_web1.0

## 安装运行

```shell
$ git clone git@github.com:Itanq/NeteaseCloudMusicRustApi.git
$ cargo run
```
服务器启动默认端口为 8000

## 使用文档
[文档地址](https://binaryify.github.io/NeteaseCloudMusicApi)


## License
[The MIT License (MIT)](https://github.com/Itanq/NeteaseCloudMusicRustApi/blob/master/LICENSE)

