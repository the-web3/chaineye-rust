# 使用结构来构造相关数据

我最近在重新学rust ，巩固一下细节，也写一个Chaineye rust 教程，供想要学习的小伙伴查看。

推特：@seek_web3

Chainey 社群： 官网 chaineye.info | Chaineye Rust 教程 | 微信: LGZAXE, 加微信之后拉各社群

所有代码和教程开源在github: https://github.com/0xchaineye/chaineye-rust

-----------------------------------------------------------------------------------------------------------------------------------------------------------

## 一. 定义和实例化结构

struct或structure是一种自定义数据类型，可让您将多个相关值打包并命名，从而组成一个有意义的组。如果您熟悉面向对象的语言，那么结构就像对象的数据属性。在本章中，我们将比较和对比元组和结构，以构建您已经了解的内容，并演示何时结构是更好的数据分组方式。

我们将演示如何定义和实例化结构。我们将讨论如何定义关联函数，尤其是称为 方法的关联函数，以指定与结构类型关联的行为。结构和枚举（在第 6 章中讨论）是在程序域中创建新类型以充分利用 Rust 的编译时类型检查的构建块。

结构类似于“元组类型”部分中讨论的元组，因为它们都包含多个相关值。与元组一样，结构的各个部分可以是不同的类型。与元组不同，在结构中，您将为每条数据命名，以便清楚这些值的含义。添加这些名称意味着结构比元组更​​灵活：您不必依赖数据的顺序来指定或访问实例的值。

要定义结构，我们输入关键字struct并命名整个结构。结构的名称应该描述组合在一起的数据片段的重要性。然后，在大括号内，我们定义了数据片段的名称和类型，我们称之为字段。例如，清单 5-1 显示了一个存储有关用户帐户信息的结构。

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {}
```
清单 5-1：User结构定义


要在定义结构后使用它，我们通过为每个字段指定具体值来创建该结构的实例。我们通过声明结构的名称创建一个实例，然后添加包含键：值对的大括号，其中键是字段的名称，值是我们要存储在这些字段中的数据。我们不必按照我们在结构中声明它们的相同顺序指定字段。换句话说，结构定义就像是该类型的通用模板，实例用特定数据填充该模板以创建该类型的值。例如，我们可以声明一个特定的用户，如清单 5-2 所示。

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```
User 清单 5-2：创建结构的实例

要从结构中获取特定值，我们使用点表示法。例如，要访问此用户的电子邮件地址，我们使用user1.email. 如果实例是可变的，我们可以通过使用点符号并分配到特定字段来更改值。email 清单 5-3 显示了如何更改可变实例字段中的值User。

```
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```
示例 5-3：更改实例email字段 中的值User

请注意，整个实例必须是可变的；Rust 不允许我们只将某些字段标记为可变的。与任何表达式一样，我们可以构造一个结构的新实例作为函数体中的最后一个表达式，以隐式返回该新实例。

清单 5-4 显示了一个返回具有给定电子邮件和用户名的build_user实例的函数。User该active字段的值为true，而sign_in_count的值为1。

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}
```
清单 5-4：一个build_user接受电子邮件和用户名并返回User实例的函数

将函数参数命名为与结构字段同名是有意义的，但必须重复email字段username名称和变量有点乏味。如果结构有更多字段，重复每个名称会变得更烦人。幸运的是，有一个方便的速记！

### 1.使用 Field Init 速记

因为参数名称和结构字段名称在清单 5-4 中完全相同，我们可以使用field init 简写语法重写 build_user，使其行为完全相同但没有重复的 usernameand email，如清单 5 所示-5。

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}
```
示例 5-5：一个build_user使用 field init 简写的函数，因为username和email参数与 struct fields 同名

在这里，我们正在创建User结构的一个新实例，它有一个名为email. 我们想将字段的值设置为函数参数email中的值 。因为字段和参数同名，所以我们只需要写而不是。email build_user email email email email: email

### 2.使用结构更新语法从其他实例创建实例

创建结构的新实例通常很有用，该实例包含另一个实例的大部分值，但会更改一些值。您可以使用 struct update syntax执行此操作。

