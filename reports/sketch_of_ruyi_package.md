# Sketch of Ruyi Package and Build System

## openRuyi 的定位

以我目前的理解，openRuyi 主要有两方面作用：

1. 作为 RISC-V 硬件开发的基准软件平台。
2. 作为以回馈 Linux 生态上游（RISC-V 为主）的软件开发平台。

### 作为基准软件平台

这一点可以参考 Intel 曾维护的 Clear Linux.

先来看 Clear Linux 为了这个定位做了什么。

#### Clear Linux 的特点

Clear Linux 的特点很显著，几乎不同于任何一个发行版。

根据我的调研（Clear Linux 停止维护了，资料搜集有点难度，可能不太全面），有：

用户角度：

1. bundle 的依赖模式。通过 bundle 来为用户提供一个开箱即用的体验，一定程度上也确保了软件体验的稳定性。
2. 使用 swupd 进行包的管理。系统全局维护一个唯一的版本号，任何 bundle 的版本号与其对齐。
3. ostree 底层。系统更新和维护是原子的，需要重启。
4. 默认的激进优化。软件包默认开启很多优化选项，有利有弊。

维护者角度：

1. bundle 的优缺点。关注点从复杂的依赖关系移开（优点），但是需要维护额外的 bundle 文件。
2. autospec 自动打包工具。自动地通过文件和日志分析，完善一份 RPM Spec 文件，进而节省 RPM 层的人力。
3. 统一的发行节奏。全局版本号滚动发行，版本对齐。

开发者的角度：

1. 进行补丁比传统模式难很多。
2. bundle 的粒度可能有点大。

总结一下的话，Clear Linux 通过自动化机制节省了和 RPM 搏斗的人力，但部分牺牲了灵活性。同时以滚动但节奏稳定的发行模式提供了完整的基准体验。

### 作为基础软件开发平台

这一点没什么特点显著的发行版，几乎所有传统发行版都算得上这个，但是我们可以总结一下需要什么，以及有什么能改进的。

主要是从包管理和构建的角度，毕竟不看包管理的话几乎所有发行版都能 dirty hack.

作为以前沿版本为主的滚动开发平台，在包的元数据上，我们需要能明确地标注出上游（VCS）的能力。

在包的构建过程上，需要能方便地打补丁，能在构建的过程中加入类似断点的功能，允许手工调试等。

这里借鉴 [Wolfi OS](https://github.com/wolfi-dev) 的 [melange](https://github.com/chainguard-dev/melange) 构建系统，它利用容器化技术，提供了类似 GitHub Actions workflow 的 YAML 包声明语法和分层缓存能力。

e.g. [kargo.yaml](https://github.com/wolfi-dev/os/blob/main/kargo.yaml)

melange 甚至提供了 update 模块，做到了自动检测与更新。

## Ruyi Package and Ruyi Build System

### Ruyi Package

具体格式待定 / KCL

#### 元数据

```python
[ "Name", "Version", "Upstream", ... ]
```

#### 构建过程表示

类似 justfile 的设计，每个过程块（justfile 中为一行或一个 shebang 开头的脚本文件）对应构建过程的一层。

提供过程块级别的控制流等语法。

### Ruyi Bundle

#### 元数据

```python
[ "Name", "版本基线", "Maintainer"
, "兼容的父全局基线", "内容的 Ruyi Package"
, ...
]
```

和 Ruyi Package 相比需要 Maintainer 是考虑到 Bundle 作为直接面向用户的界面，需要一个直接反馈的通道。

### Ruyi Build System

通过容器技术提供一个分层的构建器，通过逐层地（遵循控制流）执行过程块完成构建目的。

### Tool and Support

#### Basic service and Tool

构建器作为一个服务部署

操作前端（根据目前的想法）是一个构建的日志（或标准流）的显示面板，同时允许在某个代码块执行后暂停，接入手动调试。

#### Automation

分层的构建模式为自动化提供了良好的基础，允许按层进行的通过 **经典算法** 或 **机器学习算法** 的自动打包。

### Release Mode and User Experience

（待定）

通过原包和 Bundle 两种模式进行软件包的发布，原包的安装需要在用户本机自定义一个 Bundle 文件，并在本地构建这个 Bundle。

采用发布基线模式进行发布，但是由于 openRuyi 同时作为基础软件开发平台，需要拆分基线粒度。我们发布一个全局基线，之后每个包通过声明父基线和兼容的父基线来和这个其父基线对齐，最终结果就是所有 Bundle 都和全局基线对齐。
