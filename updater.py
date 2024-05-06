from update.add_species  import create_species
from update.add_plant    import create_plant
from update.add_activity import create_multiple_activities
from update.add_growth   import create_multiple_growth
from update.move_to_graveyard   import move_to_graveyard,remove_plant_images,remove_plant_activities,remove_plant_growth
from common.load_json           import load_plant_file
from common.load_csv            import load_activities,load_growth
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

def write_csv(csv_items:list[dict[str,str]],out_dir:str,out_file_name:str,overwrite:bool=False) -> None:
    out_file_path : str = os.path.join(out_dir,out_file_name)
    write_mode : str = 'w' if overwrite else 'a' 
    out_file = open(out_file_path,write_mode)
    csv_fields : list[str] = list(csv_items[0].keys())
    
    writer : csv.DictWriter = csv.DictWriter(out_file,delimiter=';',fieldnames=csv_fields)
    if overwrite:
        writer.writeheader()

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
        case 'move-to-graveyard': 
            graveyard_info : dict[str,str] = move_to_graveyard()
            plant_name : str = graveyard_info['graveyard_plant']
            plant_file_path : str = os.path.join(plant_dir,plant_name+'.json')
            current_plant_info : PlantInformation = load_plant_file(plant_file_path)

            full_graveyard_info : dict[str,str] = {
                    'graveyard_name':plant_name,
                    'graveyard_species':current_plant_info['species_name'],
                    'graveyard_planted':current_plant_info['obtained'].strftime(date_format),
                    'graveyard_died':graveyard_info['graveyard_died_date'],
                    'graveyard_reason':graveyard_info['graveyard_reason'] 
                    }
            write_csv([full_graveyard_info],log_dir,graveyard_file_name)
            os.remove(plant_file_path)

            plant_img_dir : str = os.path.join(img_dir,img_plants_dir)
            plant_img_small_dir : str = os.path.join(plant_img_dir,img_small_dir)
            remove_plant_images(plant_name,plant_img_dir,plant_img_small_dir)

            plant_activities : dict[str,list[LogItem]] = load_activities()
            plant_activities_csv : list[dict[str,str]] = remove_plant_activities(plant_activities,plant_name,date_format)
            write_csv(plant_activities_csv,log_dir,activity_log_file_name,True)

            plant_growth: dict[str,list[GrowthItem]] = load_growth()
            plant_growth_csv : list[dict[str,str]] = remove_plant_growth(plant_growth,plant_name,date_format)
            write_csv(plant_growth_csv,log_dir,growth_log_file_name,True)
            exit(0)
