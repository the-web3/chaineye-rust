# Rust 所有权

我最近在重新学rust ，巩固一下细节，也写一个Chaineye rust 教程，供想要学习的小伙伴查看。

推特：@seek_web3

Chainey 社群： 官网 chaineye.info | Chaineye Rust 教程 | 微信: LGZAXE, 加微信之后拉各社群

所有代码和教程开源在github: https://github.com/0xchaineye/chaineye-rust

-----------------------------------------------------------------------------------------------------------------------------------------------------------


所有权是 Rust 最独特的特性，对语言的其余部分有着深远的影响。它使 Rust 能够在不需要垃圾收集器的情况下确保内存安全，因此了解所有权的工作原理非常重要。在本章中，我们将讨论所有权以及几个相关的特性：借用、切片以及 Rust 如何在内存中放置数据。

## 一. 什么是所有权

所有权是一组控制 Rust 程序如何管理内存的规则。所有程序都必须管理它们在运行时使用计算机内存的方式。一些语言有垃圾收集，在程序运行时不断寻找不再使用的内存；在其他语言中，程序员必须显式分配和释放内存。Rust 使用第三种方法：内存通过所有权系统进行管理，该系统具有一组编译器检查的规则。如果违反任何规则，程序将无法编译。所有权的任何功能都不会在您的程序运行时减慢它的速度。

因为所有权对许多程序员来说是一个新概念，所以确实需要一些时间来适应。好消息是，您对 Rust 和所有权系统规则的经验越丰富，您就越容易自然地开发出安全高效的代码。坚持下去！

当您了解所有权时，您将为了解使 Rust 独一无二的功能打下坚实的基础。在本章中，您将通过一些专注于非常常见的数据结构的示例来学习所有权：字符串。

### 1. 栈和堆

许多编程语言不需要您经常考虑堆栈和堆。但是在像 Rust 这样的系统编程语言中，一个值是在堆栈上还是在堆上会影响语言的行为方式以及为什么你必须做出某些决定。本章稍后将结合堆栈和堆来描述所有权的部分，因此这里是准备工作的简要说明。

堆栈和堆都是代码在运行时可用的内存部分，但它们的结构不同。堆栈按获取值的顺序存储值，并以相反的顺序删除值。这称为后进先出。想想一堆盘子：当你添加更多盘子时，你把它们放在一堆盘子的顶部，当你需要一个盘子时，你从顶部取下一个。从中间或底部添加或移除盘子也不行！添加数据称为压入堆栈，删除数据称为弹出堆栈。存储在堆栈上的所有数据都必须具有已知的固定大小。编译时大小未知或大小可能发生变化的数据必须存储在堆上。

堆的组织性较差：当您将数据放在堆上时，您需要一定数量的空间。内存分配器在堆中找到一个足够大的空点，将其标记为正在使用，并返回一个指针，即该位置的地址。这个过程称为在堆上分配，有时缩写为just allocating（将值推入堆栈不被视为分配）。因为指向堆的指针是已知的、固定大小的，所以可以将指针存储在堆栈中，但是当您想要实际数据时，必须遵循指针。想想坐在餐厅里。当您进入时，您说明小组中的人数，工作人员会找到一张适合所有人的空桌子并带您到那里。如果您的团队中有人迟到，他们可以询问您的座位以找到您。

推送到堆栈比在堆上分配更快，因为分配器永远不必搜索存储新数据的位置；该位置始终位于堆栈的顶部。相比之下，在堆上分配空间需要更多的工作，因为分配器必须首先找到足够大的空间来保存数据，然后进行记账以准备下一次分配。

访问堆中的数据比访问堆栈中的数据要慢，因为您必须遵循指针才能到达那里。如果现代处理器在内存中跳跃更少，它们会更快。继续进行类比，考虑餐厅的服务器从许多桌子上接受订单。在转移到下一张桌子之前，在一张桌子上获得所有订单是最有效的。从 A 表订购，然后从 B 表订购，然后再从 A 订购，然后再从 B 订购，这将是一个慢得多的过程。出于同样的原因，如果处理器处理靠近其他数据（因为它在堆栈上）而不是更远（因为它可以在堆上）的数据，它可以更好地完成它的工作。在堆上分配大量空间也需要时间。

