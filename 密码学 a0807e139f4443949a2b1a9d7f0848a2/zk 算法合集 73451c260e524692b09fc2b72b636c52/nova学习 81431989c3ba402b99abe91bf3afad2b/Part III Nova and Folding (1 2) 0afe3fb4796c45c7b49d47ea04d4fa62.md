# Part III: Nova and Folding (1/2)

# 回顾

1. 先引入snark
2. 
3. 讲了增量可验证计算IVC。在计算的一连串步骤中，每一步不仅要接收上一步的输出，对其执行运算，并将结果作为输入输入到下一步，还要接收上一步的证明（断言包括上一步在内的所有步骤都是正确的）。它还会为下一步提供一个证明，证明上一步提供给它的证明是正确的，而且这一步本身也是正确执行的。

1. 每一步都要将上一步证明的验证电路添加到下一步要证明的语句中。这要么会导致一个标量因子 `scalar factor`的爆炸（电路呈指数增长）。累积 `accumulate`方案提供了一条出路，它允许我们将每个验证步骤中代价高昂的部分推迟到稍后进行。

nova是在accumulate的基础上做了优化，用fold技术，折叠全部证明，而不是只积累昂贵的部分。将两个步骤的验证简化为一个步骤的验证。

# **What is a computation?**

对于给定的计算 𝐶，我们声称我们知道一个输入，在正确执行计算后，我们会得到某个公共输出 𝒴。我们将把输入和执行轨迹（涵盖正确执行的概念）称为见证𝑤。除见证外，计算中可能还有其他公共输入 x。因此，在给定公共输入 x 和输出𝒴 的情况下，我们希望证明我们知道有一个 `witness` 𝑤 来完成整个过程。

计算可以是一连串的算术运算（加法、乘法等），也可以涉及条件语句等更复杂的结构。我们必须定义计算的含义以及如何表示计算。

最简单的表达方式是使用伪代码或某种高级编程语言（Python、C++ 等）。然而，这并不适合我们之后要做的事情，即获得一个简洁的非交互式证明。事实证明，我们总能将计算转化为算术可满足性问题。因此，给定公共输入 x 和公共输出 𝒴（你可以把它们想象成有限域上长度为 n 和 m 的向量，比如整数元素的序列，模数为素数 p），以及 n+k 个变量的 m 阶多项式系统 𝐶，我们想证明我们知道一个证人 𝑤（有限域上长度为 k 的向量），使得

$𝐶 (x, 𝑤) = y$

在 x = (x₁,..., xₙ) 和 𝑤 = (𝑤₁,..., 𝑤ₖ)组成的 n + k 元组处对 m 多项式集合 𝐶 进行求值，得到 m 元组 𝒴 = (𝒴₁,..., 𝒴ₘ)。这种表示法更适于创建证明。适于证明的表示法有许多变种，称为中间表示法。下面我们将集中讨论一种流行的中间表示法，即 **rank-1 constraint system**（R1CS）。

提出好的中间表示法并将计算机程序转化为这些表示法本身就是一门艺术。对于某些计算来说，这是立竿见影的。例如，如果计算的是矩阵乘法，我们只需将对矩阵项进行的运算写成多项式即可。对于其他计算，例如条件语句，这将更加棘手。举个例子，假设我们检查一个位 b，根据它是 0 还是 1，让变量 c 成为 a² 或 a³：

```rust
if b == 0 then c = a²;
else c = a³;
```

用多项式表示，我们可以将其改写为

$c = (1-b) * a² + b * a³$
关于将计算机程序转化为中间表示法的细节，我们将留待下一篇文章讨论。也可以先看这个：V神写的[**Quadratic Arithmetic Programs: from Zero to Hero**](Halo%20and%20Accumulation%2087e35b7add7d447ab19e7f26d43132ef.md)

# **Rank 1 Constraint Systems (R1CS)**

让我们回到中间表示 R1CS（又称 QAP - 二次算术程序）的定义上来。

这些是特定形状的算术电路（多项式方程）：

我们将常数 1、公共输入向量 $x = (x₁,......,xₙ)$、公共输出向量 $𝒴 = (𝒴₁,......, 𝒴ₘ)$和witness $𝑤 = (𝑤₁,......, 𝑤ₖ)$ 合并为单一向量

