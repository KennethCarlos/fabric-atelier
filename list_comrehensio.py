frutas=["banana", "abacate", "morango", "kiwi", "abacaxi"]
frutas2=[]

for i in frutas:
    if "b" in i:
        frutas2.append(i)

print(frutas2)   

frutas3= [i for i in frutas if "n" in i  ]
print(frutas3)
print("#################################################################")

########################################################################

preco=[i*10 for i in range(1,6) ]
print(preco)