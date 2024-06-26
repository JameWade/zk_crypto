# Arithmetization II

这是我们 STARK Math系列的第三篇文章，如果您还没有读过第一篇和第二篇文章，建议您先读一下。友情提示：这篇文章比前两篇文章的数学内容更多一些。

# **Recap**

在上一篇文章中，我们介绍了算术化--将计算完整性（CI）声明转换为检查多项式是否为低度的过程。通过这种转换，我们可以实现简洁的验证，CI 声明的验证者所需的资源比单纯的重放所需的资源少得多。在上一篇文章中，我们以将科拉茨序列的 CI 语句转换为执行轨迹和多项式约束条件集为例，深入探讨了这种转换的第一步。在这篇文章中，我们将进一步展示——这次使用的是斐波那契数列--证明者如何将执行轨迹和多项式约束条件结合起来，从而得到一个多项式，这个多项式保证是低度的，当且仅当执行轨迹满足我们一开始提出的多项式约束条件时。此外，我们还将展示考虑如何使验证者能够简洁地评估多项式的 `度`(domain)。我们还将简要讨论纠错码如何在 STARK 中发挥作用。

我们将假设您熟悉有限群、有限域上的多项式以及本系列的前几篇文章。

# **Queries and Error Correction Codes**

回想一下，我们的目标是让验证者只需向证明者提出极少量的问题，就能决定接受还是拒绝证明，并保证较高的准确性。理想情况下，验证者希望要求证明者提供执行跟踪中几个（随机）位置的值，并检查这些位置的多项式约束是否成立。正确的执行轨迹自然会通过这一测试。然而，要构建一个完全错误的执行轨迹并不难，它只在一个地方违反了限制条件，并由此得出一个完全不同的结果。通过少量随机查询来识别这种故障是非常不可能的。 `这里说的是想通过随机检查的方式去验证`， `就是说在抽样样本无限大的情况下，对错很容易区分`

解决类似问题的常用技术是纠错码。

纠错码通过用较长的字符串替换原来的字符串，将一组字符串（其中一些字符串可能非常相似）转换成一组成对的，不同的字符串。

有趣的是，多项式可以用来构造良好的纠错码，因为两个度数为 d 的多项式，在比 d 大得多的域上求值，几乎处处不同。这种码被称为里德-所罗门（[Reed-Solomon](https://en.wikipedia.org/wiki/Reed%E2%80%93Solomon_error_correction) ）码。

利用这一点，我们可以将执行轨迹看作是在某个域上对多项式的求值，并在更大的域上对同一个多项式进行求值，从而扩展执行轨迹。以类似的方式扩展一个错误的执行轨迹，会得到一个完全不同的字符串，这反过来又使得验证者可以使用少量的查询来区分这些情况。

因此，我们的计划是：1）将执行轨迹重新表述为一个多项式；2）将其扩展到一个大域；3）利用多项式约束条件，将其转换为另一个多项式。

# **Toy Example: Boolean Execution Trace**

假设有关的 CI 语句是 "证明者有一个 512 个数字的序列，所有数字要么是 0 要么是 1"，我们希望通过读取大大少于 512 个数字来验证这一点。让我们来看看是什么样的执行轨迹和多项式约束表达了这个示例：

1. **执行轨迹有 512 行，每行包含一个单元格，单元格中要么为 0，要么为 1。**
2. 我们在此使用的多项式约束条件是  $Aᵢ⋅Aᵢ-Aᵢ=0$，其中 Aᵢ 表示该单列执行轨迹中的第 i 个单元格（当且仅当一个数字为 0 或 1 时，该数字等于它的平方)

为了用多项式重新表述这个执行轨迹，我们指定了我们将在其中工作的域——我们 $Z_{96769}$，模96769的加法和乘法整数 $0，1，...，96768$ 集合。接下来，我们选取 $Z_{96769}^*$（我们用 F* 表示 F 的乘法群）的一个子群 $G$，使得 $|G|=512$，以及 G 的某个生成元 $g$。由于 512 整除该群的大小（即 96768），因此保证了存在这样的子群。

现在，我们将执行轨迹中的元素视为对某个阶数小于 512 的多项式 f(x) 的求值，具体方法如下：第 i 个单元包含对生成元第 i 次幂的函数 f 的求值。即：

$\forall 0\leq i<512:f(g^i):=A_i$

这样一个最多 512 次幂的多项式可以通过插值法计算出来，然后我们在一个更大的域上对其进行评估，形成里德-所罗门码字的一个特例。

最后，我们使用这个多项式创建另一个多项式，其低度取决于执行跟踪上满足的约束。

为此，我们必须研究切线并讨论多项式的根。

## **Roots of Polynomials**

