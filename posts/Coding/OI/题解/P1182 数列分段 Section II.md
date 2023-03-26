---
date: 2022-08-01T15:19:00+08:00
tags:
    - 二分
---

洛谷：[P1182 数列分段 Section II - 洛谷](https://www.luogu.com.cn/problem/P1182)

## 题意

给定一个数列，要将其分为 $M$ 段，且每段的和的最大值最小，求这个值。

## 思路

最大值最小，这种东西一般都是二分。
可以知道一个宽泛的范围：最大元素的值 <= **每段的和的最大值** <= 数列总和。
**每段的和的最大值** 小于某个值时无法在指定分段数内将数列完成分段，也就得到了我们要判断的二分区间的性质。

```
invalid     valid
------- [-]-------
```

分界点左侧都无法完成分段，右侧都可以，而我们的目标分界点在右侧区间内。

## 码

```cpp
#include <iostream>

using namespace std;

const int MAXN = 1E5 + 1;

int N, M;
int a[MAXN];

bool check(int m) {
    int cnt = 1, sum = 0;
    for (int i = 1; i <= N; i++) {
        sum += a[i];
        if (sum > m) {                  // 当前分段在限制内放不下当前这个数
            if (cnt == M) return false; // 且不能再分新的段 -> 无法完成分段
            cnt++; sum = a[i];          // 将当前元素分到新的段中
        }
    }
    return true;
}

int main () {
    cin >> N >> M;

    int maxa = 0, sum = 0;
    for (int i = 1; i <= N; i++) {
        cin >> a[i];
        maxa = max(maxa, a[i]);
        sum += a[i];
    }

    // invalid     valid
    // ------- [-]-------
    int r = sum, l = maxa;
    while (l < r) {
        int m = (l + r) >> 1;
        if (check(m)) r = m;
        else          l = m + 1;
    }
    cout << r << endl;

    return 0;
}
```