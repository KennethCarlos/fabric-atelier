try:
    letras=[1, 2, 3]
    print(letras[3])
except IndexError:
    print("index nao existe")

print("#########################################################################")
##################################################################################
try:
    valor=int(input("degite o valor do seu produto: "))
    print(valor)
except ValueError:
    print("degite um valor numero")
else:
    print("calculando a taxa")
    r=valor*2
    print(r)
finally:
    print("ok")

print("#########################################################################")
##################################################################################
