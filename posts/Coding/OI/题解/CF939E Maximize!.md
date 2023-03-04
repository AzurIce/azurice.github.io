---
title: CF939E Maximize!
date: 2022-08-01T15:15:00+08:00
tags:
    - 二分
    - 贪心
---

Codeforces：[Problem - 939E - Codeforces](https://codeforces.com/problemset/problem/939/E)

## 题意

考虑一个由正整数组成的集合 $s$（初始为空集），有两种操作：

- 向 $S$ 中添加一个 $\geq max(S)$ 中最大值的正整数
- 找到一个 $S$ 的子集 $s$ 满足 $max(s) - mean(s)$ 最大（最大值 - 平均值）。

## 思路

可以得到两个贪心结论：

1. 要想使 $mean(s)$ 尽量小，那么一定是尽可能从最小的数开始连续选。
2. 要想使 $max(s)\;\;$ 尽量大，那么一定是要选最大的一个数。

考虑选了一个相同的最大的数，子集中最大数以外的数都 $\leq mean(s)$ 的时候，$max(s) - mean(s)$ 越大（$max(s)$ 不变，$mean(s)$ 减小）。

此处可以对 **子集中最大数以外的数个数** 进行二分，按照 **结论1** 的选法，那么 **子集中最大数以外的数个数** 满足二分要求的性质：

- 当 **子集中最大数以外的数个数** <= 某个值时，子集中第二大的数 <= $mean(s)$。
- 当 **子集中最大数以外的数个数** >= 某个值时，除最大数的最大数 >= $mean(s)$。

```
valid invalid
--[-] -------
```

## 码

```cpp
#include <iostream>

#define ll long long

using namespace std;

const int MAXN = 5E5;

int a[MAXN], sz;
ll prefixSum[MAXN];

bool check(int x) {
    double mean = (prefixSum[x] + a[sz-1]) / (x + 1);
    // cout << " #check: " << a[x-1] << " " << mean << endl;
    return a[x-1] <= mean;
}

double getAns() {
    if (sz == 1) return 0;
    int l = 1, r = sz-1;
    while (l < r) {
        int m = l + r + 1 >> 1;
        // cout << " > " << l << " " << m << " " << r << endl;
        if (check(m)) l = m;
        else          r = m-1;
    }

    // cout << "l: " << l << ", max: " << a[sz-1] << ", mean: " << (prefixSum[l] + a[sz-1]) / (l + 1.0) << endl;

    return a[sz-1] - (prefixSum[l] + a[sz-1]) / (l + 1.0);
}

int main() {
    int Q; cin >> Q;

    int f, x;
    while (Q--) {
        cin >> f;
        if (f == 1) {
            cin >> a[sz++];
            prefixSum[sz] = prefixSum[sz-1] + a[sz-1];
        } else {
            printf("%.10lf\n", getAns());
        }
    }

    return 0;
}

```