from update_json.add_species import create_species
from update_json.add_plant   import create_plant

import os 
import json
import sys
 
species_dir = 'PlantSpecies'
plant_dir = 'Plants'

def write_json(json_dict:dict[str,str],out_dir:str,out_file_name:str) -> None:
    out_file_path = os.path.join(out_dir,out_file_name)
    out_file = open(out_file_path,'w+')

    out_file.write(json.dumps(json_dict))
    out_file.close()
    print('Saved %s'%out_file_name)


if __name__ == '__main__':
    action : str = sys.argv[1]
    match action:
        case 'add-species':
            species_information = create_species()
            species_file_name = species_information['name'].replace(' ','_')+ '.json'
            write_json(species_information,species_dir,species_file_name)
            exit(0)
        case 'add-plant':
            plant_information = create_plant()
            plant_file_name = plant_information['plant_name'].replace(' ','_')+'.json'
            write_json(plant_information,plant_dir,plant_file_name)
            exit(0)

