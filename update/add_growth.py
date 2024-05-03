import datetime

def get_float(prompt:str) -> float:
    nr : str = input(prompt)
    try:
        nr_float = float(nr)
        return nr_float
    except ValueError:
        print('Could not parse number, please try again')
        return get_float(prompt)

def get_date() -> str:
    date_str = input('Enter log date (dd.mm.yyyy): ').strip()
    try:
        datetime.datetime.strptime(date_str,'%d.%m.%Y')
        return date_str
    except ValueError:
        print('Could not parse date, please try again')
        return get_date()

def create_growth(): 
    print('-- Add Growth --')

    log_plant : str = input('Enter Plant Name: ').strip()
    log_date    : str       = get_date() 

    height_cm : float = get_float('Enter height (cm): ')
    width_cm : float = get_float('Enter width (cm): ')

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