关于多项式及其根的一个基本事实是，如果 p(x) 是一个多项式，那么对于某个特定值 a，p(a)=0，当且仅当存在一个多项式 q(x) ，使得  $(x-a)q(x)=p(x)$并且 $deg(p)=deg(q)+1$ 时， $p(a)=0$。    `deg是多项式的度` ，因为x-a是一次多项式，所以q的最高次项乘以x得出的度是x+1次，等于p的度。

此外，对于所有 x≠a，我们可以通过计算求出 q（x）：

${p(x)\over(x-a)}$

通过归纳，类似的事实也适用于 k 根。也就是说，如果 aᵢ 在所有 i=0...k-1 中都是 p 的根，那么存在一个 $deg(p)-k$ 度的多项式 q，并且除了这 k 个值之外，多项式q完全等于

${p(x)\over\prod_{i=0}^{k-1}(x-a_i)}$

## **Putting It Together**

用 f 来重新表述多项式约束条件，可以得到下面的多项式：

$f(x)^2-f(x) = 0$

我们给 f 下了这样的定义：当且仅当执行轨迹中的单元格为 0 或 1 时，该表达式的根为 1、g、g²、......、g⁵¹¹。我们可以定义：

$p(x) ={ f(x)^2-f(x) \over\prod_{i=0}^{511}(x-g_i)}$

从上一段我们可以知道，当且仅当执行轨迹确实是一个 512 位的列表（即 0 或 1）时，存在一个度数至多为 2-deg(f)-512 的多项式，它在所有 $x ∉{1, g, g², ..., g⁵¹¹}$上与 p 一致。请注意，验证者早先已将执行轨迹扩展到一个更大的域，因此查询该域中的多项式值是很好定义的。 `仔细思考这段话，就是说存在一个多项式q，在 $x ∉{1, g, g², ..., g⁵¹¹}$上与p一致，这里 $x ∉{1, g, g², ..., g⁵¹¹}$是为了保证分母不为0` 这里对应这上面的例子

如果存在一种协议，证明者可以通过它让验证者相信这个多项式是低度多项式，从而验证者在协议中只询问执行轨迹之外的值，那么只有当 CI 语句为真时，验证者才会确信它的真实性。事实上，在下一篇文章中，我们将展示一个协议，它可以做到这一点，而且错误概率非常小。现在，让我们来看看另一个简单有效的例子，看看在这种情况下如何进行还原。

# **Fibonacci**

接下来，我们以正确计算 Z₉₆₇₆₉中的斐波纳契数列第 512 位为例。该序列的正式定义如下：

$a_0 = 1$

$a_1 = 1$

$a_{n+2} = (a_{n+1} + a_n) mod    96769$

我们只需写下所有 512 个数字，就能创建该 CI 语句的执行跟踪：

![Untitled](Arithmetization%20II%20f7e575e35037463aa7ab1826e4ffb7c5/Untitled.png)

我们使用的多项式约束条件是:

1. $A_0-1=0$
2. $A_1 -1 =0$
3. $\forall 0 \leq i<510: A_{i+2}-A_{i+1}-A_i=0$
4. $A_511-62215 = 0$

整个这一个序列就是状态机执行过程

## ***Translate to Polynomials***

在这里，我们也要定义一个最多 512 度的多项式 f(x)，这样执行轨迹中的元素就是 f 在某个生成器 g 的幂中的求值。

$\forall 0 \leq i<512: f\left(g^i\right):=A_i$

用 f 而不是 A 来表示多项式约束条件，我们可以得到:

1. $f(1) -1 =0$
2. $f(g) -1 = 0$
3. $\forall 0 \leq i<510: f\left(g^{i+2}\right)-f\left(g^{i+1}\right)-f\left(g^i\right)=0$  //Fibonacci recurrence relation
4. $f(g^511) - 62215 =0$

多项式的组合仍然是多项式--用 $f(gⁱ)$代替约束条件中的 $Aᵢ$ 仍然意味着这些是多项式约束条件。

请注意，1、2 和 4 是指 f 的单一值的约束条件，我们将它们称为 `边界约束条件`(*boundary constraints*) 。

相反，斐波那契递推关系(Fibonacci recurrence relation)则体现了整个执行轨迹的一系列约束条件，也可以表述为：

$\forall x \in\left\{1, g, g^2, \ldots, g^{509}\right\}: f\left(g^2 x\right)-f(g x)-f(x)=0$

 通过使用生成器来索引执行轨迹的行，我们可以将 "下一行 "的概念编码为一个简单的代数关系。如果 x 是执行轨迹中的某一行，那么 gx 就是下一行，g²x 是下一行之后的一行， $g^{-1}x$  是上一行，以此类推。

递推关系多项式： $f(g²x)-f(gx)-f(x)$ 对于执行轨迹中索引行的每个 x 都为零，最后两个除外。这意味着 1、g、g²、......、g⁵⁰⁹ 都是这个递推关系多项式的根（且它的度数最多为 510），因此我们可以如下构造 q(x)：

