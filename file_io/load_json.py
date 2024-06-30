from common.types import * 
from common.constants import * 
from common.PlantSpecies import PlantSpecies

import os 
import json

def load_plant_file(file_name:str) -> PlantInformation:
    plant_file = open(file_name,'r')
    file_contents = plant_file.read()
    json_contents = json.loads(file_contents)
    plant_info = coalesce_plant(json_contents)
    return plant_info 

def load_species_file(file_name:str) -> SpeciesInformation:
    plant_file = open(file_name,'r')
    file_contents = plant_file.read()
    file_json = json.loads(file_contents)
    species_info = coalesce_species(file_json)
    return species_info 

def load_plants() -> list[PlantInformation]:
    dir_list = []
    for file_name in os.listdir(plants_dir):
        full_name = os.path.join(plants_dir,file_name)
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

def load_species() -> list[PlantSpecies]:
    species_infos : list[SpeciesInformation] = load_all_species_files()
    species_list  : list[PlantSpecies] = []

    for species_info in species_infos:
        new_species= PlantSpecies(species_info)
        species_list.append(new_species)
    return species_list

def load_bot_config() -> tuple[str,list[int]]:
    config_file_path = os.path.join(bot_dir,bot_config_name)
    config_file = open(config_file_path,'r')
    config_contents = json.loads(config_file.read())
    api_key : str = config_contents['api_key']
    whitelist : list[int] = config_contents['white_list']
    return (api_key,whitelist)

def check_plant_exists(plant_name:str) -> bool:
    plant_file_name : str = plant_name.replace(' ','')+'.json'
    plant_path : str = os.path.join(plants_dir,plant_file_name)
    return os.path.isfile(plant_path)

def check_species_exists(species_name:str) -> bool:
    species_file_name : str = species_name.replace(' ','')+'.json'
    species_path : str = os.path.join(species_dir,species_file_name)
    return os.path.isfile(species_path)
