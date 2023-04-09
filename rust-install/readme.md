让我们开始你的 Rust 之旅吧！有很多东西要学，但每一次旅程都从某个地方开始。在本章中，我们将讨论：

- 在 Linux、macOS 和 Windows 上安装 Rust
- 编写打印程序Hello, world!
- 使用cargoRust 的包管理器和构建系统

### 1. Rust 安装 

第一步是安装 Rust。我们将通过 下载 Rust rustup，这是一个用于管理 Rust 版本和相关工具的命令行工具。您需要互联网连接才能下载。

注意：如果您出于某种原因不想使用rustup，请参阅 [其他 Rust 安装方法](https://forge.rust-lang.org/infra/other-installation-methods.html)页面以获取更多选项。

以下步骤安装最新稳定版本的 Rust 编译器。Rust 的稳定性保证确保本书中所有编译的示例将继续使用更新的 Rust 版本进行编译。不同版本的输出可能略有不同，因为 Rust 经常改进错误消息和警告。换句话说，您使用这些步骤安装的任何更新、稳定的 Rust 版本都应该与本书的内容一样工作。

#### 命令行符号
在本章和整本书中，我们将展示一些在终端中使用的命令。您应该在终端中输入的行都以$. 您无需输入$字符；它指示每个命令的开始。不以开头的行$通常显示上一个命令的输出。此外，特定于 PowerShell 的示例将使用> 而不是$.

#### 1. rustup在 Linux 或 macOS 上安装

如果您使用的是 Linux 或 macOS，请打开终端并输入以下命令：

```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

该命令会下载一个脚本并开始安装该rustup 工具，该工具会安装最新的稳定版 Rust。系统可能会提示您输入密码。如果安装成功，将出现以下行：

```
Rust is installed now. Great!
```

您还需要一个链接器，它是 Rust 用来将其编译输出连接到一个文件中的程序。您很可能已经拥有一个。如果你得到链接器错误，你应该安装一个 C 编译器，它通常包含一个链接器。AC 编译器也很有用，因为一些常见的 Rust 包依赖于 C 代码并且需要 C 编译器。

在 macOS 上，您可以通过运行以下命令获得 C 编译器：

```
$ xcode-select --install
```

Linux 用户通常应根据其发行版的文档安装 GCC 或 Clang。例如，如果您使用 Ubuntu，则可以安装该build-essential软件包。

#### 2. rustup在 Windows 上安装

在 Windows 上，转到https://www.rust-lang.org/tools/install并按照安装 Rust 的说明进行操作。在安装的某个时刻，您将收到一条消息，说明您还需要适用于 Visual Studio 2013 或更高版本的 C++ 构建工具。获取构建工具的最简单方法是安装Build Tools for Visual Studio 2019。当被问及要安装哪些工作负载时，请确保选择了“C++ 构建工具”并且包含了 Windows 10 SDK 和英语语言包组件。

本书的其余部分使用可在cmd.exe和 PowerShell 中运行的命令。如果存在特定差异，我们将解释使用哪个。

### 二. 更新和卸载

通过 安装 Rust 后rustup，更新到最新版本很容易。在您的 shell 中，运行以下更新脚本：

```
$ rustup update
```
要卸载 Rust 和rustup，请从您的 shell 运行以下卸载脚本：

```
$ rustup self uninstall
```

### 三. 故障排除

要检查是否正确安装了 Rust，请打开 shell 并输入以下行：
```
$ rustc --version
```
您应该看到已发布的最新稳定版本的版本号、提交哈希和提交日期，格式如下：
```
rustc x.y.z (abcabcabc yyyy-mm-dd)
```
如果你看到这个信息，你已经成功安装了 Rust！如果您没有看到此信息并且您使用的是 Windows，请检查 Rust 是否在您的%PATH% 系统变量中。如果这一切都正确并且 Rust 仍然无法正常工作，那么您可以在很多地方获得帮助。最简单的是官方 Rust Discord上的 #beginners 频道​​。在那里，您可以与其他可以帮助您的 Rustaceans（我们称自己为愚蠢的昵称）聊天。其他重要资源包括用户论坛和Stack Overflow。


### 四. 本地文档

Rust 的安装还包括本地文档的副本，因此您可以离线阅读。运行rustup doc以在浏览器中打开本地文档。

任何时候标准库提供了一个类型或函数，而您不确定它的作用或如何使用它，请使用应用程序编程接口 (API) 文档找出答案！
