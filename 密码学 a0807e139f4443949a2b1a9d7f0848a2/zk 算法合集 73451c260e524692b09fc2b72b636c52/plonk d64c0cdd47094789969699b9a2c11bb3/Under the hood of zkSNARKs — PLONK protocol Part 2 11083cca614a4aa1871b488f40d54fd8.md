# Under the hood of zkSNARKs — PLONK protocol: Part 2

在 PLONK 系列的上一篇文章中，我们介绍了协议的核心--KZG 承诺方案，以及多项式线性独立性如何帮助 `equation`单个方程中的多个多项式。

在此，我想讨论 PLONK 中使用的另一种重要优化方法，它与 `vanishing polynomial`有关

## **Vanishing Polynomial**

在上一篇文章中，我提到多项式是将特定进程的状态编码成一个函数的有效方法。请看下面这个简单的例子：在状态 #1 时，进程值为 -6；在状态 #2 时，进程值变为 2；在状态 #3 时，进程值变为 162，以此类推。

![Untitled](Under%20the%20hood%20of%20zkSNARKs%20%E2%80%94%20PLONK%20protocol%20Part%202%2011083cca614a4aa1871b488f40d54fd8/Untitled.png)