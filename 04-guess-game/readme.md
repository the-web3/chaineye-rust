# 编写猜字游戏

我最近在重新学rust ，巩固一下细节，也写一个Chaineye rust 教程，供想要学习的小伙伴查看。

推特：@seek_web3

Chainey 社群： 官网 chaineye.info | Chaineye Rust 教程 | 微信: LGZAXE, 加微信之后拉各社群

所有代码和教程开源在github: https://github.com/0xchaineye/chaineye-rust

-----------------------------------------------------------------------------------------------------------------------------------------------------------


让我们一起完成一个动手项目，进入 Rust！本章通过向您展示如何在实际程序中使用它们来向您介绍一些常见的 Rust 概念。您将了解let, match, 方法、相关函数、使用外部 crate 等等！在接下来的章节中，我们将更详细地探讨这些想法。在本章中，您将练习基础知识。

我们将实现一个经典的初学者编程问题：猜谜游戏。它是这样工作的：程序将生成一个介于 1 和 100 之间的随机整数。然后它会提示玩家输入猜测。输入猜测值后，程序将指示猜测值是过低还是过高。如果猜对了，游戏将打印祝贺信息并退出。

## 1.设置新项目

要建立一个新项目，请转到您在第 1 章中创建的项目目录并使用 Cargo 创建一个新项目，如下所示：

```
$ cargo new guessing_game
$ cd guessing_game
```

第一个命令cargo new将项目名称 ( guessing_game) 作为第一个参数。第二个命令更改为新项目的目录。

查看生成的Cargo.toml文件：

文件名：Cargo.toml

```
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

正如您在第 1 章中看到的，cargo new生成一个“Hello, world!” 程序为您服务。查看src/main.rs文件：

文件名：src/main.rs

```
fn main() {
    println!("Hello, world!");
}
```

现在让我们编译这个“Hello, world!” cargo run程序并使用以下命令在同一步骤中运行它：

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
Hello, world!
```

当您需要快速迭代项目时，该run命令会派上用场，就像我们将在本游戏中所做的那样，在进行下一个迭代之前快速测试每个迭代。

重新打开src/main.rs文件。您将在此文件中编写所有代码。

## 2.处理猜测

猜谜游戏程序的第一部分将要求用户输入，处理该输入，并检查输入是否符合预期形式。首先，我们将允许玩家输入猜测。在src/main.rs中输入清单 2-1 中的代码 。

```
fn main() {
    use std::io;
    fn main() {
        println!("Guess the number!");
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);
    }
}
```
示例 2-1：获取用户猜测并打印出来的代码

这段代码包含很多信息，所以让我们逐行查看。要获取用户输入然后将结果打印为输出，我们需要将 io输入/输出库纳入范围。该io库来自标准库，称为std：

```
use std::io;
```

默认情况下，Rust 在标准库中定义了一些项目，并将其引入每个程序的范围。这组称为前奏，您可以在标准库文档中看到其中的所有内容。

use如果要使用的类型不在前奏中，则必须使用语句显式将该类型带入范围。使用该std::io库为您提供了许多有用的功能，包括接受用户输入的能力。

正如您在第 1 章中看到的，main函数是程序的入口点：

```
fn main() {
```

