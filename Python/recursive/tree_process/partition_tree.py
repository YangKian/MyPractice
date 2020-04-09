def trees(labels, branches=[]):
    for branch in branches: # Verifies the tree definition
        assert is_tree(branch), 'branches must be trees'
    return [labels] + list(branches)


def label(tree):
    return tree[0]


def branches(tree):
    return tree[1:]


def is_tree(tree):
    if type(tree) != list or len(tree) < 1:
        return False
    for branch in branches(tree):
        if not is_tree(branch):
            return False
    return True


def is_leaf(tree):
    return not branches(tree)


def partition_tree(n, m):
    """
    Return a partition tree of n using parts of up to m
    >>> partition_tree(2, 2)
    [2, [True], [1, [1, [True], [False]], [False]]]
    """
    if n == 0:
        return trees(True)
    elif n < 0 or m == 0:
        return trees(False)
    else:
        left = partition_tree(n - m, m)
        right = partition_tree(n, m - 1)
        return trees(m, [left, right])


def print_parts(tree, partition=[]):
    """
    >>> print_parts(partition_tree(6, 4))
    4 + 2
    4 + 1 + 1
    3 + 3
    3 + 2 + 1
    3 + 1 + 1 + 1
    2 + 2 + 2
    2 + 2 + 1 + 1
    2 + 1 + 1 + 1 + 1
    1 + 1 + 1 + 1 + 1 + 1
    """
    if is_leaf(tree):
        if label(tree):
            print(' + '.join(partition))
    else:
        left, right = branches(tree)
        m = str(label(tree))
        print_parts(left, partition + [m])
        print_parts(right, partition)
