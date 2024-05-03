from update.add_species  import create_species
from update.add_plant    import create_plant
from update.add_activity import create_multiple_activities
from common.common       import *

import os 
import json
import sys
import csv
 
species_dir = 'PlantSpecies'
plant_dir = 'Plants'

def write_json(json_dict:dict[str,str],out_dir:str,out_file_name:str) -> None:
    out_file_path = os.path.join(out_dir,out_file_name)
    out_file = open(out_file_path,'w+')

    out_file.write(json.dumps(json_dict))
    out_file.close()
    print('Saved %s'%out_file_name)

def write_csv(csv_items:list[dict[str,str]],out_dir:str,out_file_name:str) -> None:
    out_file_path : str = os.path.join(out_dir,out_file_name)
    out_file = open(out_file_path,'a')
    csv_fields : list[str] = list(csv_items[0].keys())
    
    writer : csv.DictWriter = csv.DictWriter(out_file,delimiter=';',fieldnames=csv_fields)
    for csv_item in csv_items:
        writer.writerow(csv_item)
    print('Wrote log to %s' % out_file_name)


if __name__ == '__main__':
    action : str = sys.argv[1]
    match action:
        case 'add-species':
            species_information = create_species()
            species_file_name = species_information['name'].replace(' ','')+ '.json'
            write_json(species_information,species_dir,species_file_name)
            exit(0)
        case 'add-plant':
            plant_information = create_plant()
            plant_file_name = plant_information['plant_name'].replace(' ','')+'.json'
            write_json(plant_information,plant_dir,plant_file_name)
            exit(0)
        case 'add-activities':
            log_items = create_multiple_activities()
            write_csv(log_items,log_dir,activity_log_file_name)
            exit(0)
