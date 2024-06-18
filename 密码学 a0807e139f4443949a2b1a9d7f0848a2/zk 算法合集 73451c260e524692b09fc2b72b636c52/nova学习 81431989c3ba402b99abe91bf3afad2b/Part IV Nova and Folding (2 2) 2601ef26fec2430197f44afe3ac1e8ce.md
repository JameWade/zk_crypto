# Part IV: Nova and Folding (2/2)

在上一篇文章中，我们看到了折叠是如何将给定的 R1CS 结构的两个实例折叠成一个新结构的。

其原理是随机线性组合。不过，这其中也有一定的魔法成分：由于 R1CS 对线性组合并不十分满意，因此需要一些高超的魔法才能将它们转变为更适合线性组合的宽松 R1CS。在本文中，我们将尝试用二次方程的理论来说明这一过程实际上是非常自然的：标量 u 是在二次方程同质化时自然产生的，误差项e（宽松项）与所得二次方程的值相对应，而交叉项则可以用相关的二次线性方程来自然解释。术语太复杂了，让我们一步一步来…

# 回顾

上次我们看到计算可以转化为算术可满足性问题，并介绍了一种特殊的中间表示，即等级 1 约束系统（R1CS）。它们是特定形式的二次方程组。更确切地说，我们有一个输入向量 X 和一个输出向量 Y（我们称它们为实例 I），以及一个见证向量 W。利用 1、I 和 W，我们组成向量 Z，并考虑形式如下的一元二次方程：

$A(Z) ○ B(Z) = C(Z)$

其中 $A(\cdot)$、 $B(\cdot)$ 和 $C(\cdot)$ 对应于取固定的线性组合（称为结构）。

一般来说，我们有多个这样的方程（或将 A、B 和 C 视为矩阵），不过为了便于记述，我们通常只写其中一个。如果所有二次方程都满足 Z = (1, I, W)，那么我们就说实例 I 满足见证 W。

折叠的目的如下：固定一个方程组（A、B 和 C，我们称之为结构），假设有两个实例 I₁ 和 I₂。证明者希望让验证者相信，它知道两个 W 实例的证人 W₁ 和 W₂。为此，证明者想提出一个新的实例（对于相同的 R1CS 结构，即相同的方程组），如果证明者能让验证者相信它知道实例 I 的证人 W，那么验证者就会相信证明者知道实例 I₁ 和 I₂ 的证人 W₁ 和 W₂。

因此，从某种意义上说，证明者为同一个计算（实例 I）提供了假想的输入和输出，如果证明者能让验证者相信它有一些证人，并以连接人工输入和输出的方式正确执行了计算步骤，验证者就会相信两个初始的真实输入输出对也是如此。 `通过一个新的证明W，通过验证，从而让验证者相信 $W_1$和 $W_2$`

Kothapalli、Setty 和 Tzialla 提出的折叠方案优雅地实现了这一点。

# **The power of random linear combinations**

该方案的合理性基于以下经常使用的随机线性组合原理：

*If a prover can do something (solve a problem) for arbitrary linear combinations (linear combinations determined by challenges of the verifier, or a version rendered non-interactive using Fiat-Shamir and in practice a random oracle instantiation by a hash function) of two objects, then they can do it for the objects themselves.*

通常是这样论证的：给定许多对象的随机线性组合的问题答案，就有办法提取对象本身的答案。验证者如果能回答一个随机挑战，就能回答许多这样的挑战，从而提取出初始问题的解决方案（因此，假设验证者已经知道这些问题是合理的）。

现在，R1CS 是二次方程，与同质线性方程不同，它不善于使用线性组合。二次表达式在两点线性组合处的值不能轻易地与它在这两点的值（或这些值的相应线性组合）联系起来。关于二次型的基本结果可以提供一些补救办法。

`后面没看懂，操`

# **Making things “linear-like”**

让我们先把方程改写为：

$P(Z) = A(Z) ○ B(Z) — C(Z) = 0$

回顾一下， $Z = (1，I，W)$。这里，P(Z) 是一个多变量多项式，满足 R1CS 即可将多项式求零。显然，多项式 P(Z) 的两个零点的线性组合一般不会是零，也就是说，P(Z₁) = 0 和 P(Z₂) = 0 并不意味着 $P(r₁ \cdot Z₁+ r₂ \cdot Z₂) = 0$。

如果 P(Z) 是线性的，情况就会是这样：

$P(r₁• Z₁+ r₂ • Z₂) = r₁ • P(Z₁) + r₂ • P(Z₂) = r₁ • 0 + r₂ • 0 = 0$

