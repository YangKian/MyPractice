#######################
# calculate square root
#######################


def square_root(a):
    """
    use to calculate a square root for a number
    >>> square_root(4)
    2.0
    >>> square_root(16)
    4.0
    """
    x = 1
    while x * x != a:
        x = square_root_update(x, a)
    return x


def square_root_update(x, a):
    return (x + a/x) / 2

############################
# calculate cube root
############################


def cube_root(a):
    """
    use to calculate a cube root of a number
    >>> cube_root(27)
    3.0
    >>> cube_root(729)
    9.0
    """
    x = 1
    while x * x * x != a:
        x = cube_root_update(x, a)
    return x


def cube_root_update(x, a):
    return (2 * x + a / (x * x)) / 3

############################
# General version
############################


def square_root_improve(a):
    """
    use to calculate a square root for a number
    >>> square_root_improve(4)
    2.0
    >>> square_root_improve(16)
    4.0
    """
    def update(x):
        return square_root_update(x, a)

    def close(x):
        return approx_eq(x*x, a)
    return improve(update, close)


def cube_root_improve(a):
    """
    use to calculate a cube root of a number
    >>> cube_root_improve(27)
    3.0
    >>> cube_root_improve(729)
    9.0
    """
    return improve(lambda x: cube_root_update(x, a),
                   lambda x: approx_eq(x*x*x, a))


def improve(update, close, guess=1, max_update=100):
    k = 0
    while not close(guess) and k < max_update:
        guess = update(guess)
        k += 1
    return guess


def approx_eq(x, y, tolerance=1e-15):
    return abs(x - y) < tolerance