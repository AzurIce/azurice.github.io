---
title: "KMP 算法 —— 前缀函数（前缀数组）"
date: 2023-02-14T11:23:58+08:00
draft: false
categories:
  - 笔记
tags:
  - 算法
  - 字符串
---

## 前缀函数 / 前缀数组

对于一个给定的长度为 $n$ 的字符串 $s$ ，定义其前缀函数为一个长度为 $n$ 的数组 $\pi$ 。
其中 $\pi[i]$ 为字串 $s[0 \dots i]$ 的相等 真前缀（除了 s 本身的前缀）与 真后缀（除了 s 本身的后缀）的最长长度。

* 例，对于字符串 `aabcaabcd` ：
  $\pi[0] = 0$ ，因为 `a` 没有真前缀和真后缀，规定为 0
  $\pi[1] = 1$ ，因为 `aa` 相等的真前缀和真后缀有： `a`，其中最长长度为 1
  $\pi[2]=0$ ，因为 `aab` 无相等的真前缀和真后缀
  $\pi[3]=0$ ，因为 `aabc` 无相等的真前缀和真后缀
  $\pi[4]=1$ ，因为 `aabca` 相等的真前缀和真后缀有：`a`，其中最长长度为 1
  $\pi[5]=2$ ，因为 `aabcaa` 相等的真前缀和真后缀有： `a` `aa` ，其中最长长度为 2
  $\pi[6]=3$ ，因为 `aabcaab` 相等的真前缀和真后缀有： `aab` ，其中最长长度为 3
  $\pi[7]=4$ ，因为 `aabcaabc` 相等的真前缀和真后缀有： `aabc` ，其中最长长度为 4
  $\pi[8]=0$ ，因为 `aabcaabcd` 无相等的真前缀和真后缀

## 求前缀函数 / 前缀数组

### 朴素算法

> 时间复杂度 $O(n^3)$

```cpp
vector<int> prefix_function(string s) {
    int n = s.length();
    vector<int> pi(n);
    for (int i = 1; i < n; i++)
        for (int j = i; j >= 0; j--)
            if (s.substr(0, j) == s.substr(i - j + 1, j)) {
                pi[i] = j;
                break;
            }
    return pi;
}
```

### 优化一

> 时间复杂度 $O(n^2)$

可以注意到，每一次前缀函数至多加一（因为考察的字串长度只增长一），所以内层循环没必要从 $i$ 开始枚举：

```diff
vector<int> prefix_function(string s) {
    int n = s.length();
    vector<int> pi(n);
    for (int i = 1; i < n; i++)
-       for (int j = i; j >= 0; j--)
+       for (int j = pi[i - 1] + 1; j >= 0; j--)
            if (s.substr(0, j) == s.substr(i - j + 1, j)) {
                pi[i] = j;
                break;
            }
    return pi;
}

```

最好的情况下，每一次只需要进行一次字符串比较，一共只需比较 $n-1$ 次。
最坏的情况下，前 $n-2$ 次都一次匹配完成，最后一次匹配 $n-1$ 次，一共 $2n-3$ 次。
所以时间复杂度为 $O(n^2)$

### 优化二

> 时间复杂度 $O(n)$

$$
\overbrace{
	\underbrace{
		s_0
		s_1
		\colorbox{#aaffaa}{$\dots$}
		s_{j - 2}
	}_{j - 1}
	\colorbox{#aaaaff}{$s_{j - 1}$}
}^{j = \pi[i-1]}
\colorbox{#ffaaaa}{$s_{j}$}
\dots
\overbrace{
	s_{i - j}
	\underbrace{
		s_{i-j + 1}
		\dots
		s_{i-2}
		s_{i-1}
	}_{j - 1}
}^{j = \pi[i-1]}
\colorbox{#ffaaaa}{$s_i$}
\dots
$$

令 $j = \pi[i-1]$。

可以发现：
* 如果 $s[j] = s[i]$ 
  由于 $j = \pi[i-1]$ 即 $s[0 \dots j-1] = s[i-j \dots i-1]$ 也就是长度为 $j$ 的前后缀均相等
  也就是长度为 $j$ 的前后缀后面拼上 $s[j]$ 和 $s[i]$ 后形成的前后缀可以保证相等，此时 $\pi[i] = j + 1$ 。
* 如果 $s[j] \neq s[i]$ ，也就是长度为 $j$ 的前后缀后面拼上 $s[j]$ 和 $s[i]$ 形成的前后缀无法保证相等，那么可以退而求其次去试一试 $j-1$ 长度是否可行。
  如果 $s[j-1] = s[i]$ 也就是长度为 $j-1$ 的前后缀后面拼上 $s[j-1]$ 和 $s[i]$ 后形成的前后缀可以保证相等，此时 $\pi[i] = j - 1 + 1$  
  如果仍不相等，就再次退而求其次直到 $0$。
  要注意，此处的 $j$ “退而求其次”到何种程度才能满足条件，其实是我们可以得到的。
  $$
  \overbrace{
  	\underbrace{
  		\colorbox{#ffaaaa}{$
  			s_0
  			s_1
  		$}
  	}_{j^{(2)}}
  	\dots
  	\underbrace{
  		\colorbox{#ffaaaa}{$
  			s_{j - 2}
  			s_{j - 1}
  		$}
  	}_{j^{(2)}}
  }^{j}
  s_{j}
  \dots
  \overbrace{
  	s_{i - j}
  	s_{i-j + 1}
  	\dots
  	\underbrace{
  		\colorbox{#ffaaaa}{$
  			s_{i-2}
  			s_{i-1}
  		$}
  	}_{j^{(2)}}
  }^{j}
  s_i
  \dots
  $$

  设 $j^{(2)}$ 为“退而求其次”得到的第 $2$ 个 $j$ ， $j^{(n)}$ 为“退而求其次”得到的第 $n$ 个 $j$ 。
  我们想要找到最长的 $j^{(2)}$ 使得 $s[0\dots j^{(2)}-1] = s[i-j^{(2)} \dots i-1]$ 相等
  但是由于我们已知 $s[0\dots j-1] = s[i-j\dots i-1]$ ，那么我们其实等价于想要找到最长的 $j^{(2)}$ 使得 $s[0\dots j^{(2)}-1] = s[j-j^{(2)} \dots j-1]$ 相等
  而这，其实就是 $\pi[j - 1]$ 嘛。
  所以其实我们也就得到了一个递推公式：$j^{(n)} = \pi[j^{(n-1)} - 1], \;(j^{(n-1)} > 0)$。
最终的码：

```cpp
// C++ Version
vector<int> prefix_function(string s) {
    int n = s.length();
    vector<int> pi(n);
    for (int i = 1; i < n; i++) {
        int j = pi[i - 1];
        while (j > 0 && s[i] != s[j]) j = pi[j - 1];
        if (s[i] == s[j]) j++;
        pi[i] = j;
    }
    return pi;
}
```

那么这时我们的函数中不包含任何一次字符串比较了，时间复杂度为 $O(n)$
