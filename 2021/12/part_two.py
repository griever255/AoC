import string

filename = "input.txt"

with open(filename, 'r') as f:
    input = f.read().splitlines()
f.close()

# Build out the graph of Node: [Destination]
graph = {}
for line in input:
    node1, node2 = line.split("-")
    if node1 not in graph:
        graph[node1] = [node2]
    else:
        graph[node1].append(node2)
    if node2 not in graph:
        graph[node2] = [node1]
    else:
        graph[node2].append(node1)

def isBigCave(cave):
    if cave[0] in string.ascii_uppercase:
        return True
    else:
        return False

def smallCaveFlag(path):
    for node in path:
        if node[0] in string.ascii_lowercase:
            if path.count(node) == 2:
                return True
    return False

def findPath(graph, start, end, path=[]):
    path = path + [start]
    if start == end:
        return [path]
    if start not in graph.keys():
        return []
    paths = []
    for node in graph[start]:
        if (not smallCaveFlag(path) or isBigCave(node) or node not in path) and not node == "start":
            new_paths = findPath(graph, node, end, path)
            for new_path in new_paths:
                paths.append(new_path)
    return paths

valid_paths = findPath(graph, "start", "end")
print(len(valid_paths))
