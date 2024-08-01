from common.types  import * 
from common.constants import * 
import csv
import os
import datetime

def load_activities() -> dict[str,list[LogItem]]:
    log_dict : dict[str,list[LogItem]] = {}

    log_file_path : str = os.path.join(log_dir,activity_log_file_name)
    log_file = open(log_file_path,'r')
    reader : csv.DictReader = csv.DictReader(log_file,delimiter=';')
    for log_row in reader:
        log_plants : list[str] = log_row['Plants'].split(',')
        log_plants : list[str] = list(map(lambda x: x.strip(),log_plants))
        log_date   : datetime.datetime = datetime.datetime.strptime(log_row['Date'],date_format)

        for log_plant in log_plants:
            new_log_item : LogItem = {
                'log_activity': log_row['Activity'],
                'log_date': log_date,
                'log_note':log_row['Note']
                }
            if log_plant not in log_dict:
                log_dict[log_plant] = [new_log_item]
            else:
                log_dict[log_plant].append(new_log_item)
    return log_dict

def load_growth() -> dict[str,list[GrowthItem]]:
    growth_dict : dict[str,list[GrowthItem]] = {}

    log_file_path : str = os.path.join(log_dir,growth_log_file_name)
    log_file = open(log_file_path,'r')
    reader : csv.DictReader = csv.DictReader(log_file,delimiter=';')
    for log_row in reader:
        new_growth_item : GrowthItem = {
                'log_date'      : datetime.datetime.strptime(log_row['Date'] ,date_format),
                'log_height_cm' : float(log_row['Height']),
                'log_width_cm'  : float(log_row['Width']),
                'log_note'      : log_row['Note']
                }
        if log_row['Plant'] not in growth_dict:
            growth_dict[log_row['Plant']] = [new_growth_item]
        else:
            growth_dict[log_row['Plant']].append(new_growth_item)
    return growth_dict

def load_graveyard() -> list[GraveyardPlant]:
    graveyard_list : list[GraveyardPlant] = []
    graveyard_file_path : str = os.path.join(log_dir,graveyard_file_name)
    graveyard_file = open(graveyard_file_path,'r')
    reader : csv.DictReader = csv.DictReader(graveyard_file,delimiter=';')
    for graveyard_row in reader:
        new_graveyard_item : GraveyardPlant = {
                'graveyard_plant'  : graveyard_row['Name'] ,
                'graveyard_species': graveyard_row['Species'],
                'graveyard_planted': datetime.datetime.strptime(graveyard_row['Planted'],date_format),
                'graveyard_died'   : datetime.datetime.strptime(graveyard_row['Died'],date_format),
                'graveyard_reason' : graveyard_row['Reason']
                }
        graveyard_list.append(new_graveyard_item)
    return graveyard_list