首先，在清单 5-6 中，我们展示了如何在 没有更新语法的情况下定期创建一个新User实例。user2我们为 设置了一个新值，但在其他方面使用了我们在清单 5-2 中创建的email相同值。user1

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```
清单 5-6：User使用来自的值之一创建一个新实例user1

使用 struct update 语法，我们可以用更少的代码实现相同的效果，如清单 5-7 所示。该语法..指定未显式设置的其余字段应与给定实例中的字段具有相同的值。

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```
示例 5-7：使用 struct update 语法为实例设置一个新 email值User，但使用来自 user1

清单 5-7 中的代码还在 中创建了一个实例，user2该实例具有不同的值，email但具有相同的username、 active和sign_in_count字段值user1。必须..user1最后指定任何剩余字段应从中的相应字段获取它们的值user1，但我们可以选择以任何顺序为任意数量的字段指定值，而不管结构定义中字段的顺序如何。

请注意，struct update 语法的用法=类似于赋值；这是因为它移动了数据，正如我们在“与移动交互的变量和数据”部分中看到的那样。user1在这个例子中，我们创建后不能再作为一个整体使用 user2，因为 的字段String被 移到了 中。如果我们为和 都赋予了新 值，因此只使用了 和 的值，那么在创建 之后仍然有效。和都是实现特征的类型，因此我们在“仅堆栈数据：复制”部分中讨论的行为将适用。username user1 user2 user2 String email username active sign_in_count user1 user1 user2 activesign_in_count Copy

#### 2.1. 使用没有命名字段的元组结构来创建不同的类型

Rust 还支持看起来类似于元组的结构，称为元组结构。元组结构具有结构名称提供的附加含义，但没有与其字段关联的名称；相反，它们只有字段的类型。当您想为整个元组命名并使元组与其他元组具有不同的类型时，元组结构很有用，并且在将每个字段命名为常规结构时会显得冗长或多余。

要定义元组结构，请从struct关键字和结构名称开始，后跟元组中的类型。例如，这里我们定义并使用两个名为Colorand的元组结构Point：

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

请注意，black和origin值是不同的类型，因为它们是不同元组结构的实例。您定义的每个结构都是它自己的类型，即使结构中的字段可能具有相同的类型。例如，采用类型参数的函数Color不能将 a Point作为参数，即使这两种类型都由三个值组成i32 。否则，元组结构实例类似于元组，因为您可以将它们解构为单独的部分，并且可以使用后跟索引.来访问单独的值。


#### 2.2.没有任何字段的类单元结构

您还可以定义没有任何字段的结构！这些被称为 类单元结构，因为它们的行为类似于我们在“元组类型”()部分中提到的单元类型。当您需要在某种类型上实现特征但没有要存储在类型本身中的任何数据时，类似单元的结构会很有用。我们将在第 10 章讨论 traits。下面是一个声明和实例化名为 unit struct 的示例：AlwaysEqual

```
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

要定义AlwaysEqual，我们使用struct关键字、我们想要的名称，然后是分号。不需要大括号或圆括号！AlwaysEqual然后我们可以用类似的方式在变量中获取一个实例subject：使用我们定义的名称，不带任何大括号或圆括号。想象一下，稍后我们将为这种类型实现行为，使得 的每个实例 AlwaysEqual始终等于任何其他类型的每个实例，也许为了测试目的有一个已知结果。我们不需要任何数据来实现该行为！您将在第 10 章中看到如何定义特征并在任何类型上实现它们，包括类单元结构。

### 3. 结构数据的所有权

在User清单 5-1 的结构定义中，我们使用了拥有的String 类型而不是&str字符串切片类型。这是一个深思熟虑的选择，因为我们希望这个结构的每个实例都拥有它的所有数据，并且只要整个结构有效，该数据就有效。

结构也可以存储对其他对象拥有的数据的引用，但这样做需要使用生命周期，我们将在第 10 章讨论的 Rust 特性。生命周期确保结构引用的数据有效只要结构是。假设您尝试在不指定生命周期的情况下将引用存储在结构中，如下所示；这行不通：

```
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
```
编译器会抱怨它需要生命周期说明符：

```
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:15
  |
