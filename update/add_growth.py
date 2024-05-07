from update.parse_input import get_float, get_date
from file_io.write_csv  import write_csv
from common.constants   import *

def create_growth(): 
    print('-- Add Growth --')

    log_plant : str = input('Enter Plant Name: ').strip()
    log_date  : str       = get_date('Enter log date (dd.mm.yyyy): ')

    height_cm : float = get_float('Enter height (cm): ')
    width_cm  : float = get_float('Enter width (cm): ')

    log_note : str = input('Enter additional Note: ').strip()
    log_note = ' ' if log_note is None else log_note

    return { 
            'log_date':log_date,
            'log_plant':log_plant,
            'log_height_cm':height_cm,
            'log_width_cm':width_cm,
            'log_note':log_note}


def create_multiple_growth():
    activity = create_growth()
    another_one = input('Enter anoher line (y/n): ').strip().lower()
    if another_one == 'y':
        activities = create_multiple_growth()
        activities.insert(0,activity)
        return activities
    return [activity]

def add_growth(log_items:list[dict[str,str]]) -> None:
    write_csv(log_items,log_dir,growth_log_file_name)

