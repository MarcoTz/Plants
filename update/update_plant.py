from file_io.write_json import write_json
from file_io.load_json import load_plant_file
from common.constants import date_format, plants_dir
from common.types import PlantInformation
import os 

update_fields : list[str] = ['plant_name','species_name','origin','obtained','plant_notes','current_location']

def get_update_info() -> tuple[str,str,str]: 
    plant_name : str = input('Enter plant name ').strip()

    update_key : str = ''

    while update_key not in update_fields:
        print('enter field to update:')
        print('possible fields: %s' % ', '.join(update_fields))
        update_key = input('').strip()

    update_value : str = input('Please enter new value (notes will be appended): ').strip()
    return (plant_name,update_key,update_value)



def update_plant(plant_name:str, update_key:str, update_value:str) -> None:
    plant_file_name : str = plant_name.replace(' ','')+'.json'
    print(plant_file_name)

    plant_path : str = os.path.join(plants_dir,plant_file_name)
    plant_information : PlantInformation = load_plant_file(plant_path)
    
    new_plant_information : dict[str,str] = {
            'plant_name'       : plant_information['plant_name'],
            'species_name'     : plant_information['species_name'],
            'current_location' : plant_information['current_location'],
            'origin'           : plant_information['origin'],
            'obtained'         : plant_information['obtained'].strftime(date_format),
            'plant_notes'      : ''.join(plant_information['plant_notes'])
            }
    if update_key == 'plant_notes':
        current_notes : str = new_plant_information['plant_notes'].strip()
        current_notes += ',' + update_value if current_notes != '' else update_value
    if update_key in new_plant_information:
        new_plant_information[update_key] = update_value

    write_json(new_plant_information,plants_dir,plant_file_name)
