# 枚举和模式匹配

我最近在重新学rust ，巩固一下细节，也写一个Chaineye rust 教程，供想要学习的小伙伴查看。

推特：@seek_web3

Chainey 社群： 官网 chaineye.info | Chaineye Rust 教程 | 微信: LGZAXE, 加微信之后拉各社群

所有代码和教程开源在github: https://github.com/0xchaineye/chaineye-rust

-----------------------------------------------------------------------------------------------------------------------------------------------------------

## 一. 定义枚举

结构为您提供了一种将相关字段和数据分组在一起的方式，例如 aRectangle及其widthand height，而枚举为您提供了一种表示值是一组可能值中的一个的方式。例如，我们可能想说 that Rectangle是一组可能的形状中的一个，其中还包括Circle和 Triangle。为此，Rust 允许我们将这些可能性编码为枚举。

让我们看一下我们可能想用代码表达的情况，看看为什么在这种情况下枚举比结构有用且更合适。假设我们需要使用 IP 地址。目前，IP 地址使用两个主要标准：第四版和第六版。因为这些是我们的程序将遇到的 IP 地址的唯一可能性，所以我们可以枚举所有可能的变体，这就是枚举得名的地方。

任何 IP 地址都可以是第四版或第六版地址，但不能同时是这两者。IP 地址的这一属性使枚举数据结构变得合适，因为枚举值只能是其变体之一。第四版和第六版地址基本上仍然是 IP 地址，因此当代码处理适用于任何类型 IP 地址的情况时，它们应该被视为同一类型。

我们可以通过定义一个IpAddrKind枚举并列出一个 IP 地址的可能类型来在代码中表达这个概念，V4以及V6. 这些是枚举的变体：

```
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {}
```

IpAddrKind现在是我们可以在代码的其他地方使用的自定义数据类型。

### 1. 枚举值

我们可以像这样创建两个变体中的每一个的实例IpAddrKind：

```
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {}
```

请注意，枚举的变体在其标识符下命名空间，我们使用双冒号将两者分开。这很有用，因为现在值 IpAddrKind::V4和IpAddrKind::V6属于同一类型：IpAddrKind。然后，例如，我们可以定义一个接受任何函数的函数IpAddrKind：

```
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {}
```

我们可以使用任一变体调用此函数：

```
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {}
```

使用枚举还有更多优势。更多地考虑我们的 IP 地址类型，目前我们没有办法存储实际的 IP 地址数据；我们只知道它是哪种。鉴于您刚刚在第 5 章中了解了结构，您可能会想用结构来解决这个问题，如清单 6-1 所示。

```
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```
IpAddrKind清单 6-1：使用一个 IP 地址存储数据和变体struct

在这里，我们定义了一个IpAddr具有两个字段的结构：一个kind字段类型IpAddrKind（我们之前定义的枚举）和一个address字段类型String。我们有这个结构的两个实例。第一个是home，它的值IpAddrKind::V4与其kind关联的地址数据相同127.0.0.1。第二个例子是loopback。IpAddrKind它的kind值是 的另一个变体，V6并且有::1 与之关联的地址。我们使用结构将kind和address 值捆绑在一起，所以现在变体与值相关联。

然而，仅使用枚举来表示相同的概念更为简洁：我们可以将数据直接放入每个枚举变体中，而不是结构内部的枚举。枚举的这个新定义IpAddr表示V4和V6 变体都将具有关联String值：

```
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}
```

我们直接将数据附加到枚举的每个变体，因此不需要额外的结构。在这里，也更容易看到枚举如何工作的另一个细节：我们定义的每个枚举变体的名称也成为构造枚举实例的函数。也就是说，IpAddr::V4()是一个函数调用，它接受一个String参数并返回该IpAddr类型的一个实例。我们自动将此构造函数定义为定义枚举的结果。

使用枚举而不是结构还有另一个优点：每个变体可以具有不同类型和数量的关联数据。第 4 版 IP 地址将始终包含四个数字部分，其值介于 0 和 255 之间。如果我们想将V4地址存储为四个u8值但仍将V6地址表示为一个String值，我们将无法使用结构。枚举可以轻松处理这种情况：

```
fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
```

