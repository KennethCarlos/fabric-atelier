ponto=int(input("qual é a temperartura da carne: "))
#ponto=int(ponto)

if ponto<48:
    print("consinhar por mais tempo")
elif ponto<54:
    print("o ponto da carne é rare (selada)")
elif ponto<60:
    print("o ponto da carne é medium rare (ao ponto para mal passada)")
elif ponto<65:
    print("o ponto da carne é medium (ao ponto)")
elif ponto<71:
    print("o ponto da carne é medium well (ao ponto para bem passada)")
else:
    print("o ponto da carne é well done (bem passada)") 