当您的代码调用函数时，传递给函数的值（可能包括指向堆上数据的指针）和函数的局部变量被推入堆栈。当函数结束时，这些值会从堆栈中弹出。

跟踪代码的哪些部分正在使用堆上的哪些数据，最大限度地减少堆上的重复数据量，以及清理堆上未使用的数据以免空间不足，这些都是所有权解决的问题。一旦了解了所有权，您就不需要经常考虑堆栈和堆，但是知道所有权的主要目的是管理堆数据可以帮助解释为什么它会以这种方式工作。

### 2. 所有权规则
首先，让我们看一下所有权规则。在我们处理说明它们的示例时，请牢记这些规则：

- Rust 中的每个值都有一个名为owner的变量。
- 一次只能有一个所有者。
- 当所有者超出范围时，该值将被删除。

### 3. 变量范围

现在我们已经了解了基本的 Rust 语法，我们不会fn main() { 在示例中包含所有代码，因此如果您继续学习，请确保main手动将以下示例放入函数中。因此，我们的示例将更加简洁，让我们专注于实际细节而不是样板代码。

作为所有权的第一个示例，我们将查看一些变量的范围。范围是项目在程序中有效的范围。取以下变量：

```
let s = "hello";
```

该变量s指的是字符串文字，其中字符串的值被硬编码到我们程序的文本中。该变量从声明它的那一刻起一直有效，直到当前作用域结束。清单 4-1 显示了一个带有注释的程序，其中注释了变量的s有效位置。

```
{                      // s is not valid here, it’s not yet declared
	let s = "hello";   // s is valid from this point forward

	// do stuff with s
}                      // this scope is now over, and s is no longer valid
```

#### 3.1. 一个变量及其有效范围

换句话说，这里有两个重要的时间点：

- 当s进入范围时，它是有效的。
- 它在超出范围之前一直有效。

此时，作用域和变量何时有效之间的关系与其他编程语言中的关系类似。String现在我们将通过引入类型来建立在这种理解之上。


### 3. 类型String_

为了说明所有权规则，我们需要一种比我们在第 3 章“数据类型”部分介绍的数据类型更复杂的数据类型。前面介绍的类型具有已知大小，可以存储在堆栈中并弹出当它们的范围结束时堆栈，并且如果代码的另一部分需要在不同的范围内使用相同的值，则可以快速而简单地复制以创建一个新的独立实例。但是我们想看看存储在堆上的数据，并探索 Rust 如何知道何时清理这些数据，类型String就是一个很好的例子。

String我们将专注于与所有权相关的部分。这些方面也适用于其他复杂数据类型，无论它们是由标准库提供还是由您创建。我们将String在 第 8 章进行更深入的讨论。

我们已经看到了字符串文字，其中一个字符串值被硬编码到我们的程序中。字符串文字很方便，但它们并不适合我们可能想要使用文本的所有情况。原因之一是它们是不可变的。另一个是在我们编写代码时并不是每个字符串值都可以知道：例如，如果我们想要获取用户输入并存储它怎么办？对于这些情况，Rust 有第二种字符串类型，String. 这种类型管理堆上分配的数据，因此能够存储我们在编译时未知的文本量。String您可以使用该函数从字符串文字创建一个from，如下所示：

```
let s = String::from("hello");
```

双冒号::运算符允许我们在类型下命名此特定from 函数String，而不是使用某种名称，如 string_from. 我们将在第 5 章的“方法语法”部分更多地讨论这种语法，当我们在第 7章的“模块树中引用项目的路径”中讨论模块的命名空间时。

这种字符串可以被改变：

```
let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
```

那么，这里有什么区别？为什么可以String变异但文字不能？区别在于这两种类型如何处理内存。

### 4. 内存和分配

在字符串文字的情况下，我们在编译时知道内容，因此文本直接硬编码到最终的可执行文件中。这就是字符串文字快速高效的原因。但这些属性仅来自字符串文字的不变性。不幸的是，我们不能为每段在编译时大小未知且在运行程序时其大小可能发生变化的文本在二进制文件中放入一块内存。

对于String类型，为了支持可变的、可增长的文本片段，我们需要在堆上分配一定数量的内存（编译时未知）来保存内容。这意味着：

必须在运行时从内存分配器请求内存。
当我们完成我们的String.
第一部分由我们完成：当我们调用时String::from，它的实现请求它需要的内存。这在编程语言中非常普遍。

但是，第二部分不同。在带有垃圾收集器 (GC)的语言中，GC 会跟踪并清理不再使用的内存，我们不需要考虑它。在大多数没有 GC 的语言中，我们有责任确定何时不再使用内存并调用代码显式释放它，就像我们请求它一样。正确地做到这一点在历史上一直是一个困难的编程问题。如果我们忘记了，我们就会浪费记忆。如果我们做得太早，我们就会有一个无效的变量。如果我们这样做两次，那也是一个错误。我们需要将 exactly oneallocate与 exactly one配对free。

Rust 采用不同的路径：一旦拥有它的变量超出范围，内存就会自动返回。下面是清单 4-1 中作用域示例的一个版本，使用 aString而不是字符串文字：

```
{
    let s = String::from("hello"); // s is valid from this point forward
    // do stuff with s
}   // this scope is now over, and s is no
    // longer valid
```

有一个自然点，我们可以将我们需要的内存返回String给分配器：当s超出范围时。当一个变量超出作用域时，Rust 会为我们调用一个特殊的函数。这个函数被称为 drop，它是作者String可以放置代码以返回内存的地方。Rustdrop在结束的大括号处自动调用。

```
注意：在 C++ 中，这种在项目生命周期结束时释放资源的模式有时称为资源获取即初始化 (RAII)。drop如果您使用过 RAII 模式，那么您将会熟悉 Rust 中的函数。
```

这种模式对 Rust 代码的编写方式有着深远的影响。现在看起来很简单，但在更复杂的情况下，当我们想让多个变量使用我们在堆上分配的数据时，代码的行为可能会出乎意料。现在让我们探讨其中的一些情况。

#### 4.1.与 Move 交互的变量和数据

在 Rust 中，多个变量可以以不同的方式与相同的数据交互。让我们看一下下面的示例代码

```
let x = 5;
let y = x;
```

我们大概可以猜到这是在做什么：“将值绑定5到x；然后复制其中的值x并将其绑定到y。” 我们现在有两个变量，x 和y，并且都等于5。这确实是正在发生的事情，因为整数是具有已知固定大小的简单值，并且这两个5值被压入堆栈。

现在让我们看一下版本String：

```
let s1 = String::from("hello");
let s2 = s1;
```

这看起来非常相似，所以我们可以假设它的工作方式是相同的：也就是说，第二行将复制值 ins1并将其绑定到s2. 但这并不完全是事实。

下图以了解String幕后发生的事情。AString由三部分组成，如左图所示：指向保存字符串内容的内存的指针、长度和容量。这组数据存放在栈中。右边是堆上保存内容的内存。

[![owner-1](https://github.com/0xchaineye/chaineye-rust/blob/main/images/owner-1.svg)](https://github.com/0xchaineye/chaineye-rust)

String长度是当前正在使用的内容的内存量（以字节为单位） 。String容量是从分配器接收到的内存总量，以字节为单位 。长度和容量之间的区别很重要，但在这种情况下并不重要，所以现在，忽略容量是可以的。

当我们分配s1给时s2，String数据被复制，这意味着我们复制了指针、长度和堆栈上的容量。我们不复制指针指向的堆上的数据。换句话说，内存中的数据表示如图下图所示。

[![owner-2](https://github.com/0xchaineye/chaineye-rust/blob/main/images/owner-02.svg)](https://github.com/0xchaineye/chaineye-rust)

该表示看起来不像下图，如果 Rust 也复制堆数据，内存会是什么样子。如果 Rust 这样做，s2 = s1如果堆上的数据很大，则该操作在运行时性能方面可能会非常昂贵。

[![owner-3](https://github.com/0xchaineye/chaineye-rust/blob/main/images/owner-03.svg)](https://github.com/0xchaineye/chaineye-rust)

早些时候，我们说过，当一个变量超出范围时，Rust 会自动调用该drop函数并清理该变量的堆内存。但图 2 显示两个数据指针指向同一位置。这是一个问题：当s2和s1超出范围时，它们都会尝试释放相同的内存。这被称为双重释放错误，是我们之前提到的内存安全错误之一。两次释放内存会导致内存损坏，这可能会导致安全漏洞。

为了确保内存安全，在行之后let s2 = s1;，Rust 认为s1不再有效。s1因此，Rust 在超出作用域时不需要释放任何东西。查看当您尝试使用s1after s2is created 时会发生什么；它不会工作：

```
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

你会得到这样的错误，因为 Rust 阻止你使用无效的引用：

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error
```

如果您在使用其他语言时听说过浅拷贝和深拷贝这两个术语，那么在不拷贝数据的情况下拷贝指针、长度和容量的概念可能听起来像是在制作浅拷贝。但是因为 Rust 也使第一个变量无效，而不是被称为浅拷贝，它被称为移动。在这个例子中，我们会说那s1 被移动到s2. 因此，实际发生的情况如下图所示。

[![owner-4](https://github.com/0xchaineye/chaineye-rust/blob/main/images/owner-4.svg)](https://github.com/0xchaineye/chaineye-rust)

那解决了我们的问题！只有s2有效，当它超出范围时，它会单独释放内存，我们就完成了。

此外，这还暗示了一个设计选择：Rust 永远不会自动创建数据的“深”副本。因此，可以假设任何自动 复制在运行时性能方面都是廉价的。

#### 4.2.与克隆交互的变量和数据

如果我们确实想深度复制 的堆数据String，而不仅仅是堆栈数据，我们可以使用一种称为clone. 我们将在第 5 章讨论方法语法，但是因为方法是许多编程语言的共同特征，所以您之前可能已经见过它们。

clone下面是实际方法的示例：

```
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

这工作得很好并且明确地产生了上图第三幅图中所示的行为，其中堆数据确实被复制了。

当您看到对 的调用时clone，您就知道正在执行某些任意代码并且该代码可能开销很大。这是一个视觉指示器，表明正在发生不同的事情。

#### 4.3.仅堆栈数据：复制

我们还没有谈到另一个问题。这段使用整数的代码（部分代码如清单 4-2 所示）有效且有效：

```
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

但是这段代码似乎与我们刚刚学到的相矛盾：我们没有调用 clone, 但x仍然有效并且没有被移入y.

原因是在编译时具有已知大小的整数等类型完全存储在堆栈中，因此可以快速复制实际值。这意味着我们没有理由x在创建变量后阻止其生效y。换句话说，这里的深拷贝和浅拷贝没有区别，所以调用clone和通常的浅拷贝没有什么不同，我们可以省略它。

Rust 有一个特殊的注释，称为Copy特征，我们可以将其放在存储在堆栈中的类型上，就像整数一样（我们将在第10 章中详细讨论特征）。如果一个类型实现了Copy 特征，使用它的变量不会移动，而是被简单地复制，使它们在分配给另一个变量后仍然有效。

Copy如果类型或其任何部分已经实现了特征，Rust 将不允许我们使用该类型进行注释Drop。如果类型需要在值超出范围时发生一些特殊的事情，并且我们将注释添加Copy到该类型，我们将得到一个编译时错误。要了解如何将Copy注释添加到您的类型以实现特征，请参阅附录 C 中的“可派生特征”。

那么，什么类型实现了这个Copy特征？您可以查看给定类型的文档以确保确定，但作为一般规则，任何一组简单标量值都可以实现Copy，并且任何需要分配或某种形式的资源都可以实现Copy。以下是一些实现的类型Copy：

- 所有整数类型，例如u32.
- 布尔类型 ，bool具有值true和false。
- 所有浮点类型，例如f64.
- 字符类型，char.
- 元组，如果它们只包含也实现Copy. 例如， (i32, i32)实现Copy，但(i32, String)不实现。


### 5.所有权和职能

将值传递给函数的机制类似于将值分配给变量时的机制。将变量传递给函数将移动或复制，就像赋值一样。下面代码有一个带有一些注释的示例，显示了变量进入和超出范围的位置。

```
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

s如果我们尝试在调用 之后使用takes_ownership，Rust 会抛出一个编译时错误。这些静态检查可以防止我们犯错。main尝试向该用途添加代码s，x看看您可以在哪里使用它们以及所有权规则阻止您这样做的地方。

### 6.返回值和范围

返回值也可以转移所有权。下面代码显示了一个返回一些值的函数示例，带有与上面代码中类似的注释。

```
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

变量的所有权每次都遵循相同的模式：将一个值赋给另一个变量会移动它。drop当包含堆上数据的变量超出范围时，除非数据的所有权已移至另一个变量，否则将清除该值。

虽然这可行，但获取所有权然后返回每个函数的所有权有点乏味。如果我们想让一个函数使用一个值但不取得所有权怎么办？很烦人的是，如果我们想再次使用我们传入的任何内容，除了我们可能还想返回的函数体产生的任何数据外，还需要传回。

Rust 让我们使用元组返回多个值

```
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```
但对于一个本应通用的概念来说，这是太多的仪式和大量的工作。对我们来说幸运的是，Rust 有一个使用值而不转移所有权的特性，称为引用。


## 二. 参考资料和借用

对于

```
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```
代码中元组代码的问题是我们必须将 返回 String给调用函数，这样我们仍然可以String在调用之后使用calculate_length，因为String被移到了 中 calculate_length。相反，我们可以提供对值的引用String。引用就像一个指针，因为它是我们可以遵循的地址来访问存储在该地址的数据；该数据由其他一些变量拥有。与指针不同，引用保证在该引用的生命周期内指向特定类型的有效值。

以下是您将如何定义和使用一个calculate_length函数，该函数将对对象的引用作为参数而不是获取值的所有权：

```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

首先，注意变量声明和函数返回值中的所有元组代码都不见了。其次，请注意我们传入&s1， calculate_length并且在其定义中，我们采用&String而不是 String。这些 & 符号代表引用，它们允许您引用某些值而无需取得它的所有权。下图描述了这个概念。

[![owner-5](https://github.com/0xchaineye/chaineye-rust/blob/main/images/owner-05.svg)](https://github.com/0xchaineye/chaineye-rust)

注意：与使用引用相反的&是取消引用，这是通过取消引用运算符完成的*。我们将在第 8 章看到解引用运算符的一些用法，并在第 15 章讨论解引用的细节。

让我们仔细看看这里的函数调用：

```
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

该&s1语法让我们创建一个引用但s1 不拥有它的值。因为它不拥有它，所以当引用停止使用时，它指向的值不会被删除。

同样，函数的签名用于&指示参数的类型s是引用。让我们添加一些解释性注释：

```
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
```

变量s有效的范围与任何函数参数的范围相同，但引用指向的值在s停止使用时不会被删除，因为s没有所有权。当函数将引用作为参数而不是实际值时，我们不需要返回值来归还所有权，因为我们从来没有所有权。

我们将创建引用的动作称为借用。就像在现实生活中一样，如果一个人拥有某样东西，你可以向他们借用。完成后，您必须将其归还。你不拥有它。

那么，如果我们尝试修改我们借用的东西会怎样？试试下面代码。剧透警告：它不起作用！

```
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```
这是错误：

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership` due to previous error
```

### 1. 可变引用

我们可以修复清上面的代码，让我们可以通过一些小的调整来修改借用的值，而不是使用可变引用：

```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

首先我们s变成mut. &mut s然后我们在调用函数的地方创建一个可变引用change，并更新函数签名以接受一个可变引用some_string: &mut String。这非常清楚该change函数将改变它借用的值。

可变引用有一个很大的限制：如果你有一个对一个值的可变引用，你就不能有对该值的其他引用。尝试创建两个可变引用的代码将s失败：

```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);

```

这是错误：

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership` due to previous error
```
这个错误表明这个代码是无效的，因为我们不能s一次多次借用可变的。第一个可变借用在r1并且必须持续到它在 中被使用println!，但是在创建该可变引用及其使用之间，我们尝试创建另一个可变引用以r2借用与 相同的数据r1。

防止同时对同一数据进行多个可变引用的限制允许以非常受控的方式进行突变。这是新 Rustaceans 苦苦挣扎的事情，因为大多数语言都允许您随时变异。有这个限制的好处是 Rust 可以在编译时防止数据竞争。数据竞争类似于竞争条件，并且在以下三种行为发生时发生：

- 两个或多个指针同时访问相同的数据。
- 至少有一个指针被用来写入数据。
- 没有用于同步访问数据的机制。

数据竞争会导致未定义的行为，并且当您试图在运行时跟踪它们时可能难以诊断和修复；Rust 通过拒绝编译带有数据竞争的代码来防止这个问题！

与往常一样，我们可以使用大括号来创建一个新范围，允许多个可变引用，而不是同时引用：

```
let mut s = String::from("hello");

{
	let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;

```

Rust 强制执行类似的规则来组合可变和不可变引用。此代码导致错误：

```
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

这是错误：

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error
```

哇！当我们有一个指向相同值的不可变引用时，我们也不能有一个可变引用。

不可变引用的用户不希望值突然从他们下面改变！但是，多个不可变引用是允许的，因为没有任何人正在读取数据，因此没有能力影响其他人对数据的读取。

请注意，引用的范围从引入它的地方开始，一直持续到最后一次使用该引用。例如，此代码将编译，因为不可变引用的最后一次使用println!发生在引入可变引用之前：

```
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

不可变引用的范围在最后一次使用它们之后结束r1，也就是在创建 可变引用之前。这些范围不重叠，因此允许使用此代码：编译器可以判断在范围结束之前的某个点不再使用该引用。r2println!r3

尽管借用错误有时可能令人沮丧，但请记住，这是 Rust 编译器提前指出潜在错误（在编译时而不是运行时）并向您准确显示问题所在。这样您就不必追查为什么您的数据与您想象的不一样。

### 3. 悬挂引用

在使用指针的语言中，很容易错误地创建一个悬空指针——一个引用内存中可能已提供给其他人的位置的指针——通过释放一些内存同时保留指向该内存的指针。相反，在 Rust 中，编译器保证引用永远不会成为悬挂引用：如果您有对某些数据的引用，编译器将确保数据不会在对数据的引用超出范围之前超出范围。

让我们尝试创建一个悬空引用，看看 Rust 如何防止它们出现编译时错误：

```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

这是错误：

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                 +++++++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ownership` due to previous error
```

此错误消息涉及我们尚未涵盖的功能：生命周期。我们将在第 10 章中详细讨论生命周期。但是，如果您忽略关于生命周期的部分，该消息确实包含此代码为何有问题的关键：

```
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

让我们仔细看看我们 dangle代码的每个阶段到底发生了什么：

```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

因为s是在里面创建的dangle，当 的代码dangle执行完， s就会被释放。但是我们试图返回对它的引用。这意味着该引用将指向一个无效的String. 那可不行！Rust 不会让我们这样做。

这里的解决方法是直接返回String：

```
fn main() {
    let string = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

这没有任何问题。所有权被移出，没有任何东西被释放。

### 4. 参考规则

让我们回顾一下我们讨论过的关于参考文献的内容：

- 在任何给定时间，您都可以拥有一个可变引用或任意数量的不可变引用。
- 引用必须始终有效。

## 三.切片类型

切片使您可以引用集合中连续的元素序列，而不是整个集合。切片是一种引用，因此它没有所有权。

这里有一个小的编程问题：编写一个函数，接受一串由空格分隔的单词，并返回它在该字符串中找到的第一个单词。如果函数在字符串中没有找到空格，则整个字符串必须是一个单词，因此应返回整个字符串。

让我们研究一下如何在不使用切片的情况下编写此函数的签名，以了解切片将解决的​​问题：

```
fn first_word(s: &String) -> ?
```

该first_word函数有一个&String作为参数。我们不想要所有权，所以这很好。但是我们应该返回什么？我们真的没有办法谈论字符串的一部分。但是，我们可以返回单词结尾的索引，用空格表示。让我们试试看，如清单下面代码所示。

```
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {}
```

因为我们需要逐个String元素遍历并检查值是否为空格，所以我们将String使用该方法将我们转换为字节数组 as_bytes。

```
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {}
```
接下来，我们使用以下方法在字节数组上创建一个迭代器iter：

```
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {}
```

我们将在第 13 章更详细地讨论迭代器。现在，知道这iter是一个返回集合中每个元素的方法，它enumerate包装了结果iter并将每个元素作为元组的一部分返回。从中返回的元组的第一个元素 enumerate是索引，第二个元素是对该元素的引用。这比我们自己计算索引要方便一些。

因为该enumerate方法返回一个元组，所以我们可以使用模式来解构该元组。我们将在第 6 章中更多地讨论模式。在for循环中，我们i 为元组中的索引和&item元组中的单个字节指定一个模式。因为我们从 中获得了对元素的引用，所以我们在模式中.iter().enumerate()使用 。&

在循环内部for，我们使用字节字面量语法搜索表示空格的字节。如果我们找到一个空间，我们就返回这个位置。否则，我们使用 返回字符串的长度s.len()。

```
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {}
```

我们现在有办法找出字符串中第一个单词结尾的索引，但是有一个问题。我们usize自己返回 a，但它只是在 的上下文中有意义的数字&String。换句话说，因为它是一个独立于 的值String，所以不能保证它在将来仍然有效。考虑下面的的程序，它使用了上面的 first_word 函数

```
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

这个程序编译没有任何错误，如果我们word 在调用s.clear(). 因为 根本word没有连接到状态，所以仍然包含值。我们可以将该值与变量一起使用以尝试提取第一个单词，但这将是一个错误，因为自从我们保存在.sword55ss5word

不得不担心索引word与数据不同步 s是一件乏味且容易出错的事情！如果我们编写一个函数，管理这些索引会更加脆弱second_word。它的签名必须如下所示：

```
fn second_word(s: &String) -> (usize, usize) {
```

现在我们正在跟踪一个开始和结束索引，我们有更多的值是根据特定状态的数据计算的，但与该状态完全无关。我们有三个不相关的变量浮动，需要保持同步。

幸运的是，Rust 有解决这个问题的方法：字符串切片。

### 1. 字符串切片

字符串切片是对 a 的一部分的引用String，它看起来像这样：

```
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}
```

不是对整个 的引用String，而是对额外位中指定的hello的一部分的引用。我们通过指定 ，使用方括号内的范围创建切片，其中是切片中的第一个位置，比切片中的最后一个位置多一个。在内部，切片数据结构存储切片的起始位置和长度，对应于minus 。因此，在 的情况下，将是一个切片，其中包含指向索引 6 处字节的指针，长度值为。String[0..5][starting_index..ending_index]starting_indexending_indexending_indexstarting_indexlet world = &s[6..11];worlds5

用图表显示了这一点。

[![owner-6](https://github.com/0xchaineye/chaineye-rust/blob/main/images/owner-06.svg)](https://github.com/0xchaineye/chaineye-rust)

使用 Rust 的..范围语法，如果你想从索引 0 开始，你可以删除两个句点之前的值。换句话说，它们是相等的：

```
#![allow(unused)]
fn main() {
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
}
```

出于同样的原因，如果您的切片包含 的最后一个字节String，您可以删除尾随数字。这意味着它们是相等的：

```
#![allow(unused)]
fn main() {
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
}
```

您也可以删除这两个值以获取整个字符串的一部分。所以这些是相等的：

```
#![allow(unused)]
fn main() {
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
}
```

注意：字符串切片范围索引必须出现在有效的 UTF-8 字符边界处。如果您尝试在多字节字符的中间创建一个字符串切片，您的程序将退出并出错。为了介绍字符串切片，我们在本节中仅假设 ASCII；第 8 章的“使用字符串存储 UTF-8 编码文本”部分对 UTF-8 处理进行了更深入的讨论。

考虑到所有这些信息，让我们重写first_word以返回一个切片。表示“字符串切片”的类型写为&str：

```
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {}
```

我们通过查找第一次出现的空格，以与示例 4-7 中相同的方式获取单词结尾的索引。当我们找到一个空格时，我们返回一个字符串切片，使用字符串的开头和空格的索引作为起始和结束索引。

现在，当我们调用 时first_word，我们会返回一个与基础数据相关联的单一值。该值由对切片起点的引用和切片中的元素数组成。

返回切片也适用于second_word函数：

```
fn second_word(s: &String) -> &str {
```

我们现在有一个更难搞砸的简单 API，因为编译器将确保对 的引用保持String有效。还记得清单 4-8 中程序中的错误吗？当我们得到第一个单词末尾的索引但随后清除了字符串，因此我们的索引无效？该代码在逻辑上是不正确的，但没有立即显示任何错误。如果我们继续尝试使用带有空字符串的第一个单词索引，问题会在稍后出现。切片使这个错误成为不可能，并让我们更快地知道我们的代码有问题。使用 的 slice 版本first_word会抛出一个编译时错误：

```
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

这是编译器错误：

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 |
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error
```

回忆一下借用规则，如果我们有一个对某物的不可变引用，我们就不能同时使用一个可变引用。因为clear需要截断String，所以需要获取可变引用。println! 在的调用之后使用clear中的引用word，因此不可变引用此时必须仍然处于活动状态。Rust 不允许 in 的可变引用clear和 from 的不可变引用word同时存在，编译失败。Rust 不仅使我们的 API 更易于使用，而且还消除了编译时的一整类错误！

### 1.1. 作为切片的字符串文字

回想一下，我们讨论过将字符串文字存储在二进制文件中。了解了切片之后，我们就可以正确理解字符串字面量了：

```
#![allow(unused)]
fn main() {
let s = "Hello, world!";
}
```
这里的类型s是&str：它是一个指向二进制文件特定点的切片。这也是字符串文字不可变的原因；&str是一个不可变的引用。


### 1.2. 字符串切片作为参数

知道您可以获取文字和String值的切片后，我们对 进行了另一项改进first_word，这就是它的签名：

```
fn first_word(s: &String) -> &str {
```
&String更有经验的 Rustacean 会改写清单 4-9 中所示的签名，因为它允许我们对值和值使用相同的函数&str。

```
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```
示例 4-9：通过对参数first_word类型使用字符串切片来改进函数s

如果我们有一个字符串切片，我们可以直接传递它。如果我们有一个String，我们可以传递一个 的切片String或一个对 的引用String。这种灵活性利用了deref 强制，我们将在第 15 章的“使用函数和方法的隐式 Deref 强制”部分介绍该功能 。

定义一个函数来获取字符串切片而不是对 a 的引用String 使我们的 API 更加通用和有用，而不会丢失任何功能：

文件名：src/main.rs

```
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

### 1.2.其他切片

正如您想象的那样，字符串切片是特定于字符串的。但也有更通用的切片类型。考虑这个数组：

```
#![allow(unused)]
fn main() {
let a = [1, 2, 3, 4, 5];
}
```
正如我们可能想要引用字符串的一部分一样，我们可能想要引用数组的一部分。我们会这样做：

```
#![allow(unused)]
fn main() {
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
}
```
此切片的类型为&[i32]. 通过存储对第一个元素的引用和长度，它的工作方式与字符串切片相同。您将对各种其他集合使用这种切片。我们将在第 8 章讨论向量时详细讨论这些集合。


### 四. 概括

所有权、借用和切片的概念确保 Rust 程序在编译时的内存安全。Rust 语言让您可以像其他系统编程语言一样控制内存使用，但是让数据所有者在所有者超出范围时自动清理该数据意味着您不必编写和调试额外的代码得到这个控制权。

所有权会影响 Rust 的许多其他部分的工作方式，因此我们将在本书的其余部分进一步讨论这些概念。让我们继续学习，看看如何将数据片段组合在一个struct.


