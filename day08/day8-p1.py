tree_map = []

with open('input.txt', 'r') as file:
    for line in file:
        tree_map.append([int(x) for x in line.rstrip()])

# Trees on edge
visible = len(tree_map) * 4 - 4


def visible_from_outside(pos, height):
    r, c = pos
    visibility = [True, True, True, True]
    
    # Top
    for i in range(0, r):
        if tree_map[i][c] >= height:
            visibility[0] = False
            break
    
    # Left
    for i in range(0, c):
        if tree_map[r][i] >= height:
            visibility[1] = False
            break

    # Bottom
    for i in range(len(tree_map)-1, r, -1):
        if tree_map[i][c] >= height:
            visibility[2] = False
            break

    # Right
    for i in range(len(tree_map)-1, c, -1):
        if tree_map[r][i] >= height:
            visibility[3] = False
            break
    
    return True in visibility


for i in range(1, len(tree_map)-1):
    for j in range(1, len(tree_map)-1):
        if visible_from_outside((i, j), tree_map[i][j]):
            visible += 1

print(visible)  # Answer: 1785