因此会有一定的误差。我们不需要处理 P(Z) 的零点，而是专注于 P(Z) 在不同点上的值。同样，由于 P(Z) 不是线性的，它在 Z₁ 和 Z₂ 处的值不容易与它在比如 $r₁\cdot  Z₁+ r₂ \cdot Z₂$ 处的值联系起来。

第一步是将多项式同质化。多项式有 2 次方项（来自 A(Z)○B(Z)）、线性项（不仅来自 C(Z)，还来自 A(Z)○B(Z)，因为 Z 的第一个项是 z1 = 1）和常数项。

只处理二次项（更同质）会更容易。实现这一点的方法是引入一个新变量 u，并将 P(Z) 的每项乘以 u 的适当幂次，使所有项的阶数都为 2。

因此，对于来自 C(Z)的项，我们需要一个 u 因子；对于来自 A(Z) ○ B(Z)的项，我们需要一个或两个 u 因子，其中分别包含一个或两个 z1 = 1 的实例。因此，这一过程相当于用 u 替换 z1 = 1，并用 u 乘以所有 C(Z)。结果是：

$Q(Z) = A(Z) ○ B(Z) — u • C(Z) = 0$

$Z = (u, I, W)$

Q(Z) 是 u、I 和 W 变量组成的的 2 次同质多项式。

由于所有项的阶数都是 2，我们可以看到：

$Q(r • Z) = r² • Q(Z)$

对于任意常数 r，因此它在标量乘法下正常。然而，加法不行。我们没有

$Q(Z₁ + Z₂) = Q(Z₁) + Q(Z₂)$

对于 $Z_1$和 $Z_2$,很难联系 $Q(Z₁ + Z₂), Q(Z₁), Q(Z₂)$之间的关系

# **A short tangent on Quadratic and Bilinear Forms**

同质化后的 Q(Z) 是一个多项式，其所有项的阶数都是 2。我们称之为二次型。它不是线性的，但我们可以将它与一个新函数联系起来，这个新函数就是 "双线性 "函数。更准确地说，定义：
$B(X,Y)={1 \over 2}(Q(X+Y)-Q(X)-Q(Y))$

$B ( \cdot, \cdot )$取两个向量 X 和 Y，可以证明它分别与这两个向量线性相关，因此被称为双线性方程。因此我们有

$B(R_1 \cdot X_1 + r_2 \cdot X_2   ,Y) = r_1 \cdot B(X_1,Y)+ r_2 \cdot B(X_2,Y)$

和

$B(X_1    ,r_1 \cdot Y_1 + r_2 \cdot Y_2) = r_1 \cdot B(X,Y_1)+ r_2 \cdot B(X,Y_2)$

知道了 B(X,Y)，我们就能预测二次型 Q(X) 在线性组合下的表现。

我们可以从 Q(X) 和 Q(Y) 推断出 Q(r₁X + r₂Y)，几乎就像线性一样。根据 B（X，Y）的定义方程，我们可以得出

$Q(X+Y) = Q(X)+Q(Y) -2 \cdot B(X,Y)$

具体而言，我们有

$Q(r_1 X + r_2Y)=Q(r_1X) + Q(r_2Y)-2 \cdot (B(r_1X,r_2Y)) = r_1^2Q(X)+ r_2^22Q(Y) - 2 r_1 r_2 \cdot B(X,Y)$

请注意，Q(r₁X + r₂Y)可以表示为 B(X，Y)、Q(X) 和 Q(Y) 的线性组合。这一点稍后会变得很重要。

# **Long story short…**

为了折叠，我们希望 Q(Z) 是线性的。但是，如果我们知道特定 X 和 Y 的 B(X,Y)，那么至少对于这些 X 和 Y，我们可以假装它是这样的，即我们可以把 Q(r₁X + r₂Y) 写成 B(X,Y)、Q(X) 和 Q(Y) 的线性组合。知道了 B(X,Y)，双方就可以从 Q(X) 和 Q(Y) 计算出 Q(r₁X + r₂Y)。反过来，他们可以从要折叠的 R1CS 实例的松弛向量 Eₓ 和 Eᵧ 计算出与随机线性组合相对应的松弛向量。随机性保证了健全性，就像在线性情况下一样：知道折叠实例的证明者事实上至少以极高的概率知道两个被折叠的实例。

# **The solution is simple…**

对于 X 和 Y 的折叠，证明者和验证者使用 B(X,Y)，其余的几乎与线性情况相同。但有一个问题：要计算 B(X,Y)，验证者需要 X 和 Y 本身，其中包含相关的见证。因此，这既不简洁，也不具备所需的零知识属性。事实上，这就像发送整个证人，并在此基础上做一些额外的工作。

# **Commitments save the day**

