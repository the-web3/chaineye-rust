# Hello Cargo

我最近在重新学rust ，巩固一下细节，也写一个Chaineye rust 教程，供想要学习的小伙伴查看。

推特：@seek_web3

Chainey 社群： 官网 chaineye.info | Chaineye Rust 教程 | 微信: LGZAXE, 加微信之后拉各社群

所有代码和教程开源在github: https://github.com/0xchaineye/chaineye-rust

-----------------------------------------------------------------------------------------------------------------------------------------------------------


Cargo 是 Rust 的构建系统和包管理器。大多数 Rustaceans 使用这个工具来管理他们的 Rust 项目，因为 Cargo 会为您处理很多任务，例如构建代码、下载代码所依赖的库以及构建这些库。（我们称你的代码需要 依赖的库。）

最简单的 Rust 程序，就像我们目前所写的一样，没有任何依赖关系。因此，如果我们构建了“Hello, world!” 与 Cargo 合作的项目，它只会使用 Cargo 中处理构建代码的部分。当您编写更复杂的 Rust 程序时，您将添加依赖项，如果您使用 Cargo 启动项目，添加依赖项将更容易做到。

因为绝大多数 Rust 项目都使用 Cargo，所以本书的其余部分假设你也在使用 Cargo。如果您使用“安装”部分中讨论的官方安装程序，Cargo 会随 Rust 一起 安装。如果您通过其他方式安装了 Rust，请通过在终端中输入以下内容来检查是否安装了 Cargo：

```
$ cargo --version
```

如果你看到一个版本号，你有它！如果您看到错误，例如command not found，请查看您的安装方法的文档以确定如何单独安装 Cargo。

#### 1. 使用 Cargo 创建项目

让我们使用 Cargo 创建一个新项目，看看它与我们原来的“Hello, world!”有何不同。项目。导航回您的项目目录（或您决定存储代码的任何位置）。然后，在任何操作系统上，运行以下命令：

```
$ cargo new hello_cargo
$ cd hello_cargo
```

第一个命令创建了一个名为hello_cargo的新目录。我们将项目命名为hello_cargo，Cargo 在同名目录中创建其文件。

进入hello_cargo目录并列出文件。您会看到 Cargo 为我们生成了两个文件和一个目录：一个Cargo.toml文件和一个 src目录，其中包含一个main.rs文件。

它还初始化了一个新的 Git 存储库以及一个.gitignore文件。cargo new如果您在现有的 Git 存储库中运行，则不会生成 Git 文件；您可以使用cargo new --vcs=git.

```
注意：Git 是一个常见的版本控制系统。您可以使用该标志更改cargo new为使用不同的版本控制系统或不使用版本控制系统。--vcs运行cargo new --help以查看可用选项。
```

在您选择的文本编辑器中打开Cargo.toml 。它应该类似于清单 1-2 中的代码。

文件名：Cargo.toml

```
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

示例1-2：由生成的Cargo.toml的内容cargo new

该文件采用TOML（Tom's Obvious, Minimal Language）格式，这是 Cargo 的配置格式。

第一行[package]是一个节标题，指示以下语句正在配置包。随着我们向此文件添加更多信息，我们将添加其他部分。

接下来的三行设置 Cargo 编译程序所需的配置信息：名称、版本和要使用的 Rust 版本。我们将在附录 E中讨论edition密钥。

最后一行，[dependencies]，是您列出项目的任何依赖项的部分的开始。在 Rust 中，代码包被称为 crates。这个项目我们不需要任何其他 crate，但我们将在第 2 章的第一个项目中使用，因此我们将使用这个依赖项部分。

现在打开src/main.rs看看：

文件名：src/main.rs

```
fn main() {
    println!("Hello, world!");
}
```

Cargo 生成了一个“Hello, world!” 为您准备的程序，就像我们在清单 1-1 中编写的程序一样！到目前为止，我们之前的项目与 Cargo 生成的项目的不同之处在于 Cargo 将代码放在了src 目录中，而我们在顶层目录中有一个Cargo.toml配置文件。

Cargo 希望您的源文件位于src目录中。顶级项目目录仅用于 README 文件、许可证信息、配置文件以及与您的代码无关的任何其他内容。使用 Cargo 可以帮助您组织项目。一切都有一个地方，一切都在它的位置。

如果您启动了一个不使用 Cargo 的项目，就像我们使用“Hello, world!”所做的那样 项目，您可以将其转换为使用 Cargo 的项目。将项目代码移动到src目录并创建一个适当的Cargo.toml 文件。

#### 2. 建立和运行 cargo 项目

现在让我们看看当我们构建和运行“Hello, world!”时有什么不同。与货物计划！在hello_cargo目录中，通过输入以下命令构建项目：

```
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

