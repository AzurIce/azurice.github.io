+++
title = "「题解」CF19B Checkout Assistant"
slug = "solve-cf19b"
tags = ["算法", "题解", "dp", "背包"]
+++

# 「题解」CF19B Checkout Assistant

T16:59:00+08:00
CodeForces：[Problem - 19B - Codeforces](https://codeforces.com/problemset/problem/19/B)

洛谷：[CF19B Checkout Assistant - 洛谷 | 计算机科学教育新生态 (luogu.com.cn)](https://www.luogu.com.cn/problem/CF19B)

## 「题目」

购物车中由 $n$ 件物品，第 $i$ 件物品价格为 $c_i$ 且需要花费收银员 $t_i$ 秒来结算。在收银员正在结算某物品时，可以花费 $1$ 秒偷取其他的物品（不用结算）。求将所有物品带走所需的最少花费。

## 「思路」

结算第 $i$ 件物品 $\Leftrightarrow$ 拿走 $t_i+1$ 件物品。

不必考虑拿走哪些物品，只需要总的能拿走的物品数 $\geq n$ ，所有物品都能带走即可。

即体积为 $t_i+1$，价值为 $c_i$，总体积 $\geq n$，价值最小的01背包。

## 码

```c++
#include <iostream>
#include <cstring>

#define ll long long

using namespace std;

const int MAXN = 2000 + 7;
const int MAXC = 1E9 + 7;
const int MAXT = MAXN << 1;

ll f[MAXT];
int t[MAXN], v[MAXN];

int main () {
    int n; cin >> n;

    int maxt = 0;
    for (int i = 1; i <= n; i++) {
        cin >> t[i] >> v[i];
        t[i] ++;
        maxt = max(maxt, t[i]);
    }
    maxt += n;

    memset(f, 0x7f, sizeof(f));
    f[0] = 0;
    for (int i = 1; i <= n; i++) {
        for (int j = maxt; j >= t[i]; j--) {
            f[j] = min(f[j], f[j - t[i]] + v[i]);
        }
    }

    ll ans = 2e12 + 7;
    for (int i = n; i <= maxt; i++) {
        if (f[i] < ans) ans = f[i];
    }

    cout << ans << endl;

    return 0;
}

```