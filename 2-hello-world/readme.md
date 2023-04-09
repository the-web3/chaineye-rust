# Hello World

我最近在重新学rust ，巩固一下细节，也写一个Chaineye rust 教程，供想要学习的小伙伴查看。

推特：@seek_web3

Chainey 社群： 官网 chaineye.info | Chaineye Rust 教程 | 微信: LGZAXE, 加微信之后拉各社群 

所有代码和教程开源在github: https://github.com/0xchaineye/chaineye-rust

----------------------------------------------------------------------------------------------------------------------------------------------------------


## Hello World

现在你已经安装了 Rust，让我们编写你的第一个 Rust 程序。在学习一门新语言时，传统的做法是编写一个将文本打印到屏幕上的小程序Hello, world!，所以我们将在这里做同样的事情！

注意：本书假定您基本熟悉命令行。Rust 对您的编辑或工具或代码所在的位置没有具体要求，因此如果您更喜欢使用集成开发环境 (IDE) 而不是命令行，请随意使用您最喜欢的 IDE。现在许多 IDE 都有一定程度的 Rust 支持；查看 IDE 的文档以获取详细信息。最近，Rust 团队一直专注于实现出色的 IDE 支持，并且在这方面取得了迅速的进展！

#### 1. 创建项目目录
您将首先创建一个目录来存储您的 Rust 代码。对 Rust 来说，你的代码在哪里并不重要，但是对于本书中的练习和项目，我们建议在你的主目录中创建一个项目目录并将所有项目保存在那里。

打开终端并输入以下命令以创建项目目录和“Hello, world!”目录 项目目录中的项目。

对于 Windows 上的 Linux、macOS 和 PowerShell，请输入：

```
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

对于 Windows CMD，输入：

```
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

#### 2. 编写和运行 Rust 程序

接下来，创建一个新的源文件并将其命名为main.rs。Rust 文件总是以.rs扩展名结尾。如果您在文件名中使用多个单词，请使用下划线分隔它们。例如，使用hello_world.rs而不是 helloworld.rs。

现在打开您刚刚创建的main.rs文件并输入清单 1-1 中的代码。

文件名：main.rs

```
fn main() {
    println!("Hello, world!");
}
```

示例 1-1：打印的程序Hello, world!

保存文件并返回终端窗口。在 Linux 或 macOS 上，输入以下命令来编译和运行文件：

```
$ rustc main.rs
$ ./main
Hello, world!
```
在 Windows 上，输入命​​令.\main.exe而不是./main：

```
> rustc main.rs
> .\main.exe
Hello, world!
```

无论您的操作系统是什么，字符串Hello, world!都应该打印到终端。如果您没有看到此输出，请参阅 安装部分的“疑难解答”部分以获得帮助。

如果Hello, world!打印出来了，恭喜！你已经正式编写了一个 Rust 程序。这让你成为 Rust 程序员——欢迎！

#### 3. Rust 程序剖析

让我们详细回顾一下刚刚在您的“Hello, world!”中发生的事情。程序。这是拼图的第一部分：

```
fn main() {
}
```
这些行在 Rust 中定义了一个函数。这个main函数很特别：它总是在每个可执行的 Rust 程序中运行的第一个代码。第一行声明了一个名为的函数main，它没有参数并且不返回任何内容。如果有参数，它们将放在括号内，().

另外，请注意，函数体用大括号括起来，{}. Rust 需要这些围绕所有函数体。将左大括号与函数声明放在同一行是一种很好的风格，在两者之间添加一个空格。

如果你想在 Rust 项目中坚持标准风格，你可以使用一个名为的自动格式化工具rustfmt来格式化你的代码为特定风格。Rust 团队已将此工具包含在标准的 Rust 发行版中，例如rustc，因此它应该已经安装在您的计算机上！查看在线文档以获取更多详细信息。

函数内部main是以下代码：

```
println!("Hello, world!");
```

这一行完成了这个小程序中的所有工作：它将文本打印到屏幕上。这里有四个重要的细节需要注意。

首先，Rust 风格是缩进四个空格，而不是制表符。

其次，println!调用 Rust 宏。如果它改为调用一个函数，它将被输入为println（不带!）。我们将在第 19 章更详细地讨论 Rust 宏。现在，您只需要知道使用 a! 意味着您正在调用宏而不是普通函数，并且宏并不总是遵循与职能。

第三，你看到了"Hello, world!"字符串。我们将此字符串作为参数传递给println!，然后将字符串打印到屏幕上。

第四，我们用分号 ( ;) 结束这一行，这表示这个表达式已经结束，下一个表达式已准备好开始。大多数 Rust 代码行都以分号结尾。

#### 3. 编译和运行是分开的步骤

您刚刚运行了一个新创建的程序，因此让我们检查该过程中的每个步骤。

在运行 Rust 程序之前，您必须使用 Rust 编译器通过输入rustc命令并将源文件的名称传递给它来编译它，如下所示：

```
$ rustc main.rs
```

如果您有 C 或 C++ 背景，您会注意到这类似于gcc or clang。编译成功后，Rust 会输出一个二进制可执行文件。

在 Linux、macOS 和 Windows 上的 PowerShell 上，您可以通过ls在 shell 中输入命令来查看可执行文件。在 Linux 和 macOS 上，您将看到两个文件。使用 Windows 上的 PowerShell，您将看到与使用 CMD 相同的三个文件。

```
$ ls
main  main.rs
```

在 Windows 上使用 CMD，您将输入以下内容：

```
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

这显示了带有.rs扩展名的源代码文件、可执行文件（在 Windows 上为main.exe，但在所有其他平台上为main），以及在使用 Windows 时，包含带有.pdb扩展名的调试信息的文件。从这里，您运行main或main.exe文件，如下所示：

```
$ ./main # or .\main.exe on Windows
```

如果main.rs是你的“你好，世界！” 程序，此行将打印Hello, world!到您的终端。

如果您更熟悉动态语言，例如 Ruby、Python 或 JavaScript，您可能不习惯将程序作为单独的步骤进行编译和运行。Rust 是一种提前编译的语言，这意味着您可以编译程序并将可执行文件提供给其他人，即使没有安装 Rust，他们也可以运行它。如果您给某人一个.rb、.py或 .js文件，他们需要（分别）安装一个 Ruby、Python 或 JavaScript 实现。但是在那些语言中，您只需要一个命令来编译和运行您的程序。一切都是语言设计的权衡。

对于简单的程序来说，只用编译rustc就可以了，但是随着项目的增长，您将需要管理所有选项并轻松共享您的代码。接下来，我们将向您介绍 Cargo 工具，它将帮助您编写真实的 Rust 程序。
