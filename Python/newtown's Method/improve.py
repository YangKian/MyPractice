def root(n, a):
    """
    use to calculate the nth root of a number
    :param n: nth root of a
    :param a: the number to calculate the nth root

    >>> root(2, 4)
    2.0
    >>> root(2, 16)
    4.0
    >>> root(3, 729)
    9.0
    >>> root(6, 64)
    2.0
    """
    def f(x):  # x^n = a => f(x) = x^n - a = 0, so we can use newtown's method to find x
        return power(x, n) - a

    def df(x):  # df(x) = n * x^(n-1)
        return n * power(x, n - 1)
    return find_zero(f, df)


def find_zero(f, df):
    def near_zero(x):
        return approx_eq(f(x), 0)
    return improve(newtown_update(f, df), near_zero)


def newtown_update(f, df):
    def update(x):
        return x - f(x) / df(x)
    return update


def power(x, n):
    """
    return x * x * x * x ... * x for x repeated n times
    """
    product, k = 1, 0
    while k < n:
        product, k = product * x, k + 1
    return product


def improve(update, close, guess=1, max_update=100):
    k = 0
    while not close(guess) and k < max_update:
        guess = update(guess)
        k += 1
    return guess


def approx_eq(x, y, tolerance=1e-15):
    return abs(x - y) < tolerance