此命令在target/debug/hello_cargo（或 Windows 上的target\debug\hello_cargo.exe）而不是在您的当前目录中创建一个可执行文件。您可以使用以下命令运行可执行文件：

```
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

如果一切顺利，Hello, world!应该打印到终端。第一次运行cargo build也会导致 Cargo 在顶层创建一个新文件：Cargo.lock。该文件跟踪项目中依赖项的确切版本。这个项目没有依赖，所以文件有点稀疏。您永远不需要手动更改此文件；Cargo 为您管理其内容。

我们刚刚构建了一个项目cargo build并使用 运行它 ./target/debug/hello_cargo，但我们也可以使用它cargo run来编译代码，然后在一个命令中运行生成的可执行文件：

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

请注意，这次我们没有看到表明 Cargo 正在编译的输出 hello_cargo。Cargo 发现文件没有改变，所以它只运行二进制文件。如果您修改了源代码，Cargo 会在运行之前重新构建项目，您会看到以下输出：

```
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo 还提供了一个名为cargo check. 此命令快速检查您的代码以确保它可以编译但不会生成可执行文件：

```
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

为什么你不想要一个可执行文件？通常，cargo check比 快得多 cargo build，因为它跳过了生成可执行文件的步骤。如果您在编写代码时不断检查您的工作，使用cargo check将加快该过程！因此，许多 Rustaceans 在cargo check编写程序时会定期运行以确保它可以编译。然后它们cargo build在准备好使用可执行文件时运行。

让我们回顾一下到目前为止我们对 Cargo 的了解：

- 我们可以使用cargo build.
- 我们可以使用cargo run.
- 我们可以在不生成二进制文件的情况下构建一个项目来使用 cargo check.
- Cargo 没有将构建结果保存在与我们的代码相同的目录中，而是将其存储在target/debug目录中。

使用 Cargo 的另一个优点是，无论您使用哪种操作系统，命令都是相同的。因此，此时，我们将不再提供针对 Linux 和 macOS 与 Windows 的具体说明。

#### 3.为发布而构建

当您的项目最终准备好发布时，您可以使用cargo build --release优化来编译它。此命令将在target/release而不是target/debug中创建可执行文件。优化使您的 Rust 代码运行得更快，但打开它们会延长程序编译所需的时间。这就是为什么有两种不同的配置文件的原因：一个用于开发，当您想要快速且经常重建时，另一个用于构建您将提供给用户的最终程序，该程序不会重复重建并且运行速度与可能的。如果您要对代码的运行时间进行基准测试，请务必使用target/releasecargo build --release中的可执行文件运行和基准测试。

#### 4. Cargo 作为公约

对于简单的项目，Cargo 并没有提供比仅仅使用更多的价值 rustc，但是随着你的程序变得更加复杂，它会证明它的价值。对于由多个 crate 组成的复杂项目，让 Cargo 协调构建要容易得多。

尽管这个hello_cargo项目很简单，但它现在使用了很多你将在 Rust 职业生涯中使用的真正工具。事实上，要处理任何现有项目，您可以使用以下命令使用 Git 签出代码，切换到该项目的目录并构建：

```
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

有关 Cargo 的更多信息，请查看其文档。

#### 5. 概括

您的 Rust 之旅已经有了一个良好的开端！到目前为止，您学习了如何：

- 使用安装最新稳定版本的 Rustrustup
- 更新到较新的 Rust 版本
- 打开本地安装的文档
- 编写并运行“你好，世界！” 程序rustc直接使用
- 使用 Cargo 的约定创建并运行一个新项目
