valores=[10, 12, 15, 34, 54, 62]

def removev20(x):
    return x>20

print(list(map(removev20,valores)))
print(list(filter(removev20,valores)))
print(list(filter(lambda x: x>20, valores)))