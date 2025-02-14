# `jdu`: 高性能统计文件总大小工具

`jdu`能高性能计算某文件夹下所有文件占用磁盘的总大小，它比用`du -sh`计算快。

## 1. 使用方法
* `jdu ~/*`

* `jdu 文件夹1 文件夹2 文件3...`

## 2. 测试数据
扫描大约300G的数据，统计如下数据，`jdu`大约是`du -sh`的`2~3`倍，因此在`300T`大文件夹下`jdu`比`du`所花费的时间会少很多
* `du -sh  ~/*`
```markdown
Benchmark 1: du -sh  ~/*
  Time (mean ± σ):      8.180 s ±  0.104 s    [User: 0.096 s, System: 2.685 s]
  Range (min … max):    8.075 s …  8.349 s    6 runs
```

* `jdu  ~/*`
```markdown
Benchmark 1: jdu  ~/*
  Time (mean ± σ):      3.524 s ±  0.027 s    [User: 1.110 s, System: 18.470 s]
  Range (min … max):    3.492 s …  3.568 s    6 runs
```
