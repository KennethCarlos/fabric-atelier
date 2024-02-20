
alunos={"nome":"kenneth",
         "idade":19, 
         "media": 14,
         "aprovaçao": True
        }

alunos.update({"nome":"Ana"})#para trocar o valor de um o mais key
alunos.update({"ano": 12})#adicior uma key e seu valor

del alunos["ano"]#apaga keys

print(alunos)

print("#####################################################")

######################################################################

for i,j in alunos.items():
    print(i,j)


print("#####################################################")

######################################################################

alunos.update({"disciplinas": ["ALGA, Prog, ATC"]})
print(alunos)
print(len(alunos))#quantas keys tem