$z = (z₁,…, zᵣ) = (1, x₁,…, xₙ, y_1,...y_m, 𝑤₁,…, 𝑤ₖ)$

`注意：我们要求 z₁ = 1，r = 1 + n + m + k。`

我们只允许将 $z₁,...,zᵣ$ 中的二次（2 度）多项式作为定义算术电路的多项式集，因此称为二次算术程序。由于我们将公共输出𝒴ᵢ 移到了向量 z 中，因此我们设置右边等于零。因此，系统中的每个多项式看起来都是这样的

$A \cdot z_1^2+B \cdot z_1z_2+…(all \ degree \ 2 \ terms)  \\ + a \cdot z_1 + b \cdot z_2+ ...(all degree 1 terms) =0\\+const.(degree 0 terms)$

由于 z₁ = 1，我们实际上不需要常数（零度）项。

在秩 1 约束系统中，只允许特定类型的二次多项式  `参照下面的例子，其实就是把高次幂全部拆成一次和二次的`。也就是说，限制二次项（2 级）的表达式为

$(a₁ • z₁ + … + aᵣ • zᵣ) • (b₁ • z₁ + … + bᵣ • zᵣ)$

这就是秩 1 名称的由来。将线性项移至右侧，我们可以得到以下形式的方程

$(a₁ • z₁ + … + aᵣ • zᵣ) • (b₁ • z₁ + … + bᵣ • zᵣ) = c₁ • z₁ + … + cᵣ • zᵣ$

系数是 $a₁，...，aᵣ，b₁，...，bᵣ，c₁，...，cᵣ$。这些系数中的大部分将假定为零，因此多项式是稀疏的。

请注意，这只是多项式之一。为了定义运算电路，我们允许使用多个这种形式的多项式：

$(aᵢ,₁ • z₁ + … + aᵢ,ᵣ • zᵣ) • (bᵢ,₁ • z₁ + … + bᵢ,₁ • zᵣ) + (cᵢ,₁ • z₁ + … + cᵢ,₁ • zᵣ) = 0$

# **Notation and terminology, instances and witnesses**

术语：

A bit of terminology and notation before we move on: A particular choice of public input and output (a choice of *x*₁*,…, x*ₙ*,* 𝒴₁*,…,* 𝒴ₘ) is called an instance (we will sometimes denote it by *I*) and this instance is said to be satisfied by a corresponding witness *W* in case all quadratic equations hold. `满足约束的输入和输出被叫做instance`

因此，我们将以 (1, I, W) 的形式来书写向量 z。系数 aᵢ、bⱼ、cₖ 可以说是定义了一个结构，与输入/输出相对应的变量可以看作是给出各种实例的参数。我们不需要写出线性组合（记住有多个线性组合，每个方程一个），而是用以下符号表示

$aᵢ,₁ • z₁ + … + aᵢ,ᵣ • zᵣ, bᵢ,₁ • z₁ + … + bᵢ,₁ • zᵣ$ and $cᵢ,₁ • z₁ + … + cᵢ,₁ • zᵣ$ by $A(Z), B(Z) and C(Z)$ respectively. 因此，A 将一个向量 Z 映射成一个向量，与方程的各种线性组合相对应。如果你喜欢矩阵符号，可以将 A 视为一个矩阵，其中每一行都由其中一个方程的系数（a₁, ..., aᵣ）给出， `而 A(Z) 实际上是这个矩阵与 z 的列向量 $z_i$相乘`(这里结果A（Z）应该是一个列向量)。 `由于 A 由一系列线性组合组成，因此它是线性的`，即我们有: 

$A (u • Z + v • Z’) = u • A(Z) + v • A(Z’)$
B,C同理。 `上面这个线性组合是通过拉格朗日插值完成`

