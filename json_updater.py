from update_json.add_species import create_species

import os 
import json

species_dir = 'PlantSpecies'

def write_json(json_dict:dict[str,str],out_dir:str,out_file_name:str) -> None:
    out_file_path = os.path.join(out_dir,out_file_name)
    out_file = open(out_file_path,'w+')

    out_file.write(json.dumps(json_dict))
    out_file.close()
    print('Saved %s'%out_file_name)


if __name__ == '__main__':
   species_information = create_species()
   species_file_name = species_information['name']+'.json'
   write_json(species_information,species_dir,species_file_name)
