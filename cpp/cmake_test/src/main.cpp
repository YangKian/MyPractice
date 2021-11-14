#include <static/Hello.h>
#include "shared/Hello_shared.h"

int main() {
    Hello_Dyn hi;
    hi.print();

    Hello hi_stc;
    hi_stc.print();

    return 0;
}
