velocidade=  input ("insira a velocidade do codutor: ")

if int(velocidade)>120:
    print("A velocidade acima do permitido, reduza a velocidae")
elif int(velocidade)<60:
    print ("A velocidade esta abaixo do permitido, aumente a velocidade")
else:
    print("velocidade correta")
