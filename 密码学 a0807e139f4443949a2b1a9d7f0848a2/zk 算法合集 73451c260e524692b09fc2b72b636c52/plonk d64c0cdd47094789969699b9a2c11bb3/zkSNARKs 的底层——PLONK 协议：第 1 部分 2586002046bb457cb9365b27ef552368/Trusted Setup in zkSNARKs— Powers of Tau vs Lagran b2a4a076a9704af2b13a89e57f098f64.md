# Trusted Setup in zkSNARKs— Powers of Tau vs Lagrange basis

[https://medium.com/coinmonks/trusted-setup-in-zksnarks-powers-of-tau-vs-lagrange-basis-7f12978f1eb9](https://medium.com/coinmonks/trusted-setup-in-zksnarks-powers-of-tau-vs-lagrange-basis-7f12978f1eb9)

为了揭开 "零知识证明 "这个黑盒子的神秘面纱，本文试图讨论可信设置的某些方面，并探讨它对证明者性能的影响。当我们深入研究 zkSNARK 设置时，它分为三个部分：证明者、验证者和可信设置。从性能的角度来看，验证器必须轻便、迅速地验证证明。相反，证明者承担着沉重的负担，需要进行大量耗费精力的计算来生成证明。有时，选择适当形式的可信设置有助于减轻这种负担。

![Untitled](Trusted%20Setup%20in%20zkSNARKs%E2%80%94%20Powers%20of%20Tau%20vs%20Lagran%20b2a4a076a9704af2b13a89e57f098f64/Untitled.png)

多项式是零知识证明的基础支柱之一，由于多项式乘法( `polynomial multiplication`)、求值( `evaluations`)和多分量乘法( `multi-scaler multiplications`)等操作，消耗了大约 80% 的计算时间。这些操作显然成为性能瓶颈。值得注意的是，所处理的多项式都是高阶的，通常等于或超过 2²⁰。

让我们来探讨一个涉及密码学中多项式承诺的例子。多项式承诺是一种技术，它允许验证者以可验证的方式对特定多项式做出承诺，但不会泄露多项式本身的其他信息。

为了详细说明，请考虑两个多项式 A 和 B，每个多项式的度数都是 n=2¹⁸-1。证明者试图将这两个多项式相乘，得到一个多项式 P。两个度为n的多项式相乘得到的多项式度为2n。在这种情况下，相乘后 P 的阶数为 2×(2¹⁸-1)。随后，必须在不向验证者透露 P 的系数的情况下对 P 进行随机值（ τ (tau)）求值，即在τ 点求值。

这一过程形成了对多项式 A 和 B 的承诺，因为验证者展示了在 τ 处对 P 进行评估的能力，而无需明确透露 A、B 或 P。可验证性是通过使用加密技术来实现的，以确保验证者能正确计算 P(τ)，而无需传递有关 P、A 或 B 的任何其他信息。

![Untitled](Trusted%20Setup%20in%20zkSNARKs%E2%80%94%20Powers%20of%20Tau%20vs%20Lagran%20b2a4a076a9704af2b13a89e57f098f64/Untitled%201.png)

此外，椭圆曲线的特性掩盖了 $\tau$ 的值。当任何值（标量）乘以椭圆曲线上的一个点时，结果也是椭圆曲线上的一个点。由于离散对数问题，很难推导出原始值是多少。所有这些  $\tau$ 值都是在可信设置过程中产生的。设置代理为  $\tau$ 选择一个随机值，并计算 2n-1 以内的所有幂，然后将它们乘以椭圆曲线 G 上的一个点（通常称为生成点）。这种设置通常被称为结构化参考字符串（SRS）。

![Untitled](Trusted%20Setup%20in%20zkSNARKs%E2%80%94%20Powers%20of%20Tau%20vs%20Lagran%20b2a4a076a9704af2b13a89e57f098f64/Untitled%202.png)

从我们的学校教育中，我们熟悉了多项式的乘法：我们只需将多项式 A 中的每个项与多项式 B 中的每个项相乘。在我们的例子中，n=2¹⁸，这意味着我们需要执行 68,719,476,736 次乘法运算。如果我们假设每个乘法运算需要 1 毫秒才能完成，那么这个运算大约需要 795 天才能完成。幸运的是，还有更有效的方法。我们可以使用求值( `evaluation`)方法将多项式转换为点，并执行元素乘法，其线性时间复杂度为 O(n)。 `evaluate`多项式最便捷的方法是利用快速傅立叶变换 (FFT)。

![Untitled](Trusted%20Setup%20in%20zkSNARKs%E2%80%94%20Powers%20of%20Tau%20vs%20Lagran%20b2a4a076a9704af2b13a89e57f098f64/Untitled%203.png)

`为了使多项式 A 和 B 相乘，必须用零系数填充，以确保不会丢失结果。代码的实现将满足这一要求。`

最后一步需要将 P 的求值形式转换回多项式，这样我们就可以应用可信设置中的 $\tau$ 的幂来获得承诺。为此，我们可以利用逆快速傅立叶变换。

![Untitled](Trusted%20Setup%20in%20zkSNARKs%E2%80%94%20Powers%20of%20Tau%20vs%20Lagran%20b2a4a076a9704af2b13a89e57f098f64/Untitled%204.png)

乍看之下，以这种方式获得多项式承诺似乎开销很大，但实际上，这种方法的算法复杂度为 O(n log n)，小于 O(n²)。

![Untitled](Trusted%20Setup%20in%20zkSNARKs%E2%80%94%20Powers%20of%20Tau%20vs%20Lagran%20b2a4a076a9704af2b13a89e57f098f64/Untitled%205.png)

但是，如果我们可以使用 P 的评估形式 `evaluated form`来计算承诺，从而从例程中移除最后的 IFFT 操作呢？在这种情况下，设置代理需要做额外的工作：将生成的 SRS 转换为拉格朗日基数 **`Lagrange basis`**。这个额外的操作是完全合理的，因为设置代理只需做一次，而证明者必须在每次生成证明时都进行例程操作。

![Untitled](Trusted%20Setup%20in%20zkSNARKs%E2%80%94%20Powers%20of%20Tau%20vs%20Lagran%20b2a4a076a9704af2b13a89e57f098f64/Untitled%206.png)

在可信设置中增加一个额外步骤，就基本上消除了将 P 转换回多项式形式的必要性。因此，证明者只需计算承诺即可：

![Untitled](Trusted%20Setup%20in%20zkSNARKs%E2%80%94%20Powers%20of%20Tau%20vs%20Lagran%20b2a4a076a9704af2b13a89e57f098f64/Untitled%207.png)

`这里的拉个朗日基数就是Moonmath里讲到的 $l_j(x)$`

![Untitled](../../../Moonmath+%20oi-wiki%20org%20fde9c8d4560441338455f7e19b9a0c13/Arithmetics%E5%9F%BA%E7%A1%80%20bb2261beb08441989d315d707d984ce3/Untitled%2010.png)