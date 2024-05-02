import LoadJSON

plants = LoadJSON.load_all_plants() 

for plant in plants:
    print(plant.show())
