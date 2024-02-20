numeros=[1, 2, 3, 4, 5, 6]
quadrados=lambda num: num**2
resultados=[]

for i in numeros:
    resultados.append(quadrados(i))

print(resultados)

