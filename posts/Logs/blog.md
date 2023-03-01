---
title: "博客迁移"
date: 2023-02-08T23:17:14+08:00
draft: false
categories:
    - Logs
tags:
    - testTag1
    - testTag2
---

目前正在折腾博客，迁移了部分文章，有一些图片链接还未修复。
接下来要做的：

- 搞定CDN 加速
- 把博客迁移完成
  - 文章
  - 图片资源
- 折腾折腾博客主题以及组织结构和设置

---
下面时测试
---

# First level heading

## Second level heading

### Third level heading

## Hello Hugo!

| test | table |
|------|-------|
|waibiwaibi|waibibabo|

$$
1+1=?
$$

**Bold**, *Italic*, <s>Delete</s>, <u>Underline</u>

---

------

测试下评论
抄自 [Welcome to giscus! · Discussion #62 · giscus/giscus](https://github.com/giscus/giscus/discussions/62#discussioncomment-740574)
以及 [github-flavored-markdown-test/README.md at master · suan/github-flavored-markdown-test](https://github.com/suan/github-flavored-markdown-test/blob/master/README.md)

Hello! 🎉

**Test markdown features...**

# H1
followed by some text

## H2
followed by some text

### H3
followed by some text

#### H4
followed by some text

##### H5
followed by some text

###### H6
followed by some text

Auto-detected link: https://giscus.vercel.app

Can render `inline code`, *italics*, **strong**, 😄

> here is blockquote
> test `inline code`, *italics*, **strong**, 😄

When $a \neq 0$, there are two solutions to $(ax^2 + bx + c = 0)$ and they are

$$
x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
$$


**The Cauchy-Schwarz Inequality**

$$
\left(\sum_{k=1}^n a_k b_k\right)^2 \leq \left(\sum_{k=1}^n a_k^2\right)\left(\sum_{k=1}^n b_k^2\right)
$$


Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis id sem purus, eu commodo tortor. Donec malesuada ultricies dolor a eleifend. In hac habitasse platea dictumst. Vivamus a faucibus ligula. Nullam molestie tristique arcu, eu elementum metus ultricies sed. Aenean luctus congue lectus, vitae semper erat rhoncus non. Nulla facilisi.

Horizontal rule:

---

|Table Header 1|Table Header 2|
|----|----|
|Content|	https://github.com/laymonage/giscus|
|Content|	http://github.com:\<test>|
Text right below table. Follows is a table with an empty cell, and unaligned indenting.

|Table Header 1	| Table Header 2|
|---|---|
|Content|	Content|
|Content| |


    def this_is
      puts "some #{4-space-indent} code"
    end

<code>
def this_is
  puts "some #{code tag} code"
end
</code>

<pre>
def this_is
  puts "some #{pre tag} code"
end
</pre>

```
def this_is
    puts "some #{fenced} code"
end
```

```ruby
class Classy
  def this_is
    puts "some #{colored} ruby code with ruby syntax highlighting"
    @someobj.do_it(1, 2)
  end
end
```

```javascript
var test = function this_is(){
  console.log("some" + colored + "javascript code with javascript syntax highlighting really long");
}
```

```clojure
(defproject myproject "0.5.0-SNAPSHOT"
  :description "Some clojure code with syntax highlighting."
  :dependencies [[org.clojure/clojure "1.5.1"]]
  :plugins [[lein-tar "3.2.0"]])
```

```js
var test = function this_is(){
  console.log("language declared as 'js' instead");
}
```

```bogus_language
var test = function this_is(){
  console.log("language declared as bogus_language");
}
```

```diff
- print('hello world')
+ print('Hello, world!')
```