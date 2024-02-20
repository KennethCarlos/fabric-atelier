list=[10, 20, 30, 40, 50]
List=[10, 20, 60, 70]

num=set(list)
Num=set(List)

print(num|Num)#une os set e imprime sem repetiçao
print(num-Num)#impremi os elementos diferentes de NUm em num
print(num^Num)#imprimi os numeros sem repeiçoes
print(num&Num)#imprimi somente os duplicados
print("###############################################################")

#####################################################################

list1=set([1, 2, 3, 4, 5]) 
S1={1, 2, 3, 4, 5}# criar set diretamente
S1.add("que vida")#adicionar algo ao set
S1.update([6, 7, 8, "incansavelmente"])#adicionar mais de um  elemento
S1.remove("incansavelmente")#remover item do set
S1.discard(30)#remove itens sem estarem no set
print(S1)
print("###############################################################")

#############################################################

s1={"A", "B", "C"}
s2={"A", "D", "E"}
s3={"C", "D", "F"}

print(s1.union(s2))#uni os set, mas sem repetir os itens
print(s1.difference(s3))#imprimi os itens do "s1" ausentes no "s3"
print(s1.intersection(s2))#imprime os elementos comuns entre s1 e s2
print(s1.symmetric_difference(s3))#nao imprime os diferentes