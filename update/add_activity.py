import datetime 

def get_plants()->list[str]:
    new_plant = input('').strip()
    if new_plant == '':
        return []
    rest_plants = get_plants()
    rest_plants.insert(0,new_plant)
    return rest_plants

def create_activity(): 
    print('-- Add Activity --')

    log_date    : str       = input('Enter log date (dd.mm.yyyy): ').strip()
    datetime.datetime.strptime(log_date,'%d.%m.%Y')

    log_activity : str = input('Enter Activity: ').strip()
    print('Enter affected plants (one per line, leave line blank to finish')
    log_plants : list[str] = get_plants()

    log_note : str = input('Enter additional Note: ').strip()
    log_note = ' ' if log_note is None else log_note

    return { 
            'log_date':log_date,
            'log_activity':log_activity,
            'log_plants':','.join(log_plants),
            'log_note':log_note}


def create_multiple_activities():
    activity = create_activity()
    another_one = input('Enter anoher line (y/n): ').strip().lower()
    if another_one == 'y':
        activities = create_multiple_activities()
        activities.insert(0,activity)
        return activities
    return [activity]