3 |     username: &str,
  |               ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 ~     username: &'a str,
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:4:12
  |
4 |     email: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 |     username: &str,
4 ~     email: &'a str,
  |

For more information about this error, try `rustc --explain E0106`.
error: could not compile `structs` due to 2 previous errors

```

String在第 10 章中，我们将讨论如何修复这些错误，以便您可以将引用存储在结构中，但现在，我们将使用拥有的类型而不是引用来修复此类错误&str。


### 二.使用结构的示例程序

为了解何时可能需要使用结构，让我们编写一个计算矩形面积的程序。我们将从使用单个变量开始，然后重构程序直到我们改用结构。

让我们用 Cargo 创建一个名为rectangles的新二进制项目，它将获取以像素为单位指定的矩形的宽度和高度，并计算矩形的面积。清单 5-8 显示了一个简短的程序，其中包含一种在我们项目的src/main.rs中执行此操作的方法。

```
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```
示例 5-8：计算由单独的宽度和高度变量指定的矩形的面积

现在，使用以下命令运行该程序cargo run：

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/rectangles`
The area of the rectangle is 1500 square pixels.
```

这段代码通过调用每个维度的函数成功地计算出矩形的面积 area，但我们可以做更多的事情来使这段代码清晰易读。

此代码的问题在以下签名中很明显area：

```
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

该area函数应该计算一个矩形的面积，但是我们编写的函数有两个参数，并且我们的程序中的任何地方都不清楚这些参数是否相关。将宽度和高度组合在一起会更具可读性和更易于管理。我们已经在第 3 章的“元组类型”一节中讨论了一种可能的方法：使用元组。

### 1.用元组重构

清单 5-9 显示了我们程序的另一个版本，它使用了元组。

```
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```
示例 5-9：用元组指定矩形的宽度和高度

在某种程度上，这个程序更好。元组让我们添加一点结构，我们现在只传递一个参数。但换句话说，这个版本不太清楚：元组不命名它们的元素，所以我们必须索引到元组的各个部分，使我们的计算不那么明显。

混合宽度和高度对面积计算没有影响，但如果我们想在屏幕上绘制矩形，那就很重要了！我们必须记住那width是元组索引0并且height是元组索引1。如果其他人要使用我们的代码，这将更难弄清楚并牢记在心。因为我们没有在代码中传达数据的含义，所以现在更容易引入错误。

### 2. 用结构重构：增加更多意义

我们使用结构通过标记数据来增加意义。我们可以将我们正在使用的元组转换为一个结构，该结构具有整体名称和部分名称，如清单 5-10 所示。

```
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```
示例 5-10：定义一个Rectangle结构

在这里，我们定义了一个结构并将其命名为Rectangle。在大括号内，我们将字段定义为width和height，它们的类型都是u32。然后在其中，我们创建了一个 宽度为 高度main为 的特定实例。Rectangle 30 50

我们的area函数现在定义了一个参数，我们将其命名为 rectangle，其类型是对结构Rectangle 实例的不可变借用。正如第 4 章所述，我们想要借用结构而不是拥有它。这样，main保留其所有权并可以继续使用，这就是我们在函数签名和调用函数的地方rect1使用的原因。&

该area函数访问实例的width和height字段Rectangle （请注意，访问借用的结构实例的字段不会移动字段值，这就是为什么您经常看到借用结构的原因）。我们现在的函数签名准确地表达了我们的意思：使用它的和字段area计算 的面积。这表明宽度和高度彼此相关，并且它为值提供了描述性名称，而不是使用和的元组索引值。这是清晰的胜利。Rectangle width height 0 1

### 3.使用派生特征添加有用的功能

Rectangle当我们调试我们的程序并查看其所有字段的值时，能够打印一个实例是很有用的。清单 5-11 尝试使用我们在前面章节中使用的println!宏。然而，这是行不通的。

```
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```
示例 5-11：尝试打印一个Rectangle 实例

当我们编译这段代码时，我们会收到一条错误消息：