我们已经展示了几种不同的方法来定义数据结构来存储第四版和第六版 IP 地址。然而，事实证明，想要存储 IP 地址并对其进行编码非常普遍，以至于标准库中有一个我们可以使用的定义！让我们看看标准库是如何定义的IpAddr：它具有我们定义和使用的确切枚举和变体，但它以两种不同结构的形式将地址数据嵌入到变体中，每个变体的定义不同：

```
#![allow(unused)]
fn main() {
  struct Ipv4Addr {
      // --snip--
  }

  struct Ipv6Addr {
      // --snip--
  }

  enum IpAddr {
      V4(Ipv4Addr),
      V6(Ipv6Addr),
  }
}
```
此代码说明您可以将任何类型的数据放入枚举变体中：例如，字符串、数字类型或结构。你甚至可以包括另一个枚举！此外，标准库类型通常并不比您可能想到的复杂多少。

请注意，即使标准库包含的定义IpAddr，我们仍然可以创建和使用我们自己的定义而不会发生冲突，因为我们没有将标准库的定义纳入我们的范围。我们将在第 7 章详细讨论将类型引入作用域。

让我们看一下清单 6-2 中的另一个枚举示例：这个枚举的变体中嵌入了多种类型。

```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {}
```
清单 6-2：一个Message枚举，其变体各自存储不同数量和类型的值

这个枚举有四种不同类型的变体：

Quit根本没有与之关联的数据。
Move具有命名字段，就像结构一样。
Write包括一个String.
ChangeColor包括三个i32值。
定义带有变体的枚举（如清单 6-2 中的变体）类似于定义不同类型的结构定义，只是枚举不使用关键字并且 struct所有变体都在类型下组合在一起Message 。以下结构可以包含与前面的枚举变体相同的数据：

```
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn main() {}
```

Message但是如果我们使用不同的结构，每个结构都有自己的类型，我们就不能像使用清单 6-2 中定义的枚举那样轻松地定义一个函数来接收这些类型的消息，它是一个单一的类型。

枚举和结构之间还有一个相似之处：正如我们能够使用 定义结构上的方法一样impl，我们也能够在枚举上定义方法。call这是一个我们可以在枚举上定义的方法Message：

```
fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
```
该方法的主体将用于self获取我们调用该方法的值。在此示例中，我们创建了一个m值为 的 变量Message::Write(String::from("hello"))，这就是运行时方法self主体中的内容。callm.call()

让我们看看标准库中另一个非常常见和有用的枚举：Option.

枚举Option及其相对于空值的优势
本节探讨 的案例研究Option，这是标准库定义的另一个枚举。该Option类型对非常常见的场景进行编码，在这种场景中，值可以是某物，也可以什么都不是。

例如，如果您请求非空列表中的第一项，您将获得一个值。如果您请求空列表中的第一项，您将一无所获。用类型系统表达这个概念意味着编译器可以检查你是否已经处理了所有你应该处理的情况；此功能可以防止在其他编程语言中极为常见的错误。

编程语言设计通常根据包含哪些功能来考虑，但排除的功能也很重要。Rust 没有许多其他语言所具有的 null 特性。Null是一个值，表示那里没有值。在具有 null 的语言中，变量始终处于两种状态之一：null 或 not-null。

在他 2009 年的演讲“空引用：十亿美元的错误”中，null 的发明者 Tony Hoare 是这样说的：

我称之为我的十亿美元错误。那时，我正在为面向对象语言的引用设计第一个综合类型系统。我的目标是确保所有引用的使用都绝对安全，并由编译器自动执行检查。但我无法抗拒放入空引用的诱惑，只是因为它很容易实现。这导致了无数的错误、漏洞和系统崩溃，在过去四十年中可能造成了十亿美元的痛苦和损失。

空值的问题在于，如果您尝试将空值用作非空值，则会出现某种错误。因为这个 null 或 not-null 属性是普遍存在的，所以很容易犯这种错误。

然而，null 试图表达的概念仍然是一个有用的概念：null 是当前无效或由于某种原因不存在的值。

问题不在于概念，而在于特定的实现。因此，Rust 没有空值，但它确实有一个枚举，可以对值存在或不存在的概念进行编码。这个枚举是 Option<T>，它由标准库定义 如下：

