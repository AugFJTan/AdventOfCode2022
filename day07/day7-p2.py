from filesystem import init_filesystem

root = init_filesystem()
root_size = root.get_size()

space_to_free = root_size - 40000000
smallest = root_size

def depth_first_search(directory):
    global smallest

    if directory.get_size() >= space_to_free:
        smallest = min(smallest, directory.get_size())
        
    if len(directory.subdirectories) == 0:
        return

    for subdirectory in directory.subdirectories:
        depth_first_search(subdirectory)


depth_first_search(root)

print(smallest)  # Answer: 4443914
