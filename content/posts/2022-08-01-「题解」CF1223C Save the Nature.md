+++
title = "「题解」CF1223C Save the Nature"
slug = "solve-cf1223c"
tags = ["算法", "题解", "二分", "贪心"]
+++

# 「题解」CF1223C Save the Nature

T15:18:00+08:00
洛谷：
Codeforces：[Problem - 1223C - Codeforces](https://codeforces.com/problemset/problem/1223/C)

## 题意

给定 $n$ 个数，可以任意调整顺序。

按照以下方式计算总和：

- 第 $a, 2a, 3a, \dots$ 个数的 $x\%$
- 第 $b, 2b,\ 3b, \dots$ 个数的 $y\%$

求能够使总和达到 $k$ 所需要用到的数的个数的最小值。

## 思路

对 **用到的数的个数** 进行二分，它满足二分所需性质：

- 当 **用到的数的个数** <= 某个值时，总和无法达到 $k$
- 当 **用到的数的个数** <= 某个值时，总和可以达到 $k$

可以得到一个宽泛的范围 $[1, n)$

```
invalid valid
------- [-]--
```

---

如何计算总和？这里是一个贪心的思路。

第 $i$ 个数有三种情况：

1. 是 $a$ 和 $b$ 的公倍数
2. 只是 $a$ 的倍数
3. 只是 $b$ 的倍数

不妨令 $x > y$，那么大的数一定优先给 $1$ 用，然后是 $2$，再然后是 $3$。

再然后，当我们确定下来 **用到的数的个数** 的时候，其实也就确定了这三种情况的数量：`n / lcm(a, b)`，`n / a`，`n / b`。

所以先排个序，然后按照顺序以此累加出总和即可。
最后判断是否 >= $k$（二分得到答案后记得再判断一次，因为有可能无解）。

## 码

```cpp
#include <iostream>
#include <algorithm>

#define ll long long

using namespace std;

const int MAXN = 2E5 + 1;

int n;
int p[MAXN];
int x, a, y, b; ll k;

ll gcd(ll x, ll y) {
    return y == 0 ? x : gcd(y, x % y);
}

bool check(ll cnt) {
    ll sum = 0; ll tot = 1;

    ll cntab = cnt / (1ll * a * b / gcd(a, b));
    ll cnta = cnt / a - cntab, cntb = cnt / b - cntab;

    for (ll i = 1; i <= cntab + cnta + cntb; i++) {
        if (i <= cntab) {
            sum += p[tot++] / 100 * (x + y);
        } else if (i <= cntab + cnta) {
            sum += p[tot++] / 100 * x;
        } else{
            sum += p[tot++] / 100 * y;
        }
        if (sum >= k) return true;
    }
    return false;
}

int main () {
    int q; cin >> q;

    while (q--) {
        cin >> n;
        for (int i = 1; i <= n; i++) {
            cin >> p[i];
        }
        sort(p + 1, p + 1 + n, greater<int>());
        cin >> x >> a >> y >> b >> k;
        if (y > x) { swap(x, y); swap(a, b); }

        // invalid valid(probably)
        // ------- [-]--
        int l = 1, r = n;
        while (l < r) {
            int m = l + r >> 1;
            if (check(m)) r = m;
            else          l = m + 1;
        }
        if (!check(l)) cout << -1 << endl;
        else           cout << l << endl;
    }

    return 0;
}

```