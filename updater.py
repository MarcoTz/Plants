from update.add_species  import create_species
from update.add_plant    import create_plant
from update.add_activity import create_multiple_activities
from update.add_growth   import create_multiple_growth
from common.common       import *

import os 
import json
import sys
import csv
import datetime
 
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
            plant_information : dict[str,str] = create_plant()
            plant_name : str = plant_information['plant_name']
            plant_file_name : str = plant_name.replace(' ','')+'.json'
            current_date : str = datetime.datetime.now().strftime(date_format)
            first_growth : dict[str,str] = {
                    'log_date':current_date,
                    'log_plant':plant_name,
                    'log_height_cm':plant_information['current_height'],
                    'log_width_cm':plant_information['current_width'],
                    'log_note':'Added during plant creation'}

            write_json(plant_information,plant_dir,plant_file_name)
            write_csv([first_growth],log_dir,growth_log_file_name)
            exit(0)
        case 'add-activities':
            log_items = create_multiple_activities()
            write_csv(log_items,log_dir,activity_log_file_name)
            exit(0)
        case 'add-growth':
            log_items = create_multiple_growth()
            write_csv(log_items,log_dir,growth_log_file_name)
            exit(0)
