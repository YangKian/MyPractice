def trees(labels, branches=[]):
    for branch in branches:
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


def fib_tree(n):
    """
     >>> fib_tree(1)
     [1]
    >>> fib_tree(0)
    [0]
    >>> fib_tree(2)
    [1, [0], [1]]
    >>> fib_tree(4)
    [3, [1, [0], [1]], [2, [1], [1, [0], [1]]]]
    >>> label(fib_tree(4))
    3
    """
    if n <= 1:
        return trees(n)
    else:
        left, right = fib_tree(n - 2), fib_tree(n - 1)
        return trees(label(left) + label(right), [left, right])


def count_leaves(t):
    """Count the leaves of a tree
    >>> count_leaves(fib_tree(4))
    5
    """
    if is_leaf(t):
        return 1
    else:
         return sum([count_leaves(b) for b in branches(t)])


def leaves(tree):
    """
    Return a list containing the leaf labels of tree.
    >>> leaves(fib_tree(5))
    [1, 0, 1, 0, 1, 1, 0, 1]
    """
    if is_leaf(tree):
        return [label(tree)]
    else:
        # list of leaf labels for each branch
        return sum([leaves(b) for b in branches(tree)], [])


def increment_leaves(t):
    """Return a tree like t but with leaf labels incremented."""
    if is_leaf(t):
        return trees(label(t) + 1)
    else:
        # only work for leaf node
        bs = [increment_leaves(b) for b in branches(t)]
        return trees(label(t), bs)


def increment(t):
    """Return a tree like t but with all labels incremented."""
    # base case: 如果不存在分支，如到达了叶节点，则[increment(b) for b in branches(t)] 生成的是空列表，代表没有工作需要做
    return trees(label(t) + 1, [increment(b) for b in branches(t)])


def print_tree(t, indent=0):
    print('	' * indent + str(label(t)))
    for b in branches(t):
        print_tree(b, indent + 1)