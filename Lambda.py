
somar= lambda x,y: x*y

print( somar(2,2))

print("##############################################################")

########################################################################~

def mulp(x):
    f=lambda x: x+10
    return f(x)*4

print(mulp(2))

####################
numeros=[1, 2, 3, 4, 5, 6]
quadrados=lambda num: num**2
resultados=[]

for i in numeros:
    resultados.append(quadrados(i))

print(resultados)
