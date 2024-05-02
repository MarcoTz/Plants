import classes.PlantSpecies

import os 
import json

def load_plant(file_name:str) -> classes.PlantSpecies.PlantSpecies:
    plant_file = open(file_name,'r')
    file_contents = plant_file.read()
    plant_json = json.loads(file_contents)
    return classes.PlantSpecies.PlantSpecies(plant_json)

plant_dir = 'PlantSpecies'
def load_all_plants() -> list[classes.PlantSpecies.PlantSpecies]:
    plant_list = []
    for file_name in os.listdir(plant_dir):
        full_name = os.path.join(plant_dir,file_name)
        if os.path.isdir(full_name):
            continue
        plant_list.append(load_plant(full_name))
    return plant_list
