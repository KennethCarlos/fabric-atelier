
# understand how dictionaries work
number = {"one": 1, "two": 2, "three": 3}

print(number["one"], number["two"], number["three"])

number["one"]= 10

print(number["one"])

print()
#first letter of each word in the list
planets = ['Mercury', 'Venus', 'Earth', 'Mars', 'Jupiter', 'Saturn', 'Uranus', 'Neptune']

initial_planets={planet: planet[0] for planet in planets}

print(initial_planets)
print()

#last letter of each word in the list

last_letter_planets= {planet: planet[-1] for planet in planets}

print(last_letter_planets) 
print()

#mid letter of each word in the list

middle_char_planets = {planet: planet[len(planet) // 2] if len(planet) % 2 != 0 else planet[(len(planet) // 2) - 1] for planet in planets}

print(middle_char_planets)
print()

#use dictionaries in a loop

for k in number:
    print("{} = {}".format(k, number[k]))# note that k will be the name of the variable and number[k] the variable it self
    
for k in number:    
    print("{} begins with {}". format(k, number[k]))
print()

#
# learn to use formar. PS: don't have anything to do with dictionars
print("first name: {}, last name: {}".format("kennneth", "Manhica"))

print()

#sort all intial in alphabetical order, them add a space in between

K= ' '.join(sorted(initial_planets.values()))

print (K)
print

# spectial loop and use of formt

for planet, initial in initial_planets.items():
    print("{} begins with \"{}\"".format(planet, initial))
    """
    planet in the loop refers to the name of the valiable in the dictionary "initial_planets"
    initialt in the loop refers to the value of valiable "pannet" in the dictionary"initial_planets"
    """
