---
title: CF1201C Maximum Median
date: 2022-08-01T15:16:00+08:00
tags:
    - 贪心
    - 模拟
    - 差分
---

Codeforces：[Problem - 1201C - Codeforces](https://codeforces.com/problemset/problem/1201/C)

## 题意

给定一个有 $n$ 个整数的数组，可以做 $k$ 次如下操作：

- 任选一个元素，使其增大1

求最终得到数组的中位数的最大值。

## 思路

![[../../__ Obsidian __/Excalidraw/CF1201C Maximum Median 2022-07-28 20.43.27.excalidraw]]

一个贪心结论：不需要管小的那一半数，只需向中间以及大的那一半数增加即可。

结论：要想使中位数增大一，则需要“填平”一层。

照这样模拟就好啦~

## 码

```cpp
#include <iostream>
#include <algorithm>

#define ll long long

using namespace std;

const int MAXN = 2E5 + 1;

int a[MAXN];
int differ[(MAXN >> 1) + 1];

int n, k;
int main() {
    cin >> n >> k;

    for (int i = 0; i < n; i++)
        scanf("%d", a + i);

    sort(a, a + n);

	int mid = n >> 1;

	// 差分
    a[n] = a[n-1];
    for (int i = mid; i < n; i++)
        differ[i - mid + 1] = a[i+1] - a[i];

    for (int i = 1; i <= mid + 1; i++) {
	     // 还没填到最后一个数，且还够填平当前这一层
        if (k >= (ll) i * differ[i] && i != mid + 1) {
            k -= (ll) i * differ[i];
            a[mid] += differ[i];
        } else {
            a[mid] += k / i;
            break;
        }
    }

    cout << a[mid] << endl;

    return 0;
}

```