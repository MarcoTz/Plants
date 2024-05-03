from common.common import * 

import csv
import os
import datetime

def load_activities() -> list[LogItem]:
    log_list : list[LogItem] = []

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
                'log_plant':log_plant,
                'log_note':log_row['Note']
                }
            log_list.append(new_log_item)
    return log_list 

def load_growth() -> list[GrowthItem]:
    growth_list : list[GrowthItem] = [] 

    log_file_path : str = os.path.join(log_dir,growth_log_file_name)
    log_file = open(log_file_path,'r')
    reader : csv.DictReader = csv.DictReader(log_file,delimiter=';')
    for log_row in reader:
        new_growth_item : GrowthItem = {
                'log_date' : datetime.datetime.strptime(log_row['Date'] ,date_format),
                'log_plant':log_row['Plant'],
                'log_height_cm' : float(log_row['Height']),
                'log_width_cm' : float(log_row['Width']),
                'log_note': log_row['Note']
                }
        growth_list.append(new_growth_item)
    return growth_list 
