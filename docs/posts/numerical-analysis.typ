#import "@preview/gentle-clues:1.2.0": *
#import "@preview/cuti:0.4.0": show-cn-fakebold

#show: show-cn-fakebold

#set document(title: "数值分析")
#set heading(numbering: "1.")

#let defination(body, ..args) = info(title: "定义", ..args, align(body))
#let theorem(body, ..args) = tip(title: "定理", ..args, align(body))
#let inference(body, ..args) = tip(title: "推论", ..args, align(body))
#let example(body, ..args) = task(title: "例题", ..args, align(body))

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


