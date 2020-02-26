
def cascade(n):
    """
    >>> cascade(123)
    123
    12
    1
    12
    123
    """
    if n < 10:
        print(n)
    else:
        print(n)
        cascade(n // 10)
        print(n)


def cascade_short(n):
    """
    >>> cascade_short(123)
    123
    12
    1
    12
    123
    """
    print(n)
    if n >= 10:
        cascade(n // 10)
        print(n)


def f_then_g(f, g, n):
    if n:
        f(n)
        g(n)


grow = lambda n: f_then_g(grow, print, n // 10)
shrink = lambda n: f_then_g(print, shrink, n // 10)


def inverse_cascade(n):
    grow(n)
    print(n)
    shrink(n)


def fib(n):
    if n == 0:
        return 0
    elif n == 1:
        return 1
    return fib(n - 2) + fib(n - 1)


if __name__ == "__main__":
    print(fib(5))