$q(x):=\frac{f\left(g^2 x\right)-f(g x)-f(x)}{\prod_{i=0}^{509}\left(x-g^i\right)}$

在stark中，这通常被称为 `组成多项式`(**composition polynomial**)。

事实上，当原始执行轨迹服从斐波那契递推关系时，除了这 510 个值外，该表达式与某个阶数至多为 2 的多项式一致（回顾一下，f 的阶数至多为 512）:*1, g, g², …, g⁵⁰⁹* 

不过，"组成多项式 "一词有些误导，因为当执行轨迹不满足多项式约束时，该表达式的求值在很多地方都与任何低度多项式不同。换句话说，只有当原始 CI 正确时，它才接近于低度多项式，而这正是我们的目标。

至此，所承诺的还原问题结束了，即把检查某些多项式约束是否满足某些执行轨迹的问题，转化为检查某些多项式（证明者已知）是否为低度多项式的问题。

# **Succinctness**

拥有非常高效的验证技术是 STARK 的关键所在，它可被视为由两部分组成——使用少量查询，并让验证者对每个查询进行少量计算。前者是通过纠错码实现的，它允许在极少数地方进行查询，而后者我们在这篇文章中一直都没有提及，直到现在。验证者的工作可以概括为：1）在随机位置查询组成多项式；2）根据这些查询检查低度多项式。低度简洁检查将在下一篇文章中讨论，但 "查询组成多项式 "究竟是什么意思呢？热心读者可能会对这种说法产生怀疑，这也是理所当然的。毕竟，证明者可能是恶意的。当验证者要求求出某个 x 处的组成多项式时，证明者可能会回答求出某个真正的低度多项式，这个多项式可以通过任何低度测试，但却不是组成多项式。

为了避免这种情况，验证程序在某一行 w 处明确查询斐波那契执行轨迹，询问三个地方的 f 值：f(w)、f(gw)、f(g²w)。

验证者现在可以通过以下方法计算出 w 处的组成多项式值：

$\frac{f\left(g^2 w\right)-f(g w)-f(w)}{\prod_{i=0}^{509}\left(w-g^i\right)}$

分子可以用从证明者那里得到的值来计算，而分母......嗯，这就是问题所在（被掩盖了）。

一方面，分母完全独立于执行轨迹，因此验证者可以在与证明者通信之前计算出分母。

一方面，分母完全独立于执行轨迹，因此验证者可以在与证明者通信之前计算出分母。

另一方面，在实际应用中，轨迹可能由成千上万行组成，计算分母会耗费验证者大量的运行时间。

这里的算术化对简洁性至关重要--因为如果注意到这一点，就可以非常高效地计算 g 的幂组成子群的特殊情况下的表达式：

$x^{|G|}-1 = \prod_{g\in G}(x-g)$

由于两边都是 |G| 阶多项式，其根恰好是 G 的元素，因此相等是正确的。////why

计算这个等式的右边似乎需要的运算次数与 |G| 成线性关系。然而，如果我们采用平方幂运算，计算这个等式的左侧所需的运行时间就是|G|的对数。

而斐波那契构成多项式的实际分母，可以通过将其改写为:

$\frac{f\left(g^2 w\right)-f(g w)-f(w)}{\prod_{i=0}^{509}\left(w-g^i\right)}=\frac{\left(w-g^{510}\right)\left(w-g^{511}\right) \cdot\left(f\left(g^2 w\right)-f(g w)-f(w)\right)}{w^{512}-1}$

这个看似技术性的问题是验证器能够在多对数时间内运行的核心所在，而之所以能够做到这一点，只是因为我们把执行轨迹看作是对场的某个子群的多项式的求值，而且有关的多项式约束条件在某个子群上是成立的。

类似的技巧也可应用于更复杂的执行轨迹，但关键是约束的重复模式要与场的某个子群相吻合。

# **More Constraints, More Columns!**

本帖中的示例刻意简单，以突出算术化的关键方面。一个自然而然的问题是：如何处理多列和多约束的情况。答案很简单：多列意味着有不止一个多项式需要处理，而多个组成多项式——在 STARK 的最后一个阶段，也就是低度检验阶段，会将多个约束条件产生的线性组合合并成一个多项式，即所有多项式的随机线性组合。当且仅当线性组合的所有成分都是低度时，线性组合才有可能是低度的。

# 总结

我们已经展示了在给定执行轨迹和约束多项式的情况下，验证者如何构造出一个低度多项式，该多项式的条件是且仅当原始 CI 语句成立。此外，我们还展示了验证者如何高效地查询这个多项式的值，确保证明者没有用某个虚假的低度多项式替换真实的多项式。

在下一篇文章中，我们将详细介绍低度测试，展示如何通过查询少量数值来判断某个多项式是否为低度。