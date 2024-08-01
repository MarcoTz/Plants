from update.parse_input import get_date
from common.types import PlantInformation,LogItem,GrowthItem
from file_io.load_json import load_plant_file
from file_io.load_csv import load_activities,load_growth
from file_io.write_csv import write_csv
from common.constants import * 

import os

def create_graveyard() -> dict[str,str]:
    print('-- Move plant to graveyard --')

    graveyard_plant  : str = input('Enter plant name: ')
    died_date        : str = get_date('Enter died date (dd.mm.yyyy): ')
    graveyard_reason : str = input('Enter reason of death: ')

    return {
            'graveyard_plant':graveyard_plant,
            'graveyard_died_date' : died_date,
            'graveyard_reason': graveyard_reason
    }

def remove_plant_images(plant_name:str,img_dir:str,img_small_dir:str) -> None:
    plant_imgs       : list[str] = os.listdir(img_dir)
    plant_imgs = list(map(lambda x: os.path.join(img_dir,x),plant_imgs))
    plant_imgs_small : list[str] = os.listdir(img_small_dir)
    plant_imgs_small = list(map(lambda x: os.path.join(img_small_dir,x),plant_imgs_small))
    all_imgs : list[str] = plant_imgs
    all_imgs.extend(plant_imgs_small)
    for img_path in all_imgs:
        img_name_split : list[str] = img_path.split('_')
        if plant_name in img_name_split[0] and len(img_name_split) == 2:
            os.remove(img_path)


def remove_plant_activities(plant_activities,plant_name:str,date_format:str) -> list[dict[str,str]]:
    plant_activities_csv : list[dict[str,str]] = []

    for plant in plant_activities.keys():
        if plant==plant_name:
            continue
        for log_item in plant_activities[plant]:
            new_activity = {
                'Date':log_item['log_date'].strftime(date_format),
                'Activity':log_item['log_activity'],
                'Plants':plant,
                'Note':log_item['log_note'],
            }
            plant_activities_csv.append(new_activity)
    return plant_activities_csv

def remove_plant_growth(plant_growth,plant_name:str,date_format:str) -> list[dict[str,str]]:
    plant_growth_csv : list[dict[str,str]] = []

    for plant in plant_growth.keys():
        if plant == plant_name:
            continue
        for log_item in plant_growth[plant]:
            new_growth = {
                    'Date':log_item['log_date'].strftime(date_format),
                    'Plant':plant,
                    'Height':str(log_item['log_height_cm']),
                    'Width':str(log_item['log_width_cm']),
                    'Note':log_item['log_note'],
                    }
            plant_growth_csv.append(new_growth)

    return plant_growth_csv


def move_to_graveyard(graveyard_info:dict[str,str]) -> None:
    plant_name : str = graveyard_info['graveyard_plant']

    plant_file_path : str = os.path.join(plants_dir,plant_name+'.json')
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

    plant_img_dir : str = os.path.join(out_dir,img_dir,img_plants_dir)
    plant_img_small_dir : str = os.path.join(plant_img_dir,img_small_dir)
    remove_plant_images(plant_name,plant_img_dir,plant_img_small_dir)

    plant_activities : dict[str,list[LogItem]] = load_activities()
    plant_activities_csv : list[dict[str,str]] = remove_plant_activities(plant_activities,plant_name,date_format)
    write_csv(plant_activities_csv,log_dir,activity_log_file_name,True)

    plant_growth: dict[str,list[GrowthItem]] = load_growth()
    plant_growth_csv : list[dict[str,str]] = remove_plant_growth(plant_growth,plant_name,date_format)
    write_csv(plant_growth_csv,log_dir,growth_log_file_name,True)


