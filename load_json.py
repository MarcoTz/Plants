from classes.PlantSpecies import PlantSpecies,SpeciesInformation
from classes.Plant        import Plant,PlantInformation

import os 
import json

species_dir : str = 'PlantSpecies'
plant_dir   : str = 'Plants'

def load_plant_file(file_name:str) -> PlantInformation:
    plant_file = open(file_name,'r')
    file_contents = plant_file.read()
    return json.loads(file_contents)
def load_species_file(file_name:str) -> SpeciesInformation:
    plant_file = open(file_name,'r')
    file_contents = plant_file.read()
    return json.loads(file_contents)

def load_all_plant_files() -> list[PlantInformation]:
    dir_list = []
    for file_name in os.listdir(plant_dir):
        full_name = os.path.join(plant_dir,file_name)
        if os.path.isdir(full_name):
            continue
        json_dir = load_plant_file(full_name)
        dir_list.append(json_dir)
    return dir_list 

def load_all_species_files() -> list[SpeciesInformation]:
    dir_list = []
    for file_name in os.listdir(species_dir):
        full_name = os.path.join(species_dir,file_name)
        if os.path.isdir(full_name):
            continue
        json_dir = load_species_file(full_name)
        dir_list.append(json_dir)
    return dir_list 

def load_plants() -> list[Plant]:
    plant_infos : list[PlantInformation] = load_all_plant_files()
    plant_list  : list[Plant] = []
    for plant_info in plant_infos:
        new_plant = Plant(plant_info)
        plant_list.append(new_plant)
    return plant_list

def load_species() -> list[PlantSpecies]:
    species_infos : list[SpeciesInformation] = load_all_species_files()
    species_list  : list[PlantSpecies] = []

    for species_info in species_infos:
        new_species= PlantSpecies(species_info)
        species_list.append(new_species)
    return species_list

def load_plants_species() -> tuple[list[Plant],list[PlantSpecies]]:
    plants  : list[Plant]        = load_plants()
    species : list[PlantSpecies] = load_species()
    return (plants,species)