找了个例子 [https://snowolf0620.xyz/index.php/zkp/435.html](https://snowolf0620.xyz/index.php/zkp/435.html)  还是V神之前那个例子好像

```rust
def qeval(x):
    y=x**3
    return y+x+5
```

先对每一个逻辑门都做了 `r1cs`处理：

![Untitled](Part%20III%20Nova%20and%20Folding%20(1%202)%200afe3fb4796c45c7b49d47ea04d4fa62/Untitled.png)

最终得到的矩阵：

![Untitled](Part%20III%20Nova%20and%20Folding%20(1%202)%200afe3fb4796c45c7b49d47ea04d4fa62/Untitled%201.png)

接着完成 r1cs到QAP

`例子中是将每个向量组的四个长度为 6 的行向量转换成六个长度为 4 的向量（六个阶为 3 的多项式）`

以向量组 A 举例，首先需要求出四个约束所对应的每个a 向量的第一个值的多项式，也就是对向量组 A 的第一列采用拉格朗日插值法，求过点 $(1,0),(2,0),(3,0),(4,5)$ 四个点的多项式（可以用在线工具 [planetcalc](https://zh.planetcalc.com/8692/) 求解，或者自己写脚本也行），求得三阶多项式如下

$y_1 = -5+9.166x-5x^2+0.833x^3$

最终转化的矩阵

![Untitled](Part%20III%20Nova%20and%20Folding%20(1%202)%200afe3fb4796c45c7b49d47ea04d4fa62/Untitled%202.png)

R1CS是向量内积

QAP则是用多项式

###############

现在 A(Z)、B(Z) 和 C(Z) 都是向量（由各种线性组合组成），我们可以将方程改写为

$A(Z) ○ B(Z) = C(Z)$

其中，A(Z) ○ B(Z) 表示向量 A(Z) 和 B(Z)的分量乘积，即它是一个向量，其第 i 项是 A(Z) 的第 i 项与 B(Z) 的第 i 项的乘积。顺便提一下，这种运算也被称为哈达玛乘积 `Hadamard product`。

最后，如果我们想将 Z 分离成实例 I 和证人 W 部分，我们可以将其重写为

$A(1, I, W) ○ B(1, I, W) = C(1, I, W)$

对于 IVC，当我们遍历同一个函数 F 时，每一步的结构都将保持不变，因此每一步的多项式系数 aᵢ、bⱼ、cₖ 都将相同。输入和输出将发生变化，从而产生不同的实例。

计算的 `每一步`都是上述类型的二次多项式方程组。每一步的系数都是相同的，但输入和输出都会发生变化，每次都会产生一个新的实例 I ，它将有一个证人 W，我们必须为其提供证明。 `这里是说累积算法的函数F相同，所以每次执行F就是一步，所以每一步的系数都是相同的，但是由于执行函数的输入输出不同，所以实例就会不同，那么证明就不同`

既然知道了这些步骤，我们终于可以谈谈如何将它们组合（折叠）在一起了。

# **The core idea: folding**

假设我们要折叠两个结构相同的 R1CS 实例。让我们来解开这个问题。

假设我们用一个 R1CS 结构，即一个特定形式的二次方程组，来表示一步计算：

$A(Z) ○ B(Z) = C(Z)$

这种计算有两个步骤（输入和输出不同），这意味着我们有两个实例。.因此，有一些实例 I₁和 I₂，证明者有证人 W₁ 和 W₂。证明者希望让验证者相信，它知道证人 W₁ 和 W₂，从而满足以下两个系统（每个系统验证一个步骤，对应一个不同的实例，即输入-输出）的要求：

$A(1, I_1, W_1) ○ B(1, I_1, W_1) = C(1, I_1, W_1)$

$A(1, I_2, W_2) ○ B(1, I_2, W_2) = C(1, I_2, W_2)$

现在，折叠的理念是证明者应该能够提出 `新的实例 I 和相应的证人 W`（对于相同的 R1CS 结构，即相同的 A、B、C），这样，如果证明者能够让验证者相信它知道实例 I 的证人 W，那么验证者就会相信证明者知道实例 I₁ 和 I₂ 的证人 W₁ 和 W₂。因此，从某种意义上说，证明者为同一个计算（实例 I）提供了假想的输入和输出，如果证明者能让验证者相信它有一些证人，并以连接人工输入和输出的方式正确执行了计算步骤，验证者就会相信两个初始的真实输入输出对也是如此。

# **If life were easy…**

为了理解折叠背后的原理，我们不妨简单地假设方程是线性的，而且只有一个（如果有多个方程，只需对每个方程应用相同的推理即可）。因此，我们有这样一个方程

$a₁ • z₁ + … + aᵣ • zᵣ = c₁ • z₁ + … + cᵣ • zᵣ$

或者，将所有术语集中在一边

$D(z) = d₁• z₁ + … + dᵣ • zᵣ = 0 \ for \ some \ d₁, …, dᵣ$

为简单起见，再次假设 d₁ = 0，实例长度为 2（输入和输出长度均为 1），因此它们是 (x, y) 形式，证人长度为 1，因此它们由一个字段元素𝑤 组成。所以 r = 4。假设有两个实例 (x, y) 和 (x', y')，它们有相应的证人𝑤 和𝑤'，因此  `这里没看懂`

$d₂ • x + d₂ • y + d₃ • 𝑤 = 0 \ \ and \ \ d₂ • x’ + d₂ • y’ + d₃ • 𝑤’ = 0$

我们想把这两个实例折叠成一个，这样，能够为折叠后的实例提供证人知识证明的证明者就能让验证者相信，它也能为两个初始实例提供证明。为此，我们利用了随机性的力量：验证者将向证明者提出挑战，要求证明它知道两个实例随机线性组合的证人。因此，验证者将发送一个随机 R，并要求证明者证明它知道输入 $x + R \cdot x'$ 和输出 $y + R \cdot y'$，即实例 $（x + R \cdot x'，y + R \cdot y'）$的见证人。

***为什么知道证人𝑤 和𝑤'的诚实证明者可以解决这个难题？***

这仅仅是因为方程的线性关系。相同的线性组合见证将是实例的线性组合见证。要看到这一点，我们需要检查:

$d₂ • (x + R • x’) + d₂ • (y + R • y’) + d₃ • (𝑤 + R + 𝑤’) = 0$

重新排序后，变成

$d₂ • x + d₂ • y + d₃ • 𝑤 + R • (d₂ • x’ + d₂ • y’ + d₃ • 𝑤’) = 0 + R • 0 = 0$

或者，由于 D(Z) 是线性的，我们有

$D((x, y, 𝑤) + R • (x’, y’, 𝑤’)) = D(x, y, 𝑤) + R • D(x’, y’, 𝑤’) = 0 + R • 0 = 0$

这一特性被称为完备性。 `简单点说就是证明者知道正确解，就一定会让验证者相信就是完备性`

***为什么不知道证明𝑤 和 𝑤'的不诚实证明者不能解决这个难题呢？***

这还是因为线性关系！如果证明者可以回答随机挑战，那么它就可以针对多个实例（针对多个随机 R₁、R₂......）这样做。然后，证明者可以从几个相应的证人𝑤₁、𝑤₂......中找到证人𝑤和𝑤'。因此，我们可以从挑战的解答中提取出初始实例的见证。利用这个不诚实的证明者，实际上就可以黑进系统！

这一特性被称为知识健壮性。 `证明者不知道解，验证者能分辨叫健壮性。`

# **… but life is not always easy**

我们已经看到了如何在一个良好的线性世界中进行折叠。增加输入输出并不会增加多少难度，只会增加更多的符号。

增加更多方程也可以通过单独处理所有方程（所有方程都是线性的）来进行类似处理。但遗憾的是，我们能用线性约束表达的内容非常有限，而一元二次方程（尤其是 R1CS）并不是线性的。和的平方不等于平方之和（问问毕达哥拉斯就知道了！）。

此外，如果 a₁ ≠ 0，那么我们还必须跟踪变量 z₁ 的变化情况。特别是，z₁ = 1 和 z'₁ = 1 得到 $z₁ + R \cdot z'₁ = 1 + R$ 的线性组合，这不满足 R1CS 约束条件。  `这个挺重要的，常数项是有影响的`

# **Just relax!**

但也并非全盘皆输。Kothapalli、Setty 和 Tzialla 证明了如何放宽 R1CS 约束的定义（他们称之为宽松的 R1CS）来挽救这一点。

我们有

$A(Z)○B(Z) = A(Z_1 + r • Z_2) ○ B(Z_1 + r • Z_1)  \\= A(Z_1) ○ B(Z_1) +r(A(Z_1)○B(Z_2)+A(Z_2)○B(Z_1))+r^2(A(Z_2)○B(Z_2))$

因此，问题在于

- 第三项中的系数 R² 应该是 R；
- 不应出现混合项 R（A(Z) ○ B(Z')+A(Z')○ B(Z)）
- 我们希望放宽第一项 z₁ 应为 1 的条件。

通过在实例中添加额外的参数，可以实现松弛。也就是说，我们在 R1CS 结构前增加了一个新的因子 u，以吸收额外的 R 因子，并增加了一个误差（"松弛"）向量，以考虑混合项。

因此：

- 宽松的 R1CS 结构通常由 A、B 和 C 组成；
- R1CS 实例 I 由公共输入 x = (x₁,..., xₙ)、𝒴 = (𝒴₁,..., 𝒴ₘ)组成，将通过添加标量 u 和误差向量 E 来宽松；
- 矢量 Z 放宽为（u、I、W），允许 u 代替 1 作为第一个条目；以及
- 方程结构放宽为 $A(Z) ○ B(Z) = u \cdot C(Z) + E$。

现在，如果 W₁ 是宽松 R1CS 实例  $I₁, E₁, u₁$ 的见证者，而 W₂ 是宽松 R1CS 实例 $I_2, E_2, u_2$ 的见证者、我们确实可以取实例 I₁ 和 I₂ 的线性组合 $I = I₁ + R - I₂$，如果我们对实例 I 的 u 和 E 进行适当定义，则 $W = W₁ + R - W₂$ 将满足这一要求。对于 $Z = (u₁, I₁, W₁)$和 $Z' = (u₂, I₂, W₂)$，折叠时获得 新标量和松弛向量 `new scalar and slack vector`的正确方法如下：

$u = u₁ + R • u₂$

$E = E₁ + r • (A(Z_1) ○ B(Z_2) + A(Z_2) ○ B(Z_1) — u₁C(Z_2)— u₂C(Z_1)) + r^2 • E_2$

利用宽松向量，我们使特定的二次表达式（R1CS）表现得像线性表达式一样。我们让读者自己检查折叠是否真的有效。关于更多细节、完整性和知识合理性，请参阅原始论文。

# 定义

1. 首次构造——R1CS构造
    
    prover发送 $W_1$和 $W_2$,verifier随机选择 $r \in F$作为回复
    
    $x=x_1+r \cdot x_2$
    
    $W=W_1+ r \cdot W_2$
    
    计算新的instance-witness对： $(x,W)$
    
    此时对于 $Z_1=(W_1,x_1,1)$、 $Z_2 = (W_2,x_2,1)$ 、 $Z=(W,x,1)$有
    
    $A(Z)○B(Z) = A(Z_1 + r • Z_2) ○ B(Z_1 + r • Z_2)  \\= A(Z_1) ○ B(Z_1) +r(A(Z_1)○B(Z_2)+A(Z_2)○B(Z_1))+r^2(A(Z_2)○B(Z_2)) \\ \neq C(Z)$
    
2. 继续构造——releaxed RlCS
    
    releaxed RlCS定义: $( A , B , C , m , n , l )$，其中 $m 、 n 、 l$为正整数，且 $m > l$ , $A , B , C ∈ F^{m × m}$
    且至多有n个non-zero entries，给定witness $W ∈ F^{m − l − 1}$ ,relaxed R1CS满足
    $A  (Z) \circ B  (Z) = u \cdot C ( Z) + E$
    其中 $Z=(W,x,u)$, $x \in F^l$, $E \in F^m$, $u \in F$
    
    这里当 $u=1,E=0$， relaxed R1CS就等于R1CS
    
    **继续构造，此时对于** $Z_i=(W_I,x_i,u_i)$,prover和verifier需要额外计算
    
    $u=u_1+r \cdot u_2$
    
    $E = E₁ + r • (A(Z_1) ○ B(Z_2) + A(Z_2) ○ B(Z_1) — u₁C(Z_2)— u₂C(Z_1)) + r^2 • E_2$
    
    计算新的instance-witness对： $((E,u,X),W)$
    
    对于 $Z=(W,x,u),r \in F$ 有
    
    $A(Z)○B(Z)  = A(Z_1) ○ B(Z_1) +r(A(Z_1)○B(Z_2)+A(Z_2)○B(Z_1))+r^2(A(Z_2)○B(Z_2)) \\ =（u_1C(Z_1)+E_1）+ r(A(Z_1)○B(Z_2)+A(Z_2)○B(Z_1))+r^2(u_2C(Z_2)+E_2) \\=(u_1+ru_2) \cdot C(Z_1+r Z_2)+E\\=uC(Z)+E$
    
    上面发的都是明文 $W1,W2$,后面 **`committed relaxed R1CS`** 发commitment