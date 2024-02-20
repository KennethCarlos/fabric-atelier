import random

print("r.rock \np.paper \ns.scissors" )
user=(input("chose your answer: "))

comp=random.choice(["r", "p", "s"])

if (user == "r" and comp == "s") or (user == "s" and comp == "p") or (user == "p" and comp == "r"):
    print("You won!!! :)")
elif user == comp:
    print("it's a tie")
else:
    print("You lose :(")
