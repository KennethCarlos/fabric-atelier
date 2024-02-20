limite=input("insira o limite: ")
limite= int(limite)
for i in range(1,limite+1):
    for j in range (1,i+1):
        print(i*j ,end=" ")
    print()

