from update.parse_input import get_float, get_date
from update.update_plant import update_plant
from file_io.write_csv  import write_csv
from common.constants   import *

def create_growth(): 
    print('-- Add Growth --')

    log_plant : str = input('Enter Plant Name: ').strip()
    log_date  : str       = get_date('Enter log date (dd.mm.yyyy): ')

    height_cm : float = get_float('Enter height (cm): ')
    width_cm  : float = get_float('Enter width (cm): ')

    plant_health : int = int(get_float('Enter current health (0-5): '))

    log_note : str = input('Enter additional Note: ').strip()
    log_note = ' ' if log_note is None else log_note

    return { 
            'growth_date':log_date,
            'growth_plant':log_plant,
            'growth_height':height_cm,
            'growth_width':width_cm,
            'growth_note':log_note,
            'plant_health': plant_health},



def create_multiple_growth():
    activity = create_growth()
    another_one = input('Enter anoher line (y/n): ').strip().lower()
    if another_one == 'y':
        activities = create_multiple_growth()
        activities.insert(0,activity)
        return activities
    return [activity]

def add_growth(log_items:list[dict[str,str]]) -> None:
    growth_items = []
    for growth_item in log_items:
        update_health = growth_item['plant_health']
        plant_name : str = growth_item['log_plant']
        update_plant(plant_name, 'plant_health',update_health)
        growth_item.pop('plant_health')
        growth_items.append(growth_item)
    write_csv(growth_items,log_dir,growth_log_file_name)
