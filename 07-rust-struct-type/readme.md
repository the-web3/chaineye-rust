# 使用结构来构造相关数据

我最近在重新学rust ，巩固一下细节，也写一个Chaineye rust 教程，供想要学习的小伙伴查看。

推特：@seek_web3

Chainey 社群： 官网 chaineye.info | Chaineye Rust 教程 | 微信: LGZAXE, 加微信之后拉各社群

所有代码和教程开源在github: https://github.com/0xchaineye/chaineye-rust

-----------------------------------------------------------------------------------------------------------------------------------------------------------

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











