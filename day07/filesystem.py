class Directory:
    def __init__(self, name):
        self.name = name
        self.parent = None
        self.files = []
        self.subdirectories = []
    
    def get_subdirectory(self, name):
        for directory in self.subdirectories:
            if directory.name == name:
                return directory
        return None  # Unlikely, but just in case
    
    def get_size(self):
        total = 0
        
        total += sum(self.files)
        
        for directory in self.subdirectories:
            total += directory.get_size()
        
        return total


def init_filesystem():
    root = Directory('/')
    current_directory = root

    with open('input.txt', 'r') as file:
        file.readline()

        for line in file:
            data = line.rstrip()
            
            if data[0] == '$':
                command = data.split()
                
                if command[1] == 'cd':
                    if command[2] == '..':
                        current_directory = current_directory.parent
                    else:
                        current_directory = current_directory.get_subdirectory(command[2])
            else:
                contents = data.split()
                
                if contents[0] == 'dir':
                    new_directory = Directory(contents[1])
                    new_directory.parent = current_directory
                    
                    current_directory.subdirectories.append(new_directory)
                else:
                    current_directory.files.append(int(contents[0]))

    return root