我们不使用松弛向量（即 Q(X) 和 B(X,Y)，因为我们无法将它们传递给验证者），而是使用对它们的承诺。还记得我们在上面提到过，随机线性组合的松弛向量可以表示为两个实例的松弛向量和 B(X, Y) 的线性组合吗？现在这将非常方便。如果我们使用加法同态承诺，就可以用松弛向量的承诺和 B(X, Y) 的承诺来表达对折叠松弛向量的承诺。换句话说，在上面的表达式中取承诺（用条形表示），我们可以得到

$COM(Q(r_1 X + r_2Y))=r_1^2COM(Q(X)) + r_2^2COM(Q(Y))-2 \cdot r_1r_2COM(B(X,Y))$

因此，证明者发送的不是松弛向量，而是对松弛向量的承诺（以及对 B(X, Y) 的承诺，以便验证者计算折叠实例的承诺）。这就是所谓的承诺松弛 R1CS **`Committed Relaxed R1CS`**。

# 续上篇最后面的构造

[https://blog.csdn.net/qq_34793644/article/details/130867097?ops_request_misc=%257B%2522request%255Fid%2522%253A%2522171396992116800184195648%2522%252C%2522scm%2522%253A%252220140713.130102334.pc%255Fall.%2522%257D&request_id=171396992116800184195648&biz_id=0&utm_medium=distribute.pc_search_result.none-task-blog-2~all~first_rank_ecpm_v1~rank_v31_ecpm-1-130867097-null-null.142^v100^pc_search_result_base6&utm_term=relaxed%2BR1CS&spm=1018.2226.3001.4449](https://blog.csdn.net/qq_34793644/article/details/130867097?ops_request_misc=%257B%2522request%255Fid%2522%253A%2522171396992116800184195648%2522%252C%2522scm%2522%253A%252220140713.130102334.pc%255Fall.%2522%257D&request_id=171396992116800184195648&biz_id=0&utm_medium=distribute.pc_search_result.none-task-blog-2~all~first_rank_ecpm_v1~rank_v31_ecpm-1-130867097-null-null.142%5Ev100%5Epc_search_result_base6&utm_term=relaxed%2BR1CS&spm=1018.2226.3001.4449)

1. **committed relaxed R1CS**
    
    定义： $(A,B,C,m,n,l)$,其中 $m、n、l$为正整数，且 $m>l$, $A,B,C \in F^{m \times m}$，且至多有n个 non-zero entries, $(\overline {E},u, \overline {w},x)$是commitment relaxed R1CS 的instance，其中 $(\overline {E}, \overline {w})$是 $(E,w)$的承诺， $x \in F^l$是公共输入和输出， $u \in F$,如果 $(E,r_E,W,r_W) \in (F^m,F,F^{m-l-1},F)$满足：
    
    - $\overline {E}=Comm(E,r_E)$
    - $\overline {W}=Comm(W,r_W)$
    - $(A \cdot Z) ○(B \cdot Z) = u \cdot (C \cdot Z)+E$
    
    $(E,r_E,W,r_W)$ 是instance$(\overline {E},u, \overline {w},x)$对应的witness，其中 $Z=(W,x,u)$
    
2. **committed relaxed R1CS的Folding Scheme**
    
    此时prover和Verifier都计算instance $(\overline {E_1},u_1, \overline {w_1},x_1)$和$(\overline {E_2},u_2, \overline {w_2},x_2)$
    
    prover单独计算witness $(E_1,r_{E1},W_1,r_{W1})$ 和$(E_2,r_{E2},W_2,r_{W2})$ 
    
    令 $Z_1 = (W_1,x_1,u_1)$、 $Z_2 = (W_2,x_2,u_2)$，交互如下：
    
    1. prover:发送 $\overline {T}=Comm(T,r_T)$，其中 $r_T \in F$,T为交叉项
        
        $T=AZ_1 ○ BZ_2+AZ_2○BZ_1 -  u_1CZ_2 - u_2CZ_1$
        
    2. Verifier：选择随机数 $r \in F$并发送
    3. prover和Verifier计算folded instance $(\overline {E},u, \overline {w},x)$
        
         $\overline {E} = (\overline {E_1} + r \cdot \overline {T}+r^2 \cdot \overline {E_2})$
        
        $u=u_1 + r \cdot u_2$
        
        $\overline {W} = \overline {W_1} + r \cdot \overline {W_2}$
        
        $x=x_1+r \cdot x_2$
        
    4. prover单独计算folded witness$(E,r_E,W,r_W)$ 
    
     ${E} = ( {E_1} + r \cdot  {T}+r^2 \cdot  {E_2})$
    
    $r_E = r_{E1} + r \cdot r_T + r^2 \cdot r_{E2}$
    
     ${W} =  {W_1} + r \cdot  {W_2}$
    
    $r_W = r_{W_1}+r \cdot r_{W_2}$