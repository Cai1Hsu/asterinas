✨亲爱的开发者小伙伴✨，欢迎来到2025年OS大赛特供版星绽仓库。

# 🚀 欢迎登上OS大赛的星绽快车！

你是否憧憬亲手打造一个既酷炫又安全的操作系统？是否想用Rust语言在系统编程的宇宙中留下自己的轨迹？欢迎选择基于星绽参加2025年的OS大赛，使用Rust语言在探索OS领域的星辰大海！

## 🌠 为什么选择星绽？

[星绽](https://github.com/asterinas/asterinas)是一个与Linux兼容、但比Linux更安全可靠的OS内核。目前，星绽支持两个CPU体系架构（amd64和riscv64），实现了190个Linux系统调用，5个文件系统（包括Ext2和extFAT32），3种网络socket类型（包括TCP、UDP、Unix等），常用Virtio驱动，第一方代码（不包括依赖）合计超过10万行。项目的2024年度进展报告[见这里](https://asterinas.github.io/2025/01/20/asterinas-in-2024.html)。

相比其他基于Rust语言的OS内核，星绽具有如下特色：

* **首创[框内核架构](https://asterinas.github.io/book/kernel/the-framekernel-architecture.html)**: 所有unsafe Rust代码被限制在了一个极小的OS开发框架中（名为[OSTD](https://asterinas.github.io/book/ostd/index.html)），绝大多数OS功能使用纯safe Rust开发，让你对代码更加自信。
* **形式化验证加持**：OS开发框架中涉及unsafe的部分关键模块[已被形式化证明](https://asterinas.github.io/2025/02/13/towards-practical-formal-verification-for-a-general-purpose-os-in-rust.html)，内存安全更加有保障。
* **性能比肩Linux**：实现代码经过高度优化，在[LMbench等基准测试上比肩Linux](https://asterinas.github.io/benchmark/)。
* **开箱即用工具箱**：为OS开发者量身定制了开发工具箱（名为[OSDK](https://asterinas.github.io/book/osdk/guide/index.html)），令Rust OS开发像Rust应用开发一样丝滑。

## 💰 咦！参与还有奖？

今年是星绽第一次作为OS大赛的内核赛道的基础OS，而大赛的评测有一些统一要求，目前星绽尚未满足，这意味着每一个选择星绽作为基础OS的参赛小组都有一些共同需要做的事情，比如：

1. 改造Makefile及其他基础设施以满足评测平台的提交要求；
2. 增加LoongArch的支持，包括代码、测试、CI等；
3. 为了运行官方测试集，修复bug或增加功能。

我们欢迎参赛同学们将为了满足以上公共需求的代码提交到本代码仓库（星绽主仓库更佳！）。我们合并代码时会综合考虑（1）时效性（越早提交越好），（2）质量（可读性、干净和可维护），（3）原子性（每个PR或commit只做一件不可细分的事情）。

在初赛阶段结束之前，我们会评出贡献最大的一些同学，并提供一些物质奖励以表彰你们的贡献：
1. 一等奖（1名）：Macbook Air 13寸 M4芯片
2. 二等奖（2名）：iPad Mini A17 Pro
3. 三等奖（3名）：Air Pods 4 主动降噪

（奖励规则后续会细化，奖品也可能会所有调整，最终解释权归星绽社区，敬请关注本仓库后续正式公告！）

在克服了以上公共的基本要求之后，我们期待各位参赛同学自由选择感兴趣的、有挑战的OS功能和特性，把他们加到星绽中。祝大家赛出风格，赛出水平，有所收获，共同进步！

如有任何问题，欢迎在本仓库上提Issue，或者加微信交流（姓名：蚂蚁集团 田洪亮，Github/微信ID: tatetian）。

-----

<p align="center">
    <img src="docs/src/images/logo_en.svg" alt="asterinas-logo" width="620"><br>
    A secure, fast, and general-purpose OS kernel written in Rust and compatible with Linux<br/>
    <a href="https://github.com/asterinas/asterinas/actions/workflows/test_x86.yml"><img src="https://github.com/asterinas/asterinas/actions/workflows/test_x86.yml/badge.svg?event=push" alt="Test x86-64" style="max-width: 100%;"></a>
    <a href="https://github.com/asterinas/asterinas/actions/workflows/test_x86_tdx.yml"><img src="https://github.com/asterinas/asterinas/actions/workflows/test_x86_tdx.yml/badge.svg" alt="Test Intel TDX" style="max-width: 100%;"></a>
    <a href="https://asterinas.github.io/benchmark/"><img src="https://github.com/asterinas/asterinas/actions/workflows/benchmark_x86.yml/badge.svg" alt="Benchmark x86-64" style="max-width: 100%;"></a>
    <br/>
</p>

English | [中文版](README_CN.md) | [日本語](README_JP.md)

## Introducing Asterinas

Asterinas is a _secure_, _fast_, and _general-purpose_ OS kernel
that provides _Linux-compatible_ ABI.
It can serve as a seamless replacement for Linux
while enhancing _memory safety_ and _developer friendliness_.

* Asterinas prioritizes memory safety
by employing Rust as its sole programming language
and limiting the use of _unsafe Rust_
to a clearly defined and minimal Trusted Computing Base (TCB).
This innovative approach,
known as [the framekernel architecture](https://asterinas.github.io/book/kernel/the-framekernel-architecture.html),
establishes Asterinas as a more secure and dependable kernel option.

* Asterinas surpasses Linux in terms of developer friendliness.
It empowers kernel developers to
(1) utilize the more productive Rust programming language,
(2) leverage a purpose-built toolkit called [OSDK](https://asterinas.github.io/book/osdk/guide/index.html) to streamline their workflows,
and (3) choose between releasing their kernel modules as open source
or keeping them proprietary,
thanks to the flexibility offered by [MPL](#License).

While the journey towards a production-grade OS kernel is challenging,
we are steadfastly progressing towards this goal.
Over the course of 2024,
we significantly enhanced Asterinas's maturity,
as detailed in [our end-year report](https://asterinas.github.io/2025/01/20/asterinas-in-2024.html).
In 2025, our primary goal is to make Asterinas production-ready on x86-64 virtual machines
and attract real users!

## Getting Started

Get yourself an x86-64 Linux machine with Docker installed.
Follow the three simple steps below to get Asterinas up and running.

1. Download the latest source code.

```bash
git clone https://github.com/asterinas/asterinas
```

2. Run a Docker container as the development environment.

```bash
docker run -it --privileged --network=host --device=/dev/kvm -v $(pwd)/asterinas:/root/asterinas asterinas/asterinas:0.14.1-20250326
```

3. Inside the container, go to the project folder to build and run Asterinas.

```bash
make build
make run
```

If everything goes well, Asterinas is now up and running inside a VM.

## The Book

See [The Asterinas Book](https://asterinas.github.io/book/) to learn more about the project.

## License

Asterinas's source code and documentation primarily use the 
[Mozilla Public License (MPL), Version 2.0](https://github.com/asterinas/asterinas/blob/main/LICENSE-MPL).
Select components are under more permissive licenses,
detailed [here](https://github.com/asterinas/asterinas/blob/main/.licenserc.yaml). For the rationales behind the choice of MPL, see [here](https://asterinas.github.io/book/index.html#licensing).
