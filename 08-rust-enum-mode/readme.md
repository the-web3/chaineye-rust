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


## 二.控制match流构造
    
Rust 有一个非常强大的控制流结构match，它允许您将一个值与一系列模式进行比较，然后根据匹配的模式执行代码。模式可以由文字值、变量名、通配符和许多其他内容组成；第 18 章涵盖了所有不同类型的模式以及它们的作用。的力量match来自于模式的表现力和编译器确认所有可能的情况都被处理的事实。

将match表达式想象成一个硬币分类机：硬币沿着轨道滑下，轨道上有不同大小的孔，每枚硬币都会从它遇到的第一个孔中掉落。以同样的方式，值通过 a 中的每个模式match，并且在第一个模式中值“适合”，该值落入关联的代码块中以在执行期间使用。

说到硬币，让我们以它们为例使用match！我们可以编写一个函数，它接受一枚未知的美国硬币，并以与计数机类似的方式确定它是哪一枚硬币，并以美分返回它的价值，如清单 6-3 所示。

```    
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}
```
    
清单 6-3：一个枚举和一个以match枚举的变体作为其模式的表达式

让我们分解一下match函数中的value_in_cents。首先，我们列出match关键字，后跟表达式，在本例中为值 coin。这看起来和使用 with 的条件表达式很相似if，但是有很大的不同：使用 with if，条件需要计算为一个布尔值，但在这里它可以是任何类型。coin本例中的类型是Coin我们在第一行定义的枚举。

接下来是match武器。手臂有两部分：模式和一些代码。这里的第一个手臂有一个模式，它是值Coin::Penny，然后=> 是分隔模式和要运行的代码的运算符。本例中的代码就是值1。每个手臂与下一个手臂用逗号隔开。

当match表达式执行时，它会将结果值按顺序与每个臂的模式进行比较。如果模式与值匹配，则执行与该模式关联的代码。如果该模式与值不匹配，则继续执行下一个分支，就像在硬币分类机中一样。我们可以根据需要拥有任意数量的手臂：在清单 6-3 中，我们match有四个手臂。

与每个臂关联的代码是一个表达式，匹配臂中表达式的结果值是为整个表达式返回的值match。

