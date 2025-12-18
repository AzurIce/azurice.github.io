#import "@preview/gentle-clues:1.2.0": *
#import "@preview/cuti:0.4.0": show-cn-fakebold

#show: show-cn-fakebold

#set document(title: "数值分析")
#set heading(numbering: "1.")

#let defination(body, ..args) = info(title: "定义", ..args, align(body))
#let theorem(body, ..args) = tip(title: "定理", ..args, align(body))
#let inference(body, ..args) = tip(title: "推论", ..args, align(body))
#let example(body, ..args) = task(title: "例题", ..args, align(body))

#counter(heading).update(2)

= 线性方程组的解法

== 三种迭代矩阵

三种迭代方式都可以表示为下式：

$ bold(x)^((k+1)) = bold(B)bold(x)^((k)) + bold(g) $

三种迭代方法中都会将原稀疏矩阵 $bold(A)$ 分解为 $bold(A) = bold(D) - bold(L) - bold(U)$，其中 $bold(D)$ 为对角矩阵，$bold(L)$ 为严格下三角矩阵，$bold(U)$ 为严格上三角矩阵.

=== Jacobi 迭代

$ bold(B)_J = bold(D)^(-1)(bold(L) + bold(U)) = bold(I) - bold(D)^(-1)bold(A) $
$ bold(g)_J = bold(D)^(-1)bold(b) $

=== Seidel 迭代

$ bold(B)_S = (bold(D) - bold(L))^(-1)bold(U) $
$ bold(g)_S = (bold(D) - bold(L))^(-1)bold(b) $

=== Sor 迭代

$ bold(B)_omega = (bold(D) - omega bold(L))^(-1)[(1 - omega)bold(D) + omega bold(U)] $
$ bold(g)_omega = omega (bold(D) - omega bold(L))^(-1)bold(b) $

== 谱半径

#defination[
  设 $bold(A) in bold(R)^(n times n)$，$lambda_k (k = 1, 2, dots, n)$ 是 $bold(A)$ 的 $n$ 个特征值，则称实数

  $ rho(bold(A)) = max_(1 <= k <= n) |lambda_k| $

  为矩阵的谱半径. 注意如果 $lambda_k$ 为复数，则 $|lambda_k|$ 表示复数模.
]

== 收敛条件

#theorem[
  单步定长线性迭代公式 $bold(x)^((k+1)) = bold(B)bold(x)^((k)) + bold(g)$ 收敛的充要条件是迭代矩阵谱半径 $rho(bold(B)) < 1$.
]

#inference[
  判定收敛的三个充分条件：
  - $||bold(B)|| < 1 =>$ Jacobi、Seidel、Sor 迭代均收敛
  - $bold(A)$ 为严格对角占优阵 $=>$ Jacobi、Seidel 迭代均收敛
  - $bold(A)$ 为对称正定矩阵 $=>$ Seidel 迭代收敛
]

#example[
  给定方程组

  #set math.mat(align: right, delim: "[")
  $ mat(2, -1, 1; 1, 1, 1; 1, 1, -2) mat(x_1; x_2; x_3) = mat(1; 1; 0) $

  试考察用 Jacobi 迭代法和 Seidel 迭代法求解的收敛性.

  ---

  *写出迭代矩阵 $->$ 求特征值 $->$ 谱半径 $->$ 判断收敛性*

  ---

  对 Jacobi 迭代，迭代矩阵为 

  $ bold(B)_J = bold(D)^(-1)(bold(L) + bold(U)) = mat(2, 0, 0; 0, 1, 0; 0, 0, -2)^(-1)mat(0, 1, -1; -1, 0, -1; -1, -1, 0) = mat(0, 1/2, -1/2; -1/2, 0, -1/2; -1/2, -1/2, 0) $

  由 $|lambda bold(I) - bold(B)_J| = lambda^3 + 5/4 lambda = 0$，有特征值 $lambda_1 = 0, lambda_2 = sqrt(5)/2 i, lambda_3 = -sqrt(5)/2 i$，
  
  谱半径 $rho(bold(B)_J) = sqrt(5)/2 > 1$，故 Jacobi 迭代法不收敛.

  对 Seidel 迭代，迭代矩阵为

  $ bold(B)_S = (bold(D) - bold(L))^(-1)bold(U) = mat(2, 0, 0; 1, 1, 0; 1, 1, -2)^(-1)mat(0, 1, -1; 0, 0, -1; 0, 0, 0) = mat(0, 1/2, -1/2; 0, -1/2, -1/2; 0, 0, -1/2) $

  由 $|lambda bold(I) - bold(B)_S| = lambda (lambda + 1/2)^2 = 0$，有特征值 $lambda_1 = 0, lambda_2 = lambda_3 = -1/2$，

  谱半径 $rho(bold(B)_S) = 1/2 < 1$，故 Seidel 迭代法收敛.

]

