from datetime import datetime

class Funcionarios:
    def __init__(self, nome,sobrenome, ano_nascimento):
        self.nome=nome
        self.sobrenome=sobrenome
        self.ano_nascimento=ano_nascimento

    def nome_completo(self):
        return self.nome +" "+self.sobrenome
    
    def idade_funcionario(self):
        ano = datetime.now().year
        self.idade=int(ano-self.ano_nascimento)
        return self.idade
     

# criae objecto
usuario1=Funcionarios("Antonio", "Junior",2003)
usuario2=Funcionarios("Ana", "matrins", 2007)
usuario3=Funcionarios("Gilberto", "carbral", 1999)

print(usuario3.nome_completo())
print(Funcionarios.idade_funcionario(usuario3))
