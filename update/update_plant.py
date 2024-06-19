from file_io.write_json import write_json
from file_io.load_json import load_plant_file
from common.constants import date_format, plants_dir
from common.types import PlantInformation
import os 

update_fields : list[str] = ['plant_name','species_name','origin','obtained','auto_watering','plant_notes','current_location','plant_health']

def get_update_info() -> tuple[str,str,str]: 
    plant_name : str = input('Enter plant name ').strip()

    update_key : str = ''

    while update_key not in update_fields:
        print('enter field to update:')
        print('possible fields: %s' % ', '.join(update_fields))
        update_key = input('').strip()

    update_value : str = input('Please enter new value (notes will be appended): ').strip()
    return (plant_name,update_key,update_value)


def update_plant_dict(info_dict:dict[str,str]) -> None:
    update_plant(info_dict['plant_name'],info_dict['update_key'],info_dict['update_value'])

def update_plant(plant_name:str, update_key:str, update_value:str) -> None:
    plant_file_name : str = plant_name.replace(' ','')+'.json'
    print(plant_file_name)

    plant_path : str = os.path.join(plants_dir,plant_file_name)
    plant_information : PlantInformation = load_plant_file(plant_path)
    
    new_plant_information : dict[str,str | list[str]] = {
            'plant_name'       : plant_information['plant_name'],
            'species_name'     : plant_information['species_name'],
            'plant_health'     : str(plant_information['plant_health']),
            'current_location' : plant_information['current_location'],
            'origin'           : plant_information['origin'],
            'obtained'         : plant_information['obtained'].strftime(date_format),
            'auto_watering'    : str(plant_information['auto_water']),
            'plant_notes'      : plant_information['plant_notes']
            }
    if update_key == 'plant_notes':
        current_notes : list[str] = list(new_plant_information['plant_notes'])
        current_notes.append(update_value)
        new_plant_information['plant_notes'] = current_notes
    elif update_key in new_plant_information:
        new_plant_information[update_key] = update_value

    write_json(new_plant_information,plants_dir,plant_file_name)