fn语法声明了一个新函数，括号 ,表示()没有参数，大括号{, 开始函数体。

正如你在第 1 章中学到的，println!是一个将字符串打印到屏幕的宏：

```
println!("Guess the number!");
println!("Please input your guess.");
```

此代码正在打印一个提示，说明游戏是什么并请求用户输入。

## 3. 用变量存储值

接下来，我们将创建一个变量来存储用户输入，如下所示：

```
 let mut guess = String::new();
```

现在节目越来越有趣了！这条小线发生了很多事情。我们使用let语句来创建变量。这是另一个例子：

```
let apples = 5;
```

这一行创建了一个名为的新变量apples并将其绑定到值 5。在 Rust 中，变量默认是不可变的。我们将在第 3 章的“变量和可变性”部分详细讨论这个概念。为了使变量可变，我们mut在变量名之前添加：

```
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

注意：//语法开始一个注释，一直持续到行尾。Rust 会忽略注释中的所有内容。我们将在第 3 章更详细地讨论注释。

回到猜谜游戏程序，您现在知道它let mut guess会引入一个名为 的可变变量guess。等号 ( =) 告诉 Rust 我们现在想将一些东西绑定到变量上。等号右侧guess是绑定的值，它是调用的结果 String::new，该函数返回 a 的新实例String。 String是标准库提供的字符串类型，是可增长的 UTF-8 编码的文本位。

该行中的::语法::new表明这new是该String类型的关联函数。关联函数是在类型上实现的函数，在本例中为String. 此new函数创建一个新的空字符串。您会new在许多类型上找到一个函数，因为它是产生某种新值的函数的通用名称。

完整地说，该let mut guess = String::new();行创建了一个可变变量，该变量当前绑定到一个新的空实例 a String。哇！

## 4.接收用户输入

use std::io;回想一下，我们在程序的第一行中包含了标准库中的输入/输出功能。现在我们将从模块中调用该stdin函数io，这将允许我们处理用户输入：

```
    io::stdin()
        .read_line(&mut guess)
```

如果我们没有在程序开头导入io库use std::io，我们仍然可以通过将此函数调用编写为 std::io::stdin. 该stdin函数返回 的实例 std::io::Stdin，该实例表示终端标准输入句柄的类型。

接下来，该行.read_line(&mut guess)调用read_line标准输入句柄上的方法来获取用户的输入。我们还将&mut guess作为参数传递read_line给它以告诉它存储用户输入的字符串。完整的工作read_line是将用户输入的任何内容放入标准输入并将其附加到字符串中（不覆盖其内容），所以我们因此将该字符串作为参数传递。字符串参数需要是可变的，以便该方法可以更改字符串的内容。

&表示此参数是一个引用，它为您提供了一种方法，让您的代码的多个部分访问一个数据，而无需多次将该数据复制到内存中。引用是一个复杂的特性，Rust 的主要优势之一是使用引用是多么安全和容易。你不需要知道很多细节来完成这个程序。现在，您需要知道的是，与变量一样，引用在默认情况下是不可变的。因此，您需要编写&mut guess而不是 &guess使其可变。（第 4 章将更彻底地解释参考资料。）

## 5.Result使用类型处理潜在故障

我们仍在处理这行代码。尽管我们现在讨论的是第三行文本，但它仍然是单个逻辑代码行的一部分。下一部分是这个方法：
```
.expect("Failed to read line");
```
我们可以把这段代码写成：

```
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

但是，一长行很难阅读，因此最好将其分开。当您使用.method_name()语法调用方法时，引入换行符和其他空格以帮助分解长行通常是明智的。现在让我们讨论一下这条线的作用。

如前所述，read_line将用户输入的任何内容放入我们传递给它的字符串中，但它也返回一个值——在本例中为 io::Result. Rust 在其标准库中命名了许多类型 Result：子模块的通用Result 版本和特定版本，例如io::Result. 这些Result 类型是枚举，通常称为枚举，它可以具有一组固定的可能性，称为变体。枚举通常与 一起使用match，这是一种条件，可以方便地根据计算条件时枚举值的变体来执行不同的代码。

第 6 章将更详细地介绍枚举。这些Result类型的目的是对错误处理信息进行编码。

Result的变体是Ok或Err。变体Ok表示操作成功，里面Ok是成功生成的值。Err 变体表示操作失败，并Err包含有关操作失败的方式或原因的信息。

Result与任何类型的值一样，该类型的值具有在其上定义的方法。的实例io::Result具有您可以调用的expect方法。如果此实例io::Result是一个Err值， expect将导致程序崩溃并显示您作为参数传递给 的消息expect。如果该read_line方法返回一个Err，它可能是来自底层操作系统的错误的结果。如果此实例io::Result是一个Ok值，expect则将获取Ok所持有的返回值并将该值返回给您，以便您可以使用它。在这种情况下，该值是用户输入中的字节数。

如果您不调用expect，程序将编译，但您会收到警告：

```
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Rust 警告你没有使用Result返回的值read_line，表明程序没有处理可能的错误。

抑制警告的正确方法是实际编写错误处理，但在我们的例子中，我们只想在出现问题时让这个程序崩溃，所以我们可以使用expect. 您将在第 9 章了解如何从错误中恢复。

## 6.println!使用占位符打印值

除了右大括号之外，到目前为止，代码中只有一行要讨论：

```
println!("You guessed: {}", guess);
```

此行打印现在包含用户输入的字符串。{}花括号的集合是一个占位符：可以将其想象{}成一个固定值的小螃蟹钳。您可以使用花括号打印多个值：第一组花括号保存格式字符串后列出的第一个值，第二组保存第二个值，依此类推。在一次调用中打印多个值println!如下所示：

```
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

此代码将打印x = 5 and y = 10.

## 7. 测试第一部分

让我们测试猜谜游戏的第一部分。运行它使用cargo run：

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
```
```
6
You guessed: 6
```
至此，游戏的第一部分就完成了：我们从键盘获取输入，然后打印出来。

## 8. 生成秘密数字

接下来，我们需要生成一个用户将尝试猜测的秘密数字。密码每次都应该不同，这样游戏就可以玩不止一次。我们将使用 1 到 100 之间的随机数，这样游戏不会太难。Rust 的标准库中还没有包含随机数功能。然而，Rust 团队确实提供了一个具有上述功能的randcrate 。

### 8.1. 使用 crate 获得更多功能

请记住，一个 crate 是 Rust 源代码文件的集合。我们一直在构建的项目是一个二进制 crate，它是一个可执行文件。rand crate 是一个库 crate ，其中包含旨在用于其他程序的代码，并且不能单独执行。

Cargo 对外部 crate 的协调是 Cargo 真正发光的地方。在我们编写使用 的代码之前rand，我们需要修改Cargo.toml文件以包含randcrate 作为依赖项。现在打开该文件并将以下行添加到[dependencies]Cargo 为您创建的部分标题下方的底部。请务必rand完全按照我们在此处指定的版本号使用此版本号，否则本教程中的代码示例可能无法正常工作。文件名：Cargo.toml

文件名：Cargo.toml

```
rand = "0.8.3"
```

在Cargo.toml文件中，标题后面的所有内容都是该部分的一部分，该部分一直持续到另一个部分开始。在[dependencies]您告诉 Cargo 您的项目依赖于哪些外部 crates 以及您需要这些 crates 的哪些版本。在这种情况下，我们使用rand语义版本说明符来指定 crate 0.8.3。Cargo 了解语义版本控制（有时称为SemVer），这是编写版本号的标准。该数字0.8.3实际上是 的简写^0.8.3，这意味着至少0.8.3但低于 的任何版本0.9.0。Cargo 认为这些版本具有与版本兼容的公共 API0.8.3，并且此规范确保您将获得仍然可以与本章中的代码一起编译的最新补丁版本。不保证任何版本 0.9.0或更高版本都具有与以下示例使用的相同的 API。

现在，在不更改任何代码的情况下，让我们构建项目，如清单 2-2 所示。

```
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.3
  Downloaded libc v0.2.86
  Downloaded getrandom v0.2.2
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.10
  Downloaded rand_chacha v0.3.0
  Downloaded rand_core v0.6.2
   Compiling rand_core v0.6.2
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```
清单 2-2：cargo build添加 rand crate 作为依赖项后运行的输出

您可能会看到不同的版本号（但它们都将与代码兼容，感谢 SemVer！），不同的行（取决于操作系统），并且行的顺序可能不同。

当我们包含一个外部依赖项时，Cargo 会从注册表中获取依赖项所需的所有内容的最新版本，该注册表是来自Crates.io的数据副本。Crates.io 是 Rust 生态系统中的人们发布他们的开源 Rust 项目供其他人使用的地方。

更新注册表后，Cargo 会检查该[dependencies]部分并下载列出的任何尚未下载的 crate。在这种情况下，虽然我们只列出rand了一个依赖项，但 Cargo 还抓取了其他rand依赖于工作的 crate。下载 crates 后，Rust 编译它们，然后使用可用的依赖项编译项目。

如果您立即cargo build再次运行而不进行任何更改，则除了该Finished行之外，您将不会得到任何输出。Cargo 知道它已经下载并编译了依赖项，并且您没有在Cargo.toml文件中更改任何有关它们的内容。Cargo 也知道你没有改变你的代码，所以它也不会重新编译。无事可做，它只是退出。

如果你打开src/main.rs文件，做一个小改动，然后保存并再次构建，你只会看到两行输出：

```
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

这些行显示 Cargo 仅通过您对src/main.rs文件的微小更改来更新构建 。你的依赖没有改变，所以 Cargo 知道它可以重用它已经下载和编译的东西。

### 8.2. 使用Cargo.lock文件确保可重现的构建
Cargo 有一种机制，可确保您每次或其他任何人构建代码时都可以重建相同的工件：Cargo 将仅使用您指定的依赖项的版本，直到您另有说明。例如，假设下周版本 0.8.4 的randcrate 发布，并且该版本包含一个重要的错误修复，但它也包含一个会破坏您的代码的回归。为了处理这个问题，Rust 在你第一次运行时创建了Cargo.lockcargo build文件，所以我们现在在guessing_game 目录中有这个文件 。

当您第一次构建项目时，Cargo 会找出符合条件的所有依赖项版本，然后将它们写入Cargo.lock文件。当您将来构建项目时，Cargo 将看到Cargo.lock文件存在并使用其中指定的版本，而不是再次执行所有确定版本的工作。这使您可以自动进行可重现的构建。换句话说，0.8.3由于Cargo.lock 文件，您的项目将一直保持在您明确升级之前。

## 8.3. 更新板条箱以获取新版本
当你确实想更新一个 crate 时，Cargo 会提供 command update，它会忽略Cargo.lock文件并在Cargo.toml中找出符合你规格的所有最新版本。Cargo 会将这些版本写入Cargo.lock文件。否则，默认情况下，Cargo 只会查找大于0.8.3和小于的版本0.9.0。如果randcrate 已经发布了两个新版本0.8.4，并且0.9.0如果您运行，您将看到以下内容cargo update：

```
$ cargo update
    Updating crates.io index
    Updating rand v0.8.3 -> v0.8.4
```

货物忽略0.9.0释放。此时，您还会注意到Cargo.lock文件中的更改，指出rand您现在使用的 crate 版本是0.8.4. 要使用rand版本0.9.0或0.9.x系列中的任何版本，您必须将Cargo.toml文件更新为如下所示：

```
[dependencies]
rand = "0.9.0"
```
下次运行时cargo build，Cargo 将更新可用 crate 的注册表，并rand根据您指定的新版本重新评估您的要求。

关于Cargo及其生态系统还有很多话要说，我们将在第 14 章讨论，但现在，你只需要知道这些。Cargo 使得重用库变得非常容易，因此 Rustaceans 能够编写由多个包组装而成的较小项目。

## 9. 生成随机数

让我们开始使用rand来生成一个数字来猜测。下一步是更新src/main.rs，如清单 2-3 所示。

文件名：src/main.rs

```
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

示例 2-3：添加代码以生成随机数

首先，我们添加行use rand::Rng。该Rng特征定义了随机数生成器实现的方法，并且该特征必须在我们使用这些方法的范围内。第 10 章将详细介绍特征。

接下来，我们在中间添加两条线。在第一行中，我们调用 rand::thread_rng为我们提供将要使用的特定随机数生成器的函数：一个位于当前执行线程的本地并由操作系统播种的函数。然后我们调用gen_range 随机数生成器上的方法。此方法由我们通过语句Rng 带入范围的特征定义。use rand::Rng该 gen_range方法将范围表达式作为参数，并在该范围内生成一个随机数。我们在这里使用的范围表达式的形式start..end是包含下限但不包含上限，因此我们需要指定1..101请求一个介于 1 和 100 之间的数字。或者，我们可以传递范围1..=100，这是等价的。

```
注意：您不会只知道要使用哪些特征以及从 crate 调用哪些方法和函数，因此每个 crate 都有包含使用说明的文档。Cargo 的另一个巧妙功能是运行该cargo doc --open命令将在本地构建所有依赖项提供的文档并在浏览器中打开它。例如，如果您对randcrate 中的其他功能感兴趣，请运行cargo doc --open并单击rand左侧边栏中的。
```

第二个新行打印密码。这在我们开发能够对其进行测试的程序时很有用，但我们将从最终版本中删除它。如果程序一开始就打印答案，那就不是什么游戏了！

尝试运行该程序几次：

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5

```
你应该得到不同的随机数，它们都应该是 1 到 100 之间的数字。干得好！

## 10. 将猜测与秘密号码进行比较

现在我们有了用户输入和一个随机数，我们可以比较它们。该步骤如清单 2-4 所示。请注意，正如我们将解释的那样，此代码还不能完全编译。

现在我们有了用户输入和一个随机数，我们可以比较它们。该步骤如清单 2-4 所示。请注意，正如我们将解释的那样，此代码还不能完全编译。

文件名：src/main.rs

此代码无法编译！

```
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

示例 2-4：处理比较两个数字的可能返回值

首先，我们添加另一个use语句，将标准库中调用的类型 std::cmp::Ordering引入范围。该Ordering类型是另一个枚举，具有变体Less、Greater和Equal。这是比较两个值时可能出现的三种结果。

然后我们在底部添加五个使用该Ordering类型的新行。该 cmp方法比较两个值，并且可以在任何可以比较的东西上调用。它引用了您想要比较的任何内容：这里将 与 进行guess比较secret_number。然后它返回 我们通过语句Ordering带入范围的枚举的变体。use我们使用一个 match表达式来决定下一步要做什么，这取决于Ordering从调用返回的哪个变体以及和cmp中的值。guesssecret_number

一个match表情是由胳膊组成的。一个手臂包含一个要匹配的模式match，以及如果给定的值 适合该手臂的模式则应该运行的代码。Rust 取给定的值match并依次查看每个臂的模式。模式和match结构是强大的 Rust 功能，可以让您表达代码可能遇到的各种情况并确保您处理所有情况。这些特性将分别在第 6 章和第 18 章中详细介绍。

match让我们用我们在这里使用的表达式来看看一个例子。假设用户猜到了 50，这次随机生成的密码是 38。当代码比较 50 和 38 时，该cmp方法将返回 Ordering::Greater，因为 50 大于 38。match表达式获取该Ordering::Greater值并开始检查每个手臂的模式。它查看第一个臂的模式，Ordering::Less发现值 Ordering::Greater不匹配Ordering::Less，因此它忽略该臂中的代码并移动到下一个臂。下一个手臂的图案是 Ordering::Greater，它确实匹配Ordering::Greater！该手臂中的相关代码将执行并打印Too big!到屏幕上。表达式结束，match 因为在这种情况下它不需要查看最后一个手臂。

但是，示例 2-4 中的代码还不能编译。让我们尝试一下：

```
$ cargo build
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_core v0.6.2
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:22:21
   |
22 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `String`, found integer
   |
   = note: expected reference `&String`
              found reference `&{integer}`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` due to previous error
```
错误的核心是类型不匹配。Rust 有一个强大的静态类型系统。但是，它也有类型推断。当我们编写 时 let mut guess = String::new()，Rust 能够推断出guess应该是 aString并且没有让我们编写类型。secret_number另一方面， 是数字类型。Rust 的一些数字类型的值可以在 1 到 100 之间：i32一个 32 位数字；u32，一个无符号的 32 位数字；i64, 一个 64 位数字；以及其他人。除非另有说明，否则 Rust 默认为i32的类型，secret_number除非您在其他地方添加类型信息，否则会导致 Rust 推断出不同的数字类型。错误的原因是 Rust 无法比较字符串和数字类型。

最终，我们希望将String作为输入读取的程序转换为实数类型，以便我们可以将其与秘密数字进行数字比较。我们通过将这一行添加到main函数体来做到这一点：

文件名：src/main.rs

```

    // --snip--

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```
该行是：

```
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```
我们创建一个名为 的变量guess。但是等等，程序不是已经有一个名为 的变量guess吗？确实如此，但有益的是，Rust 允许我们用一个新的值来掩盖之前的值。guessShadowing 让我们可以重用guess 变量名，而不是强迫我们创建两个唯一的变量，例如 guess_str和guess。我们将在第 3 章中更详细地介绍这一点，但现在知道，当您想将值从一种类型转换为另一种类型时，通常会使用此功能。

我们将这个新变量绑定到表达式guess.trim().parse()。表达式中的是指包含输入作为字符串guess 的原始变量。guess实例上的trim方法String将消除开头和结尾的任何空格，我们必须这样做才能将字符串与u32只能包含数字数据的 进行比较。用户必须按 Enter来满足read_line并输入他们的猜测，这会在字符串中添加一个换行符。例如，如果用户输入5并按下回车，guess看起来像这样5\n：代表“\n 换行符”。（在 Windows 上，按enter会导致回车和换行， \r\n）。该trim方法消除\n或\r\n，得到公正5。

字符串上的parse方法将字符串解析为某种数字。因为这个方法可以解析多种数字类型，所以我们需要使用 . 来告诉 Rust 我们想要的确切数字类型let guess: u32。后面的冒号 ( :)guess告诉 Rust 我们将注释变量的类型。Rust 有一些内置的数字类型；这里u32看到的是一个无符号的 32 位整数。对于小的正数，这是一个很好的默认选择。您将在第 3 章了解其他数字类型。此外，u32此示例程序中的注释和与的比较secret_number意味着 Rust 也将推断它secret_number应该是 a u32。所以现在比较将在相同类型的两个值之间进行！

该parse方法仅适用于可以逻辑转换为数字的字符，因此很容易导致错误。例如，如果字符串包含A👍%，则无法将其转换为数字。因为它可能会失败，所以该parse方法返回一个Result类型，就像该read_line 方法所做的一样（在前面的“使用类型处理潜在故障” Result中讨论过）。我们将再次Result使用该expect方法以同样的方式处理。如果由于无法从字符串中创建数字而parse返回一个变体，则该调用将使游戏崩溃并打印我们给它的消息。如果可以成功地将字符串转换为数字，它将返回的变体Err ResultexpectparseOkResult, 并将expect从值中返回我们想要的Ok数字。

让我们现在运行程序！

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```
好的！即使在猜测之前添加了空格，程序仍然认为用户猜了 76。运行程序几次以验证不同类型输入的不同行为：正确猜数字，猜一个太高的数字，并猜测一个太低的数字。

我们现在已经完成了大部分游戏，但用户只能进行一次猜测。让我们通过添加一个循环来改变它！

## 11. 通过循环允许多个猜测

loop关键字创建一个无限循环。我们将添加一个循环，让用户有更多机会猜测数字：

文件名：src/main.rs

```
    // --snip--

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```
如您所见，我们已将猜测输入提示中的所有内容移至循环中。确保将循环内的行每行缩进四个空格，然后再次运行程序。该程序现在将永远要求另一个猜测，这实际上引入了一个新问题。用户似乎无法退出！

用户可以随时使用键盘快捷键 ctrl-c中断程序。但是还有另一种方法可以摆脱这个贪得无厌的怪物，正如“比较猜测与秘密数字”parse中的讨论所述：如果用户输入非数字答案，程序将崩溃。我们可以利用它来允许用户退出，如下所示：

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
键入quit将退出游戏，但您会注意到，输入任何其他非数字输入也会如此。至少可以说这是次优的；当猜到正确的数字时，我们希望游戏也停止。

猜对后退出
break让我们通过添加一条语句来编写游戏以在用户获胜时退出：

文件名：src/main.rs
```

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```
在用户猜对秘密数字时，添加break后面You win!的行使程序退出循环。退出循环也意味着退出程序，因为循环是main.

处理无效输入
为了进一步细化游戏的行为，而不是在用户输入非数字时使程序崩溃，让我们让游戏忽略非数字，以便用户继续猜测。我们可以通过改变 whereguess 从 a 转换为 aString的行来做到这一点u32，如清单 2-5 所示。

文件名：src/main.rs
```

        // --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // --snip--
```
示例 2-5：忽略一个非数字猜测并要求另一个猜测而不是让程序崩溃

我们从一个expect调用切换到一个match表达式，从一个错误崩溃转移到处理错误。请记住，它parse返回一个Result 类型，并且Result是一个具有变体Ok或的枚举Err。我们在 match这里使用表达式，就像我们对方法的Ordering结果所做的那样cmp 。

如果parse能够成功地将字符串转换为数字，它将返回一个Ok包含结果数字的值。该Ok值将匹配第一个臂的模式，并且match表达式将只返回生成的 num值parse并将其放入该Ok值中。这个数字最终会出现在guess我们正在创建的新变量中我们想要的位置。

如果无法parse将字符串转换为数字，它将返回一个 包含有关错误的更多信息的值。该值与第一个臂中的模式不匹配，但它与第二个臂中的模式匹配。下划线 ,是一个包罗万象的值；在这个例子中，我们说我们想要匹配所有的 值，不管它们里面有什么信息。所以程序将执行第二个臂的代码，告诉程序进入下一次迭代并要求另一个猜测。因此，有效地，程序忽略了所有可能遇到的错误！ErrErrOk(num)matchErr(_)_Errcontinueloopparse

现在程序中的所有内容都应该按预期工作。让我们尝试一下：

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

惊人的！通过一个微小的最后调整，我们将完成猜谜游戏。回想一下，程序仍在打印密码。这对测试很有效，但它破坏了游戏。让我们删除println!输出秘密号码的那个。清单 2-6 显示了最终代码。

文件名：src/main.rs

```
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```
示例 2-6：完整的猜谜游戏代码

## 12. 总结

至此，您已经成功构建了猜谜游戏。恭喜！

这个项目是向您介绍许多新的 Rust 概念的实践方式：、、、 let函数match、外部 crate 的使用等等。在接下来的几章中，您将更详细地了解这些概念。第 3 章涵盖了大多数编程语言所具有的概念，例如变量、数据类型和函数，并展示了如何在 Rust 中使用它们。第 4 章探讨所有权，这是 Rust 与其他语言不同的一个特性。第 5 章讨论结构和方法语法，第 6 章解释枚举是如何工作的。
