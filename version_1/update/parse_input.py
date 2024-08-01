import datetime 

def get_lines()->list[str]:
    new_plant = input('').strip()
    if new_plant == '':
        return []
    rest_plants = get_lines()
    rest_plants.insert(0,new_plant)
    return rest_plants

def get_date(prompt:str) -> str:
    log_date    : str       = input(prompt).strip()
    try:
        datetime.datetime.strptime(log_date,'%d.%m.%Y')
        return log_date
    except ValueError:
        print('Could not parse date, please try again')
        return get_date(prompt)


def get_float(prompt:str) -> float:
    nr : str = input(prompt)
    try:
        nr_float = float(nr)
        return nr_float
    except ValueError:
        print('Could not parse number, please try again')
        return get_float(prompt)