```
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

该println!宏可以进行多种格式设置，默认情况下，花括号指示println!使用称为Display: output for direct end user consumption 的格式。到目前为止，我们看到的基本类型都是Display默认实现的，因为您只有一种方式可以1向用户显示一个或任何其他基本类型。但是对于结构，格式化输出的方式 println!就不那么清晰了，因为有更多的显示可能性：你要不要逗号？你想打印大括号吗？是否应显示所有字段？由于这种歧义，Rust 不会尝试猜测我们想要什么，并且结构没有提供与占位符Display一起使用的实现。println!{}

如果我们继续阅读错误，我们会发现这个有用的注释：

```
= help: the trait `std::fmt::Display` is not implemented for `Rectangle`
= note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

让我们试试吧！宏println!调用现在看起来像println!("rect1 is {:?}", rect1);。将说明符:?放在大括号内表示 println!我们要使用名为Debug. 该Debug特征使我们能够以对开发人员有用的方式打印我们的结构，这样我们就可以在调试代码时看到它的价值。

使用此更改编译代码。糟了！我们仍然得到一个错误：

```
error[E0277]: `Rectangle` doesn't implement `Debug`
```

但同样，编译器给了我们一个有用的提示：

```
= help: the trait `Debug` is not implemented for `Rectangle`
= note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```
Rust确实包含打印调试信息的功能，但我们必须明确选择让该功能可用于我们的结构。为此，我们#[derive(Debug)]在结构定义之前添加外部属性，如清单 5-12 所示。

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```
示例 5-12：添加属性以派生特征并使用调试格式Debug 打印实例Rectangle

现在当我们运行程序时，我们不会得到任何错误，我们会看到以下输出：

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1 is Rectangle { width: 30, height: 50 }
```

