#include <stdio.h>

#define FORALL_REGS(_) _(X) _(Y)
#define LOGIC X1 = !X && Y; \
              Y1 = !X && !Y;

#define DEFINE(x) static int x, x##1;
#define UPDATE(x) x = x##1;
#define PRINT(x)  printf(#x " = %d; ", x);

/**
 *  使用预编译：
 *  优点：
 *      1.增加/删除寄存器只需要修改一个地方；
 *      2.阻止了一些编程错误：
 *          a.忘记更新寄存器
 *          b.忘记打印寄存器
 *      3.语义明晰
 *  缺点：
 *      1.可读性变差了（更不像 C 代码了）
 *      2.给 IDE 解析带来一些困难
 */
int main() {
    FORALL_REGS(DEFINE);
    // 宏展开后实际上变成了：
    // -> DEFINE(X) DEFINE(Y)
    // -> static int X, X1; static int Y, Y1;
    // 如果这个时候我需要再增加一个变量 Z，则只需要修改宏定义：
    // #define FORALL_REGS(_) _(X) _(Y) _(Z), 就会完成自动生成代码，增加容错性

    while (1) {
        FORALL_REGS(PRINT); putchar('\n'); sleep(1);
        LOGIC;
        FORALL_REGS(UPDATE);
    }
}
