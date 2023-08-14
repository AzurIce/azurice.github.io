---
date: 2023-03-04
---

## 一、安装 PySide6

```shell
pip install pyside6
```

编写一个 `.py` 运行测试一下：

```python
import PySide6.QtCore

# 打印 PySide6 版本
print(PySide6.__version__)

# 打印用于编译 PySide6 的 Qt 版本
print(PySIde6.QtCore.__version__)
```

## 二、来写个 HelloWorld 吧！

```python
import sys
import random
from PySide6 import QtCore, QtWidgets, QtGui

class MyWidget(QtWidgets.QWidget):
    def __init__(self):
        super().__init__()
        
        self.hello = ["Hello World!", "Hello AzurIce!", "Hello PySide!"]
        
        self.button = QtWidgets.QPushButton("Click me!")
        self.text = QtWidgets.QLabel("Hello World!",
                                     alignment=QtCore.Qt.AlignCenter)
        
        self.layout = QtWidgets.QVBoxLayout(self)
        self.layout.addWidget(self.text)
        self.layout.addWidget(self.button)
        
        self.button.clicked.connect(self.magic)
        
    @QtCore.Slot()
    def magic(self):
        self.text.setText(random.choice(self.hello))

if __name_ == "__main__":
    app = QtWidgets.QApplication([])
    
    widget = MyWidget()
    widget.resize(800, 600)
    widget.show()
    
    sys.exit(app.exec())
```