如果匹配臂代码很短，我们通常不会使用花括号，就像清单 6-3 中每个臂只返回一个值一样。如果要在匹配臂中运行多行代码，则必须使用大括号，并且臂后面的逗号是可选的。例如，以下代码打印“Lucky penny!” 每次使用 a 调用该方法Coin::Penny，但仍会返回块的最后一个值1：

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}
```
    
### 1. 绑定到值的模式
    
匹配臂的另一个有用特性是它们可以绑定到与模式匹配的值的部分。这就是我们如何从枚举变体中提取值。

例如，让我们更改我们的枚举变体之一以在其中保存数据。从 1999 年到 2008 年，美国为 50 个州铸造了不同设计的 25 美分硬币。没有其他硬币有状态设计，因此只有 25 美分硬币具有这种额外价值。enum我们可以通过更改Quartervariant 以包含UsState存储在其中的值来将此信息添加到我们的，我们已在清单 6-4 中完成。

```  
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {}
```
    
示例 6-4：一个Coin枚举，其中Quartervariant 也有一个UsState值

假设一位朋友正试图收集所有 50 个州的 25 美分硬币。当我们按硬币类型对零钱进行分类时，我们还会说出与每个硬币相关的州名称，这样如果我们的朋友没有硬币，他们可以将其添加到他们的收藏中。

在此代码的匹配表达式中，我们添加了一个变量 calledstate来匹配 variant 的值的模式Coin::Quarter。当 a Coin::Quarter匹配时，state变量将绑定到该季度状态的值。然后我们可以state在该手臂的代码中使用，如下所示：

```
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
```
    
如果我们打电话value_in_cents(Coin::Quarter(UsState::Alaska))，coin 将会是Coin::Quarter(UsState::Alaska)。当我们将该值与每个匹配臂进行比较时，在我们到达之前它们都不匹配Coin::Quarter(state)。那时， 的绑定state将是 value UsState::Alaska。然后我们可以在表达式中使用该绑定，从而从的枚举变体println!中获取内部状态值。CoinQuarter

匹配Option<T>
在上一节中，我们希望在使用 ; 时T从 case 中获取内部值。我们也可以处理using ，就像我们处理枚举一样！我们将比较 的变体，而不是比较硬币，但表达式的工作方式保持不变。SomeOption<T>Option<T>matchCoinOption<T>match

假设我们要编写一个函数，它接受一个值Option<i32>，如果其中有一个值，则将该值加 1。如果里面没有值，该函数应该返回该None值并且不尝试执行任何操作。

这个函数非常容易编写，感谢match，它看起来像清单 6-5。

```
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```
match示例 6-5：一个使用表达式的函数Option<i32>

plus_one让我们更详细地检查 的第一次执行。当我们调用时 ，主体中的plus_one(five)变量将具有值。然后我们将其与每个匹配臂进行比较：xplus_oneSome(5)

```
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```
    
该Some(5)值与模式不匹配None，所以我们继续下一个分支：

```
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```
    
Some(5)匹配吗Some(i)？确实如此！我们有相同的变体。绑定i 到包含在中的值Some，因此i采用值5。然后执行 match arm 中的代码，因此我们将 1 的值加 1并使用我们的总计i创建一个新值。Some6

现在让我们考虑plus_one清单 6-5 中的第二次调用，其中xis None。我们输入match并与第一只手臂进行比较：

```
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```   
    
它匹配！没有要添加的值，因此程序停止并返回 None右侧的值=>。因为第一只手臂匹配，所以没有比较其他手臂。

组合match和枚举在许多情况下都很有用。你会在 Rust 代码中经常看到这种模式：match针对一个枚举，将一个变量绑定到里面的数据，然后基于它执行代码。一开始有点棘手，但一旦您习惯了它，您会希望它有所有语言版本。它一直是用户的最爱。

比赛是详尽无遗的
我们需要讨论的另一方面是match：武器的模式必须涵盖所有可能性。考虑我们plus_one函数的这个版本，它有一个错误并且无法编译：

此代码无法编译！
    
``` 
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```  
    
我们没有处理这个None案例，所以这段代码会导致一个错误。幸运的是，Rust 知道如何捕捉这个错误。如果我们尝试编译这段代码，我们会得到这个错误：

```
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0004]: non-exhaustive patterns: `None` not covered
 --> src/main.rs:3:15
  |