好的！它不是最漂亮的输出，但它显示了该实例所有字段的值，这在调试过程中肯定会有所帮助。当我们有更大的结构时，输出更容易阅读是很有用的；在这些情况下，我们可以在字符串中使用{:#?}而不是。在此示例中，使用样式将输出以下内容：{:?}println!{:#?}

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```

另一种使用格式打印出值的方法Debug是使用dbg! 宏，它获取表达式的所有权（与println!获取引用相反），打印dbg!代码中发生宏调用的文件和行号以及该表达式的结果值，并返回该值的所有权。

* 注意：调用dbg!宏打印到标准错误控制台流 ( stderr)，而不是println!，它打印到标准输出控制台流 ( stdout)。我们将在 第 12 章的“将错误消息写入标准错误而不是标准输出”部分详细讨论stderr和。stdout

width这是一个示例，我们对分配给该字段的值以及整个结构的值感兴趣 rect1：

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

我们可以dbg!围绕表达式30 * scale，因为dbg! 返回表达式值的所有权，该width字段将获得相同的值，就好像我们在那里没有调用一样dbg!。我们不想dbg!取得 的所有权，因此我们在下一次调用中rect1使用对的引用。rect1下面是这个例子的输出：

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/rectangles`
[src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

我们可以看到输出的第一位来自src/main.rs 的第 10 行，我们正在调试表达式30 * scale，它的结果值为60（ Debug为整数实现的格式是只打印它们的值）。src/main.rsdbg!第 14 行的调用输出 了 的值，也就是结构体。此输出使用类型的 漂亮格式。当您试图弄清楚您的代码在做什么时，宏真的很有帮助！&rect1RectangleDebugRectangledbg!

除了Debugtrait 之外，Rust 还提供了许多 traits 供我们与deriveattribute 一起使用，可以为我们的自定义类型添加有用的行为。这些特征及其行为在附录 C中列出。我们将在第 10 章介绍如何使用自定义行为实现这些特征以及如何创建您自己的特征derive。有关详细信息，请参阅Rust 参考的“属性”部分。

我们的area函数非常具体：它只计算矩形的面积。将此行为更紧密地与我们的结构联系起来会很有帮助，Rectangle因为它不适用于任何其他类型。area让我们看看如何通过将函数转换为 在我们的类型上定义的area 方法来继续重​​构此代码Rectangle。

## 三. 方法语法


方法类似于函数：我们用fn关键字和名称声明它们，它们可以有参数和返回值，并且它们包含一些在从其他地方调用方法时运行的代码。与函数不同，方法是在结构（或枚举或特征对象，我们分别在第6 章和第 17 章中介绍）的上下文中定义的，并且它们的第一个参数始终是，self它表示结构的实例 方法正在被召唤。

### 1. 定义方法

让我们更改将实例作为参数的area函数Rectangle，改为area在结构上定义一个方法Rectangle，如清单 5-13 所示。

文件名：src/main.rs

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```
示例 5-13：在结构上定义一个area方法 Rectangle

为了在 的上下文中定义函数Rectangle，我们 impl 为启动一个（实现）块 Rectangle。此 impl 块中的所有内容都将与类型相关联 Rectangle。然后我们将 area 函数移动到大括号内 impl，并将第一个（在本例中是唯一的）参数更改为self在签名中和主体中的任何地方。在 main 中 ，我们调用area函数并rect1作为参数传递的地方，我们可以改为使用方法语法area在我们的实例上调用方法Rectangle 。方法语法在实例之后：我们添加一个点，后跟方法名称、括号和任何参数。

在 的签名中area，我们使用&selfinstead of rectangle: &Rectangle。The&self实际上是self: &Self. 在impl块中，类型Self是块所针对的类型的别名impl。方法的第一个参数必须有一个名为selftype 的参数，因此 Rust 允许您仅使用第一个参数位置中的Self名称来缩写它。请注意，我们仍然需要在简写前面self使用来表示此方法借用实例，就像我们在 中所做的那样 。方法可以获取 的所有权，不可变地借用 ，就像我们在这里所做的那样，或者可变地借用，就像它们可以任何其他参数一样。&selfSelfrectangle: &Rectangleselfselfself

我们选择&self这里的原因与我们&Rectangle在函数版本中使用的原因相同：我们不想取得所有权，我们只想读取结构中的数据，而不是写入它。如果我们想更改我们调用该方法的实例作为该方法的一部分，我们将用作&mut self第一个参数。self很少有方法通过使用第一个参数来获取实例的所有权；当方法转换self成其他东西并且你想阻止调用者在转换后使用原始实例时，通常会使用这种技术。

使用方法而不是函数的主要原因，除了提供方法语法和不必self在每个方法的签名中重复类型之外，是为了组织。我们已经将我们可以用一个类型的实例做的所有事情都放在一个impl块中，而不是让我们代码的未来用户Rectangle在我们提供的库的不同地方搜索功能。

请注意，我们可以选择为方法指定与结构字段之一相同的名称。例如，我们可以在其上定义一个方法，Rectangle该方法也被命名为 width：

文件名：src/main.rs

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
在这里，如果实例字段中的值大于并且值为 ，我们选择让width方法返回：我们可以出于任何目的在同名方法中使用字段。在 中 ，当我们跟在括号中时，Rust 知道我们指的是方法。当我们不使用括号时，Rust 知道我们指的是字段 。truewidth0false0mainrect1.widthwidthwidth

通常，但并非总是如此，当我们为方法赋予与字段相同的名称时，我们希望它只返回字段中的值而不做任何其他事情。像这样的方法称为getters，Rust 不会像其他一些语言那样自动为结构字段实现它们。Getters 很有用，因为您可以将字段设为私有，但将方法设为公有，从而作为类型的公共 API 的一部分启用对该字段的只读访问。我们将在第 7 章讨论什么是公共和私有以及如何将字段或方法指定为公共或私有。

    接线员在哪里->？
    在 C 和 C++ 中，两种不同的运算符用于调用方法： .如果您直接在对象上调用方法，则使用，->如果您在指向对象的指针上调用方法，并且需要先取消引用指针。换句话说，ifobject是一个指针， object->something()类似于(*object).something().

    Rust 没有与->运算符等价的东西；相反，Rust 具有称为自动引用和取消引用的功能。调用方法是 Rust 中为数不多的具有这种行为的地方之一。

    它是这样工作的：当你用 调用一个方法时object.something()，Rust 会自动添加&、&mut，或者*匹配object方法的签名。换句话说，以下是相同的：

    p1.distance(&p2);
    (&p1).distance(&p2);
    第一个看起来干净多了。这种自动引用行为之所以有效，是因为方法有一个明确的接收者—— self. 给定方法的接收者和名称，Rust 可以确定该方法是读取 ( &self)、变异 ( &mut self) 还是消费 ( self)。Rust 让方法接收者隐式借用这一事实是使所有权在实践中符合人体工学的重要组成部分。

### 2. 具有更多参数的方法

让我们通过在结构上实现第二个方法来练习使用方法Rectangle 。这次我们想要一个实例Rectangle获取另一个实例Rectangle并返回true如果第二个Rectangle可以完全适合self（第一个Rectangle）; 否则，它应该返回false。也就是说，一旦我们定义了can_hold方法，我们就希望能够编写如清单 5-14 所示的程序。

文件名：src/main.rs

```
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```
示例 5-14：使用尚未编写的can_hold 方法

预期输出如下所示，因为 的两个维度都 rect2小于 的维度rect1，但rect3大于 的 维度rect1：

Can rect1 hold rect2? true
Can rect1 hold rect3? false
我们知道我们想要定义一个方法，所以它会在impl Rectangle 块中。方法名称将是can_hold，它将以不可变的方式借用另一个Rectangle作为参数。我们可以通过查看调用该方法的代码来判断参数的类型： rect1.can_hold(&rect2)传入，这是对 的一个实例的&rect2不可变借用 。这是有道理的，因为我们只需要读取（而不是写入，这意味着我们需要一个可变的借用），并且我们希望保留所有权，以便我们可以在调用该方法后再次使用它。的返回值将是一个布尔值，实现时会检查是否宽高 rect2Rectanglerect2mainrect2can_holdcan_holdself分别大于另一个 的宽度和高度Rectangle。让我们将新can_hold方法添加到impl清单 5-13 中的块中，如清单 5-15 所示。

文件名：src/main.rs

```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```
示例 5-15：实现将另一个实例作为参数的can_hold方法 RectangleRectangle

当我们使用清单 5-14 中的函数运行这段代码时main，我们将得到我们想要的输出。方法可以接受多个参数，我们在参数后添加到签名中self，这些参数就像函数中的参数一样工作。

相关功能
块中定义的所有函数impl都称为关联函数 ，因为它们与以impl. 我们可以定义没有self第一个参数（因此不是方法）的关联函数，因为它们不需要类型的实例来处理。我们已经使用了这样一个函数：String::from在类型上定义的函数String。

不是方法的关联函数通常用于将返回结构的新实例的构造函数。这些通常称为new，但 new不是特殊名称，也没有内置到语言中。例如，我们可以选择提供一个名为的关联函数square，该函数将具有一个维度参数并将其用作宽度和高度，从而使创建正方形更容易Rectangle，而不必指定两次相同的值：

文件名：src/main.rs

```
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

Self返回类型和函数主体中的关键字是出现在关键字之后的类型的别名，impl在本例中是Rectangle.

要调用此关联函数，我们使用::带有结构名称的语法； let sq = Rectangle::square(3);是一个例子。此函数由结构命名空间：::语法用于关联函数和模块创建的命名空间。我们将在第 7 章讨论模块。

多impl块
每个结构允许有多个impl块。例如，清单 5-15 等同于清单 5-16 中所示的代码，其中每个方法都在其自己的impl块中。

```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```
示例 5-16：使用多个impl 块重写示例 5-15

这里没有理由将这些方法分成多个impl块，但这是有效的语法。impl我们将在第 10 章中看到使用多个块的情况，我们将在该章讨论泛型类型和特征。

## 四. 概括
结构让您可以创建对您的领域有意义的自定义类型。通过使用结构，您可以保持关联的数据片段相互连接并命名每个片段以使您的代码清晰。在impl块中，您可以定义与您的类型关联的函数，而方法是一种关联函数，可让您指定结构实例具有的行为。

但是结构并不是您可以创建自定义类型的唯一方式：让我们转向 Rust 的枚举功能，将另一个工具添加到您的工具箱中。

