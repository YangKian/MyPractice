import re
import collections

Node = collections.namedtuple('node', 'node_index, value, children')

'''
    use stack to abstract the catalog of the file
'''
if __name__ == '__main__':
    with open('raw.txt', 'r', encoding='utf8') as file:
        content = file.read()
        pattern = re.compile(r"={2,} .*\n")
        match = re.findall(pattern, content)
        if match:
            match = [x.replace('=', '#').replace('&#x2014;', ' - ') for x in match]

    titles = [Node(node_index=x.count('#'), value=x, children=[]) for x in match]

    stack = []
    stack.append(titles[0])

    for title in titles[1:]:
        if stack[-1].node_index > title.node_index:
            temp = []
            while len(stack) != 0 and stack[-1].node_index > title.node_index:
                temp.append(stack.pop())
            stack[-1].children.extend(temp)
            stack.append(title)
        else:
            stack.append(title)

    temp = []
    while len(stack) != 1:
        node = stack.pop()
        if stack[-1].node_index < node.node_index:
            stack[-1].children.append(node)
            stack[-1].children.extend(temp)
            temp = []
        else:
            temp.append(node)

    [print(i) for i in stack]
