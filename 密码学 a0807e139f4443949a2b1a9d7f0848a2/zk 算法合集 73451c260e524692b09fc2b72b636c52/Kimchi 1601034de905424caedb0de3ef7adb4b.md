# Kimchi

[https://minaprotocol.com/blog/kimchi-the-latest-update-to-minas-proof-system](https://minaprotocol.com/blog/kimchi-the-latest-update-to-minas-proof-system)

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled.png)

Kimchi 是Mina用来生成递归证明的主要工具，它使 Mina 区块链的大小保持在 22KB 左右。

pickles是递归层，用来创建证明的协议，并将区块链的大小减少到 22KB 以下。

用了[pasta curves](https://electriccoin.co/blog/the-pasta-curves-for-halo-2-and-beyond/).

# **Arithmetic circuits**

算术电路由算术门构成：

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%201.png)

使用加法和乘法就可以重写大部分程序

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%202.png)

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%203.png)

下面是一个约束单比特的例子：

1. `x(x-1) = 0` 确保 x 确实是一个位（0 或 1）。

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%204.png)

电路目前只有两个门电路。在 PLONK 中，可以将其写成一个作用于寄存器（两个输入 L 和 R 以及输出 O）的门电路列表：

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%205.png)

但是这里不是L和R的加法，而是1-R，所以要调整

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%206.png)

现在，让我们把乘法门添加到列表中。但请记住，输出寄存器是不能使用的，因为它必须为 0：

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%207.png)

最后一步，连接两个寄存器，第一步的输出是第二步的输入：

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%208.png)

现在，当证明者想要进行证明时，他们会运行程序，并在执行跟踪中记录每个寄存器的值。例如，这里有一个 x = 0 的程序。把 `x=0` 代入电路执行过程：

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%209.png)

Kimchi中则使用了通用门：

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%2010.png)

通过改变参数，就可以变成不同的门：

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%2011.png)

# **From PLONK to Kimchi**

Kimchi 是在 PLONK 基础上进行的一系列改进、优化和变更。例如，它通过在协议中使用bulletproof-style polynomial commitment ，克服了 PLONK 受信任设置的限制。这样一来，就无需相信可信设置的参与者是诚实的（如果他们不诚实，就会破坏协议）。说到电路，由于我们在这里讨论的是电路，Kimchi 在 PLONK 已有的 3 个寄存器的基础上又增加了 12 个寄存器：

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%2012.png)

这些寄存器分为两类：IO 寄存器和临时寄存器（有时称为advice wires），前者可以相互连线，后者只能由相关门使用。

有了更多的寄存器，就意味着我们现在可以让门接受多个输入，而不仅仅是一个输入：

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%2013.png)

这就带来了新的可能性。例如，一个标量乘法门至少需要三个输入（一个标量和两个曲线点坐标）。由于某些运算比其他运算发生得更频繁，因此可以更高效地改写成新的门。目前，Kimchi 提供了 9 个新门：

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%2014.png)

Kimchi 的另一个概念是，一个门可以直接将其输出写入下一个门使用的寄存器中。这对于像 "poseidon "这样需要连续使用多次（具体来说是 11 次）来表示 poseidon 哈希函数的门非常有用

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%2015.png)

Kimchi 实现的另一项性能改进是查找。有时，一些操作可以写成lookups。例如，XOR 表：

![Untitled](Kimchi%201601034de905424caedb0de3ef7adb4b/Untitled%2016.png)

一个 4 位值的 XOR 表大小为 $2^8$。因此，Kimchi 构建了这个表，并允许门（目前只有 Chacha 使用）简单地对表进行查找，以获取操作结果。