3 |         match x {
  |               ^ pattern `None` not covered
  |
note: `Option<i32>` defined here
  = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
4 ~             Some(i) => Some(i + 1),
5 ~             None => todo!(),
  |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `enums` due to previous error
```
    
Rust 知道我们没有涵盖所有可能的情况，甚至知道我们忘记了哪种模式！Rust 中的匹配是穷尽的：我们必须穷尽所有最后的可能性以使代码有效。特别是在 的情况下 Option<T>，当 Rust 防止我们忘记显式处理这种 None情况时，它会保护我们在可能为 null 时假设我们有一个值，从而使前面讨论的数十亿美元的错误成为不可能。

包罗万象的模式和_占位符
使用枚举，我们还可以对一些特定值执行特殊操作，但对所有其他值执行一个默认操作。想象一下，我们正在实现一个游戏，如果您在掷骰子时掷出 3，您的玩家不会移动，而是会得到一顶新的花式帽子。如果您掷出 7，您的玩家将失去一顶花哨的帽子。对于所有其他值，您的玩家将在游戏板上移动该数量的空格。这是一个match实现该逻辑的代码，掷骰子的结果是硬编码的而不是随机值，所有其他逻辑由没有主体的函数表示，因为实际实现它们超出了本例的范围：

```
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}
``` 
    
对于前两个臂，模式是字面值3和7。对于涵盖所有其他可能值的最后一个臂，模式是我们选择命名的变量other。为 arm 运行的代码other通过将变量传递给函数来使用该变量move_player。

即使我们没有列出 a u8可能具有的所有可能值，此代码也可以编译，因为最后一个模式将匹配所有未明确列出的值。match这种包罗万象的模式满足了必须详尽无遗的要求。请注意，我们必须将包罗万象的手臂放在最后，因为模式是按顺序评估的。如果我们早点放置 catch-all 手臂，其他手臂将永远不会运行，因此如果我们在 catch-all 之后添加手臂，Rust 会警告我们！

当我们想要一个包罗万象但又不想使用 包罗万象的模式中的值时，Rust 也有一个我们可以使用的模式：_是一个特殊的模式，它匹配任何值并且不绑定到那个值。这告诉 Rust 我们不会使用该值，因此 Rust 不会警告我们有未使用的变量。

让我们改变游戏规则：现在，如果您掷出的不是 3 或 7，则必须再次掷出。我们不再需要使用 catch-all 值，因此我们可以更改我们的代码以使用_而不是变量 named other：

```
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}
```
    
这个例子也满足了详尽性要求，因为我们明确地忽略了最后一个分支中的所有其他值；我们没有忘记任何事情。

最后，我们将再次更改游戏规则，这样如果您掷出 3 或 7 以外的任何东西，轮到您时不会发生任何其他事情。我们可以使用单位值（我们提到的空元组类型）来表示在“元组类型”部分）作为与手臂一起使用的代码_：

``` 
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}
```
    
在这里，我们明确地告诉 Rust，我们不会使用任何其他与早期 arm 中的模式不匹配的值，并且在这种情况下我们不想运行任何代码。

## 三. 简洁的控制流程if let

该if let语法使您可以将ifand组合let成一种不那么冗长的方式来处理与一个模式匹配的值，同时忽略其余模式。考虑清单 6-6 中的程序，它匹配变量Option<u8>中的一个值 config_max，但只想在该值是Some 变体时执行代码。

``` 
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}
```  
    
示例 6-6：Amatch只关心在值是时执行代码Some

如果值为，我们通过将值绑定到模式中的变量来Some打印出变体中的值。我们不想对该值做任何事情。为了满足表达式，我们必须在处理完一个变体后添加，这是添加烦人的样板代码。SomemaxNonematch_ => ()

相反，我们可以使用if let. match以下代码的行为与清单 6-6 中的相同：

``` 
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
``` 
    
语法if let采用由等号分隔的模式和表达式。它的工作方式与 a 相同match，其中表达式被赋予了 match并且模式是它的第一臂。在这种情况下，模式是 Some(max)，并且max绑定到 中的值Some。然后我们可以像在相应的手臂中使用的那样max在块的主体中​​使用。如果值与模式不匹配，则块中的代码不会运行。if letmaxmatchif let

使用if let意味着更少的打字、更少的缩进和更少的样板代码。但是，您失去了执行的详尽检查match。match在 和之间进行选择if let取决于您在特定情况下所做的事情，以及获得简洁性是否是失去详尽检查的适当权衡。

换句话说，您可以将其视为if leta 的语法糖match，它在值与一个模式匹配时运行代码，然后忽略所有其他值。

我们可以包含 an elsewith an if let。随附的代码块与 等同于 and 的表达式中的 case else随附的代码块相同。回想一下 清单 6-4 中的枚举定义，其中变量也包含一个 值。如果我们想计算我们看到的所有非四分之一硬币的数量，同时还要宣布四分之一的状态，我们可以使用 如下表达式来实现：_matchif letelseCoinQuarterUsStatematch

``` 
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}
``` 
    
或者我们可以使用if letandelse表达式，如下所示：

```   
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
```  
    
如果您遇到这样一种情况，即您的程序的逻辑过于冗长而无法使用 a 来表达match，请记住它if let也在您的 Rust 工具箱中。

## 四.概括
    
我们现在已经介绍了如何使用枚举来创建可以是一组枚举值之一的自定义类型。我们已经展示了标准库的Option<T> 类型如何帮助您使用类型系统来防止错误。当枚举值中包含数据时，您可以使用matchorif let来提取和使用这些值，具体取决于您需要处理的情况。

您的 Rust 程序现在可以使用结构和枚举来表达您领域中的概念。创建自定义类型以在您的 API 中使用可确保类型安全：编译器将确保您的函数仅获取每个函数期望的类型的值。

为了向您的用户提供一个组织良好且易于使用且仅准确公开您的用户需要的 API，现在让我们转向 Rust 的模块。   
    





