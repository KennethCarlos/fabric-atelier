cores=["verde", "branco", "amarelo", "vermelho", "preto"]

cliente= input("digite a cor desejada: ")
if cliente.lower() in cores:
    print("esta na lista")
else:
    print("nao esta na lista ")