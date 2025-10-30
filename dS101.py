from sklearn import tree
 
#each value represente [height, weight, she size]

P=[[170,62,40], [160, 70, 40], [180, 80, 44], [175, 75, 43], [165, 60, 39], [155, 55, 38], [185, 85, 45], [178, 68, 41], [162, 62, 40], [172, 78, 42.5], [168, 66, 41]]

G= ["male", "female", "male", "male", "female", "female", " male", "male", "female"," male", "female" ]

#clf-classifier

#
clf=tree.DecisionTreeClassifier()

clf= clf.fit(P,G)

prediction = clf.predict([[170,60,40]]) 

print(prediction)
