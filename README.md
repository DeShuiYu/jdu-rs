<center><code>jdu</code>:超高性能统计所有文件总大小工具</center>

***

&emsp;&emsp;`jdu`是一个超高性能计算某一文件夹下所有文件总大小的工具，它比用`du -sh`计算快。尤其在TB数量级下结果更加明显，且支持扩平台。

## 1. 快速开始
### 1.1 安装

```shell
# 下载
> wget https://github.com/DeShuiYu/jdu-rs/releases/download/v0.0.1/jdu-x86_64-unknown-linux-gnu.zip
# 解压
> unzip jdu-x86_64-unknown-linux-gnu.zip
# 重命名
> mv jdu-x86_64-unknown-linux-gnu jdu
```
  
### 1.2 使用
&emsp;&emsp;假设需要计算家目录下文件夹所有文件的总大小，可以采用如下的命令：
```shell
 > ./jdu ~/*
 ```
 &emsp;&emsp;假设需要某文件夹下所有文件的总大小和某文件的大小，可以采用如下的命令：
```shell
> ./jdu 文件夹1 文件夹2 文件3 ...
```

## 2. 测试数据
扫描大约`300G`的数据，统计如下数据，`du -sh`大约是`jdu`的`2~3`倍，因此在`300T`大文件夹下`jdu`比`du`所花费的时间会少很多
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
