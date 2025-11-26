#set document(title: "线性代数")

#title()

= 矩阵的逆

若 $bold(A)bold(B) = bold(B)bold(A) = bold(I)$，则称 $bold(B)$ 为 $bold(A)$ 的逆矩阵，记作 $bold(B) = bold(A)^(-1)$.

= 特征值

若 $bold(A)bold(x) = lambda bold(x)$，则称 $lambda$ 为 $bold(A)$ 的特征值，$bold(x)$ 为 $bold(A)$ 的特征向量.

== 求解特征值

设 $lambda$ 是 $bold(A)$ 的特征值，则 $bold(A)bold(x) = lambda bold(x)$ 有非零解,

将 $bold(A)bold(x) = lambda bold(x)$ 改写为 $(bold(A) - lambda bold(I))bold(x) = 0$，即 $det(bold(A) - lambda bold(I)) = 0$.


