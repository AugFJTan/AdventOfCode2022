from filesystem import init_filesystem

total = 0

def depth_first_search(directory):
    global total

    if directory.get_size() <= 100000:
        total += directory.get_size()
        
    if len(directory.subdirectories) == 0:
        return

    for subdirectory in directory.subdirectories:
        depth_first_search(subdirectory)


root = init_filesystem()

depth_first_search(root)

print(total)  # Answer: 1243729
