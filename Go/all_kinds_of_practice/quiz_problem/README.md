# Quiz Problem
csv 文件中存储了问题和答案，用户通过命令行读入问题，并给出自己的答案，程序自动进行比对，给出结果

### 快速开始
在当前文件夹下输入以下命令：
`go build . && quiz_problem`

参数补充：

`-csv`：设置要打开的csv文件，如：`-csv=problems.csv`

`-limit`：支持设置答题时间，单位为秒，如：`-limit=30`

`-h`：获取帮助
