tree_map = []

with open('input.txt', 'r') as file:
    for line in file:
        tree_map.append([int(x) for x in line.rstrip()])


def scenic_score(pos, height):
    r, c = pos
    # Initialise with max score
    viewing_distance = [r, c, len(tree_map) - r - 1, len(tree_map) - c - 1]
    
    # Top
    for i in range(r-1, -1, -1):
        if tree_map[i][c] >= height:
            viewing_distance[0] = r-i
            break
    
    # Left
    for i in range(c-1, -1, -1):
        if tree_map[r][i] >= height:
            viewing_distance[1] = c-i
            break

    # Bottom
    for i in range(r+1, len(tree_map)):
        if tree_map[i][c] >= height:
            viewing_distance[2] = i-r
            break

    # Right
    for i in range(c+1, len(tree_map)):
        if tree_map[r][i] >= height:
            viewing_distance[3] = i-c
            break

    return viewing_distance[0] * viewing_distance[1] * viewing_distance[2] * viewing_distance[3]


scores = []

for i in range(1, len(tree_map)-1):
    for j in range(1, len(tree_map)-1):
        scores.append(scenic_score((i, j), tree_map[i][j]))

print(max(scores))  # Answer: 345168