#counter(heading).update(4)

= 插值与拟合方法

有一类问题：已知函数上的若干点 $(x_i, f(x_i))$ 求未知函数 $f(x)$。

定义 $P(x)$ 为所求的近似函数，$delta(x) = f(x) - P(x)$ 为偏差。

求解这类问题的方法主要有两种：*插值* 和 *拟合*，其区别在于：

- *插值*：要求 $delta(x_i) = 0, i = 0, 1, dots, n$

  已知函数 $y = f(x)$ 在 $[a, b]$ 上的 $n + 1$ 个互异点 $x_0, x_1, dots, x_n$ 出的函数值 $f(x_i), i = 0, 1, dots, n$，求 $f(x)$ 的一个近似函数 $P(x)$ 满足插值条件：

  $ P(x_i) = f(x_i), i = 0, 1, dots, n $

  此时，称 $P(x)$ 为 $f(x)$ 的一个插值函数，$f(x)$ 为被插函数，$x_i$ 为插值节点，误差函数 $R(x) = f(x) - P(x)$ 为插值余项。

  假设 $[a, b]$ 是包含所有插值节点的最小闭区间，那么利用插值函数计算其内部的函数值时被称为 *内插计算*，反之则被称为 *外插计算*，前者一般比后者精确.

- *拟合*：要求 $delta$ 的某种范数最小，且同一个 $x_i$ 可能不互异（即可能有不同观测值）

== 插值

=== 代数插值

插值函数是多项式时称为代数插值（或多项式插值）：

$ P(x) = P_m(x) = sum_(k=0)^m a_k x^k, a_k in RR $

若它满足插值条件

$ P(x_i) = f(x_i), i = 0, 1, dots, n $

则有关于 $a_0, a_1, dots, a_m$ 的线性方程组

$
cases(
  a_0 + a_1 x_0 + a_2 x_0^2 + dots + a_m x_0^m &= y_0,
  a_0 + a_1 x_1 + a_2 x_1^2 + dots + a_m x_1^m &= y_1,
  dots,
  a_0 + a_1 x_n + a_2 x_n^2 + dots + a_m x_n^m &= y_n
)
$

当 $m = n$ 时，方程组才可能有唯一解，故对 $n + 1$ 个插值节点选取 $n$ 次多项式作为插值函数，此时其系数矩阵为范德蒙行列式：

#set math.mat(delim: "|")
$
D = mat(
  1, x_0, x_0^2, dots, x_0^n;
  1, x_1, x_1^2, dots, x_1^n;
  dots;
  1, x_n, x_n^2, dots, x_n^n
) = product_(n >= i >= j >= 0) (x_i - x_j)
$

由插值点互异有 $D != 0$，故方程组有唯一解。

#theorem[
  对于 $n + 1$ 个互异查直接点，存在唯一的 $n$ 次插值多项式.
]

==== Lagrange 插值

==== Newton 插值

==== Hermite 插值

==== 分段多项式插值

==== 三次样条插值




#example[
  已知 $sin x$ 在 $30 degree$，$45 degree$，$60 degree$ 处的值分别为 $1/2$，$sqrt(2)/2$，$sqrt(3)/2$，分别用一次插值和二次插值求 $sin 50 degree$ 的近似值，并估计截断误差.

  ---


]
