valor=int(input("insira o preço inicial do valor: "))
limite=int(input("insira o valor minimo: "))
dia=1
sub=valor*.1
while valor>limite:
  print(f" No {dia}º dia o produto vai ser vendido a {valor}MTS")
  dia+= 1
  valor-=sub