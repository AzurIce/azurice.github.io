---
date: 2022-03-20T16:53:00+08:00
tags:
    - dp
    - 背包
---

CodeForces：[Problem - F - Codeforces](https://codeforces.com/contest/1650/problem/F)

洛谷：[CF1650F Vitaly and Advanced Useless Algorithms - 洛谷 | 计算机科学教育新生态 (luogu.com.cn)](https://www.luogu.com.cn/problem/CF1650F)

## 「题意」

有 $n$ 个任务，每个任务要在 $a_i$ 时刻前做完。（$a$ 升序）

有 $m$ 个计划，每个计划最多只能执行一次，每个计划由 $e_i$, $t_i$, $p_i$ 描述：如果执行了第 $i$ 个计划，那么 $t_i$ 时间后，任务 $e_i$ 会被完成 $p_i$ 的百分比。

输入 $T$ 组数据。

输出每组执行的计划个数 $k$，并按顺序输出执行的计划（若不行则输出 `-1`，若有多种方案输出其中一种即可）。

## 「思路」

首先显然有一个贪心的结论：在当前任务未完成时，优先选择能对当前任务有进展的计划（优先完成离ddl近的任务）。

那么对于每一个任务的目标就是 **能使进度达到100%的最少时间**。

很容易借助 **差分** 得到 **每一个任务的限制时间** 。

对于每一个任务，就是个 **01背包**，对于每一个计划选择 **执行** 或 **不执行**。

在当前任务省下来的时间，就可以添加到下一个任务的时间中去。

## 「码」

```c++
#include <iostream>
#include <cstring>
#include <tuple> // 元组 c++11
#include <vector>

#define INF 0x7f
#define ll long long

using namespace std;

const int MAXN = 1E5 + 7;

int a[MAXN];
ll dp[200 + 7]; // 最少时间
bool f[MAXN][200 + 7];

int solveTask(int a, vector<tuple<int, int, int>> &plans, vector<int> &ans) {
    // memset(f, 0, sizeof(f));
    memset(dp, INF, sizeof(dp));
    dp[0] = 0;

    int n = plans.size();
    int endk = 0;
    for (int j = 0; j < n; j++) {
        auto [e, t, p] = plans[j]; // 结构化绑定 c++17, auto 自动类型推断 c++11
        for (int k = 200; k >= 0; k--) {
            if (k >= p && dp[k - p] + t < dp[k]) {
                dp[k] = dp[k - p] + t;
                f[j][k] = 1;
                if (k >= 100 && dp[k] <= a && (dp[k] < dp[endk] || endk == 0)) {
                    endk = k;
                }
            } else {
                f[j][k] = 0;
            }
        }
    }

    if (endk == 0) return -1;
    int k = endk, j = n-1;
    for (int j = n-1; j >= 0 && k; j--) {
        if (f[j][k]) {
            auto [e, _, p] = plans[j]; // 结构化绑定 c++17, auto 自动类型推断 c++11
            ans.emplace_back(e); // 在vector末尾原位构造 c++11
            k -= p;
        }
    }
    return dp[endk];
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    cout.tie(0);

    int T; cin >> T;

    int n, m;
    while (T--) {
        cin >> n >> m;

        a[0] = 0;
        for (int i = 1; i <= n; i++) {
            cin >> a[i];
            a[i-1] = a[i] - a[i-1];
        }

        vector<vector<tuple<int, int, int>>> plans(n); // 元组 c++11
        for (int i = 1, _e, _t, _p; i <= m; i++) {
            cin >> _e >> _t >> _p;
            plans[_e - 1].emplace_back(i, _t, _p); // 在vector末尾原位构造 c++11
        }

        bool f = 0;
        vector<int> ans;
        int saved_time = 0;
        for (int i = 0; i < n; i++) {
            a[i] += saved_time;
            int mint = solveTask(a[i], plans[i], ans);
            if (mint < 0) {
                f = 1;
                break;
            }
            saved_time = a[i] - mint;
        }

        if (f) {
            cout << -1 << endl;
        } else {
            cout << ans.size() << endl;
            // auto 自动类型推断 c++11
            for (auto iter = ans.begin(); iter != ans.end(); iter++) {
                cout << *iter << ' ';
            }
            cout << endl;
        }
    }

    return 0;
}

```

## 「几个坑点」

看到那个注释掉的 `memset()` 没！

大坏蛋！

调了一个小时，问了别人才知道这个 `memset()` 可能会导致超时！

原因是 `f` 数组太大了，且这个 `memset` 在每个 `case` 中都被调用了 `n` 次！

这回我可记住了！