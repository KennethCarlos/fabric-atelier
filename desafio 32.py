import math

l= [1, 2, 3, 4, 5, 6]
resul=[]

for i in l:
    Q= lambda i: math.pow(i,2)
    resul.append(Q(i))

print (f'os quadrados de {l} sao {resul}')