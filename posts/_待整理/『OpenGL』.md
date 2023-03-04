# OpenGL

## 一、什么是 OpenGL？

> [glspec46.core]() - 1.2

OpenGL（Open Graphics Library） 是一个图形硬件的 API（Application Programming Interface）。

对于程序员来说，OpenGL 是一组命令，允许对着色器程序或光影，光影所使用的数据，以及在光影之外对OpenGL的状态控制。

对于实现者来说，OpenGL 是一组命令，控制显卡的操作。

## 二、处理模型

OpenGL 只关心处理在GPU内存中的数据，保活向屏幕缓冲区渲染内容以及从屏幕缓冲区读取内容。

OpenGL 绘制 由多种着色器程序和被上下文状态控制的固定管线处理单元处理的 *primitives（图元）*，图元是点、线段或几何图形。

## GLFW

### 0. 安装CMake

### 1. 获取源码并进行构建

> 官网：[Download | GLFW](https://www.glfw.org/download.html) Source package

> Github：[Releases · glfw/glfw (github.com)](https://github.com/glfw/glfw/releases) Release中的 `glfw-3.3.6.zip`

1. 使用CMake生成工程

   打开CMake-gui

   选择源码目录与构建目录，Configure选择VS对应已安装版本，VS需要安装“使用C++的桌面开发”（包含用于Windows的C++CMake工具）

   再次点击 Configure 来保存设置，随后点击 Generate 生成工程文件。

2. 使用 VS 构建

   打开build文件夹中的GLFW.sln，点击生成解决方案。

   编译出的库glfw3.lib则会出现在src/Debug文件夹内

### 2. 创建新的项目

链接GLFW

解决方案资源管理器中右键解决方案 - 属性

VC++目录 - 包含目录 中添加 GLFW 的 include 文件夹

​                  - 库目录 中添加 glfw3.lib所在文件夹



链接器 的 Input 中的 附加依赖项 添加 glfw3.lib

以及 opengl32.lib库（Windows自己就有，直接加就好）

GLAD：[https://glad.dav1d.de](https://glad.dav1d.de/)

### 3. helloworld

```c++
#include <glad/glad.h>
#include <GLFW/glfw3.h>

int main()
{
    glfwInit();
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    
    return 0;
}
```

首先调用 `glfwinit()` 初始化 **GLFW**

然后通过 `glfwWindowHint()` 来配置 **GLFW**，第一个参数表示选项名称，第二个参数为选项值。（见：[GLFW: Window guide](https://www.glfw.org/docs/latest/window.html#window_hints)）

随后新建一个窗口对象：

```C++
GLFWwindow* window = glfwCreateWindow(800, 600, "LearnOpenGL", NULL, NULL);
if (window == NULL)
{
    std::cout << "Failed to create GLFW window" << std::endl;
    glfwTerminate();
    return -1;
}
glfwMakeContextCurrent(window);
```

再初始化 **GLAD**，给 **GLAD** 传入了用来加载系统相关的OpenGL函数指针地址的函数也就是 **GLFW** 提供的 glfwGetProcAddress

```c++
if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress))
{
    std::cout << "Failed to initialize GLAD" << std::endl;
    return -1;
}
```

设定视口

```C++
glViewport(0, 0, 800, 600);
```

前两个参数控制窗口左下角的位置。第三个和第四个参数控制渲染窗口的宽度和高度（像素）。

> OpenGL幕后使用glViewport中定义的位置和宽高进行2D坐标的转换，将OpenGL中的位置坐标转换为你的屏幕坐标。例如，OpenGL中的坐标(-0.5, 0.5)有可能（最终）被映射为屏幕中的坐标(200,450)。注意，处理过的OpenGL坐标范围只为-1到1，因此我们事实上将(-1到1)范围内的坐标映射到(0, 800)和(0, 600)。

写一个相应窗口大小改变的回调函数：

```c++
void framebuffer_size_callback(GLFWwindow* window, int width, int height)
{
    glViewport(0, 0, width, height);
}
```

注册它

```c++
glfwSetFramebufferSizeCallback(window, framebuffer_size_callback);
```

渲染循环：

```c++
while(!glfwWindowShouldClose(window))
{
    glfwSwapBuffers(window);
    glfwPollEvents();    
}
```

- glfwWindowShouldClose函数在我们每次循环的开始前检查一次GLFW是否被要求退出，如果是的话该函数返回`true`然后渲染循环便结束了，之后为我们就可以关闭应用程序了。
- glfwPollEvents函数检查有没有触发什么事件（比如键盘输入、鼠标移动等）、更新窗口状态，并调用对应的回调函数（可以通过回调方法手动设置）。
- glfwSwapBuffers函数会交换颜色缓冲（它是一个储存着GLFW窗口每一个像素颜色值的大缓冲），它在这一迭代中被用来绘制，并且将会作为输出显示在屏幕上。

> **双缓冲(Double Buffer)**
>
> 应用程序使用单缓冲绘图时可能会存在图像闪烁的问题。 这是因为生成的图像不是一下子被绘制出来的，而是按照从左到右，由上而下逐像素地绘制而成的。最终图像不是在瞬间显示给用户，而是通过一步一步生成的，这会导致渲染的结果很不真实。为了规避这些问题，我们应用双缓冲渲染窗口应用程序。**前**缓冲保存着最终输出的图像，它会在屏幕上显示；而所有的的渲染指令都会在**后**缓冲上绘制。当所有的渲染指令执行完毕后，我们**交换**(Swap)前缓冲和后缓冲，这样图像就立即呈显出来，之前提到的不真实感就消除了。

最后循环结束后释放、删除所有资源：

```c++
glfwTerminate();
```



#### 处理输入：

```c++
void processInput(GLFWwindow *window)
{
    if(glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_PRESS)
        glfwSetWindowShouldClose(window, true);
}
```

#### 渲染：

```c++
// 渲染循环
while(!glfwWindowShouldClose(window))
{
    // 输入
    processInput(window);

    // 渲染指令
    ...

    // 检查并调用事件，交换缓冲
    glfwPollEvents();
    glfwSwapBuffers(window);
}
```



```c++
glClearColor(0.2f, 0.3f, 0.3f, 1.0f);
glClear(GL_COLOR_BUFFER_BIT);
```

glClearColor来设置清空屏幕所用的颜色。

我们可以通过调用glClear函数来清空屏幕的颜色缓冲，它接受一个缓冲位(Buffer Bit)来指定要清空的缓冲，可能的缓冲位有GL_COLOR_BUFFER_BIT，GL_DEPTH_BUFFER_BIT和GL_STENCIL_BUFFER_BIT。由于现在我们只关心颜色值，所以我们只清空颜色缓冲。

> !!!!!!!!!!!!!!!状 态 机!!!!!!!!!!!!!!!
>
> glClearColor函数是一个**状态设置**函数，而glClear函数则是一个**状态使用**的函数，它使用了当前的状态来获取应该清除为的颜色。
>
> !!!!!!!!!!!!!!!状 态 机!!!!!!!!!!!!!!!



## 画一个三角形

OpenGL 中有这样一个命令：`void GenBuffers(sinzei n, uint *buffers);`

会生成n个没有使用的缓冲对象名存储到buffers中。

还有一个命令：`void BindBuffer(enum target, uint buffer)`

会将刚才的缓冲对象名绑定到target指定的缓冲对象类型上（0表示取消绑定）

缓冲对象类型：

| 目标名                      | 用处                 |
| --------------------------- | -------------------- |
| `ARRAY_BUFFER`              | 顶点属性             |
| `ATOMIC_COUNTER_BUFFER`     | 原子的 计数存储      |
| `COPY_READ_BUFFER`          | 缓冲复制源           |
| `COPY_WRITE_BUFFER`         | 缓冲复制目的地       |
| `DISPATCH_INDIRECT_BUFFER`  | 不直接的计算调度命令 |
| `DRAW_INDIRECT_BUFFER`      | 不直接的命令参数     |
| `ELEMENT_ARRAY_BUFFER`      | 顶点数组切片         |
| `PARAMETER_BUFFER`          | 绘制参数             |
| `PIXEL_PACK_BUFFER`         | 像素读取目标         |
| `PIXEL_UNPACK_BUFFER`       | 材质数据源           |
| `QUERY_BUFFER`              | 请求结果缓冲         |
| `SHADER_STORAGE_BUFFER`     | 着色器读写存储       |
| `TEXTURE_BUFFER`            | 材质数据缓冲         |
| `TRANSFORM_FEEDBACK_BUFFER` | 变换反馈缓冲         |
| `UNIFORM_BUFFER`            | Uniform block存储    |

介绍一个 `ARRAY_BUFFER`，它

顶点缓冲对象（VertexBufferObjects，VBO）被用于管理GPU中存储顶点的内存。