from file_io.write_json import write_json
from file_io.load_json  import load_species_file
from common.constants   import species_dir
from common.types       import SpeciesInformation
import os 

update_fields : list[str] = [
        "name",
        "scientific_name",
        "species_type",
        "sunlight_requirements",
        "temperature_min",
        "temperature_max",
        "optimal_temperature_min",
        "optimal_temperature_max"   ,
         "plant_distance_cm",
         "ph_min",
         "ph_max",
         "watering_notes",
         "avg_watering_days",
         "fertilizing_notes",
         "avg_fertilizing_days",
         "pruning_notes",
         "companions",
         "additional_notes"]

append_fields : list[str] = ["watering_notes","fertilizing_notes","pruning_notes","companions","additional_notes"]


def get_update_info() -> tuple[str,str,str]:
    species_name : str = input('Enter species name: ').strip()
    update_key : str = ''

    while update_key not in update_fields:
        print('Please enter field to update')
        print('Possible fields: %s' % ', '.join(update_fields))
        update_key = input().strip()

    update_value : str = input('Please enter new value (notes will be appended): ').strip()

    return (species_name,update_key,update_value)

def update_species_dict(info_dict:dict[str,str])->None:
    update_species(info_dict['species_name'],info_dict['update_key'],info_dict['update_value'])

def update_species(species_name:str, update_key:str, update_value:str) -> None: 
    species_file_name : str = species_name.replace(' ','') +'.json'
    species_path : str = os.path.join(species_dir,species_file_name)
        
    species_info : SpeciesInformation = load_species_file(species_path)

    new_species_info : dict[str,str|list[str]] = {
            "name"                      : species_info['name'],
            "scientific_name"           : species_info['scientific_name'],
            "species_type"              : species_info['species_type'],
            "sunlight_requirements"     : species_info['sunlight_requirements'],
            "temperature_min"           : str(species_info['temperature_min']),
            "temperature_max"           : str(species_info['temperature_max']),
            "optimal_temperature_min"   : str(species_info['optimal_temperature_min']),
            "optimal_temperature_max"   : str(species_info['optimal_temperature_max']),
            "plant_distance_cm"         : str(species_info['plant_distance_cm']),
            "ph_min"                    : str(species_info['ph_min']),
            "ph_max"                    : str(species_info['ph_max']),
            "watering_notes"            : species_info['watering_notes'],
            "avg_watering_days"         : str(species_info['avg_watering_days']),
            "fertilizing_notes"         : species_info['fertilizing_notes'],
            "avg_fertilizing_days"      : str(species_info['avg_fertilizing_days']),
            "pruning_notes"             : species_info['pruning_notes'],
            "companions"                : species_info['companions'],
            "additional_notes"          : species_info['additional_notes']
            }

    if update_key in append_fields:
        current_value:list[str] = list(new_species_info[update_key])
        current_value.append(update_value)
        new_species_info[update_key] = current_value
    elif update_key in new_species_info:
        new_species_info[update_key] = update_value

    write_json(new_species_info,species_dir,species_file_name)