```
#![allow(unused)]
fn main() {
  enum Option<T> {
      None,
      Some(T),
  }
}
```

枚举Option<T>是如此有用，以至于它甚至包含在前奏中；您不需要明确地将其纳入范围。它的变体也包含在prelude中：你可以直接使用Someand而无需 前缀。枚举仍然只是一个常规枚举，并且仍然 是 type 的变体。NoneOption::Option<T>Some(T)NoneOption<T>

语法<T>是我们尚未讨论的 Rust 的一个特性。它是一个泛型类型参数，我们将在第 10 章中更详细地介绍泛型。现在，您只需要知道这意味着枚举的变体可以保存<T>任何Some类型Option的一个数据，并且每个具体代替使用的类型T使整体Option<T>类型成为不同的类型。Option以下是一些使用值来保存数字类型和字符串类型的示例：

```
fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
```

的类型some_number是Option<i32>。的类型some_char是 Option<char>，这是一个不同的类型。Rust 可以推断出这些类型，因为我们已经在Some变量中指定了一个值。对于absent_number，Rust 要求我们注释整体类型：编译器无法仅通过查看一个 值Option来推断相应变体将持有的类型。在这里，我们告诉 Rust 我们的意思是类型 。SomeNoneabsent_numberOption<i32>

当我们有一个Some值时，我们知道存在一个值并且该值保存在Some. 当我们有一个None值时，在某种意义上它意味着与 null 相同的东西：我们没有有效值。那么为什么拥有Option<T> 比拥有空值更好呢？

简而言之，因为Option<T>和T（T可以是任何类型）是不同的类型，编译器不会让我们使用一个Option<T>值，就好像它绝对是一个有效值一样。例如，此代码将无法编译，因为它试图将 an 添加i8到 an Option<i8>：

此代码无法编译！

```
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
```

如果我们运行这段代码，我们会收到如下错误消息：

```
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`
  = help: the following other types implement trait `Add<Rhs>`:
            <&'a f32 as Add<f32>>
            <&'a f64 as Add<f64>>
            <&'a i128 as Add<i128>>
            <&'a i16 as Add<i16>>
            <&'a i32 as Add<i32>>
            <&'a i64 as Add<i64>>
            <&'a i8 as Add<i8>>
            <&'a isize as Add<isize>>
          and 48 others

For more information about this error, try `rustc --explain E0277`.
error: could not compile `enums` due to previous error
```

激烈的！实际上，此错误消息意味着 Rust 不理解如何添加 ani8和 an Option<i8>，因为它们是不同的类型。当我们有一个像 Rust 中的类型的值时i8，编译器将确保我们始终有一个有效值。我们可以放心地继续，而不必在使用该值之前检查是否为 null。只有当我们有一个Option<i8>（或我们正在使用的任何类型的值）时，我们才需要担心可能没有值，并且编译器将确保我们在使用该值之前处理这种情况。

换句话说，您必须先将 an 转换Option<T>为 a T，然后才能T对其执行操作。通常，这有助于捕捉 null 最常见的问题之一：假设某些东西实际上不是 null。

消除错误假设非空值的风险有助于您对代码更有信心。为了拥有一个可能为空的值，您必须通过设置该值的类型来明确选择加入Option<T>。然后，当您使用该值时，您需要显式处理该值为 null 的情况。只要值的类型不是 Option<T>，您就可以放心地假设该值不为空。这是 Rust 有意设计的决定，目的是限制 null 的普遍存在并提高 Rust 代码的安全性。

那么当你有一个类型的值时，你如何T从变体中获取值以便你可以使用该值呢？枚举有大量的方法，在各种情况下都很有用；您可以在其文档中查看它们。熟悉上的方法将对您的 Rust 之旅非常有用。SomeOption<T>Option<T>Option<T>

通常，为了使用一个Option<T>值，您需要拥有处理每个变体的代码。您想要一些只有在您有值时才会运行的代码 Some(T)，并且允许此代码使用内部T. 您希望某些其他代码仅在您有值时运行None，而该代码没有 T可用值。表达式match是一个控制流结构，当与枚举一起使用时，它会执行此操作：它将根据枚举的变体运行不同的代码，并且该代码可以使用匹配值中的数据。








