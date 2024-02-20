import random

ans=random.randint(1,100)
guess=int
print(guess)

while ans != guess:
    guess=int(input("adivinhe o numero entre 1 e 100: "))
    if guess<ans:
        print (f"O numero é maior que {guess}")
    elif guess>ans:
        print (f"O numero é menor que {guess}")
    
print(f"voce acerotou!!!")

