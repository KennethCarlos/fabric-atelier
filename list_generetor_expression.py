from sys import getsizeof

numeros=[i*10 for i in range(10)]
print(type(numeros))
#print(numeros)
print(getsizeof(numeros))

print("====================================")

Numeros=(i*10 for i in range(10))
print(type(Numeros))
#print(list(Numeros))
print(getsizeof(Numeros))