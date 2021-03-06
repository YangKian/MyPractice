#include <stdint.h>

typedef uint8_t u8;
#define NMEM 16

enum { RA, R1, R2, R3, PC, NREG };
extern u8 M[NMEM], R[NREG];
#define pc (R[PC])

void idex_bad() {
    u8 inst = M[pc++];
    u8 op = inst >> 4;
    if (op == 0x0 || op == 0x1) {
        int rt = (inst >> 2) & 3, rs = (inst & 3);
        if (op == 0x0) R[rt] = R[rs];
        else if (op == 0x1) R[rt] = R[rs];
    }
    if (op == 0xe || op == 0xf) {
        int addr = inst & 0xf;
        if (op == 0xe) R[0] = M[addr];
        else if (op == 0xf) M[addr] = R[0];
    }
}

// 优化
/**
 * 指针：
 *  - 内存只是个字节序列
 *  - 无论何种类型的指针都只是地址 + 对指向内存的解读
 *
 * 这个联合体的含义：
 *  inst_t 指向了一段内存，通过联合体的定义，这段内存可以有
 *  两种解读方式，rtype 或者 mtype
 *      - 对于 rtype，从低位到高位，指定前 2 个字节为 rs，中间 2 个字节为 rt，最后 4 个字节为 op
 *          即： |  op  | rt | rs |
 *      - 对于 mtype，从低位到高位，指定前 4 个字节为 addr，后 4 个字节为 op
 *          即：|  op  |  addr  |
 */
typedef union inst {
    struct (u8 rs: 2, rt: 2, op: 4; ) rtype; // bit fields 语法特性
    struct (u8 addr: 4, op: 4; ) mtype;
} inst_t;
#define RTYPE(i) u8 rt = (i)->rtype.rt, rs = (i)->rtype.rs;
#define MTYPE(i) u8 addr = (i)->mtype.addr;

void index() {
    inst_t *cur = (inst_t *)&M[pc]; // 使用一个指针（inst_t *) 指向 M[pc] 的位置
    switch (cur->rtype.op) { // 根据当前指令的 op code 来执行不同的操作
        case 0b0000: { RTYPE(cur); R[rt] = R[rs]; pc++; break; }
        case 0b0001: { RTYPE(cur); R[rt] += R[rs]; pc++; break; }
        case 0b1110: { MTYPE(cur); R[RA] = M[addr]; pc++; break; }
        case 0b1111: { MTYPE(cur); M[addr] = R[RA]; pc++; break; }
        default: panic("invalid instruction at PC = %x", pc);
    }
}

