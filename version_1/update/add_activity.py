from update.parse_input import get_date,get_lines
from file_io.write_csv  import write_csv
from common.constants   import * 

def create_activity() -> dict[str,str]: 
    print('-- Add Activity --')

    log_date    : str       =  get_date('Enter log date (dd.mm.yyyy): ')

    log_activity : str = input('Enter Activity: ').strip()
    print('Enter affected plants (one per line, leave line blank to finish')
    log_plants : list[str] = get_lines()

    log_note : str = input('Enter additional Note: ').strip()
    log_note = ' ' if log_note is None else log_note

    return { 
            'log_date':log_date,
            'log_activity':log_activity,
            'log_plants':','.join(log_plants),
            'log_note':log_note}


def create_multiple_activities() -> list[dict[str,str]]:
    activity = create_activity()
    another_one = input('Enter anoher line (y/n): ').strip().lower()
    if another_one == 'y':
        activities = create_multiple_activities()
        activities.insert(0,activity)
        return activities
    return [activity]

def add_activities(log_items:list[dict[str,str]]) -> None:
    write_csv(log_items,log_dir,activity_log_file_name)
