import datetime 

def get_notes() -> list[str]:
    new_note:str = input('').strip()
    if new_note == '':
        return []
    else:
        rest_notes :list[str] = get_notes()
        rest_notes.insert(0,new_note)
        return rest_notes

def get_float(prompt:str) -> float:
    nr : str = input(prompt)
    try: 
        nr_float = float(nr)
        return nr_float
    except ValueError:
        print('Could not parse number, please try again')
        return get_float(prompt)

def get_date() -> str:
    date_str = input('Enter obtained date (dd.mm.yyyy): ').strip()
    try:
        datetime.datetime.strptime(date_str,'%d.%m.%Y')
        return date_str
    except ValueError: 
        print('Could not parse date, please try again')
        return get_date()


def create_plant():

    print('--Create new Plant--')
    plant_name   : str       = input('Enter Plant name: ').strip()
    species_name : str       = input('Enter plant species: ').strip()
    height       : float     = get_float('Enter current height (in cm): ')
    width        : float     = get_float('Enter current width (in cm): ')
    location     : str       = input('Enter current location: ').strip()
    origin       : str       = input('Enter plant origin: ').strip()
    obtained     : str       = get_date() 
    print('Enter additional notes (leave line blank to finish)')
    plant_notes  : list[str] = get_notes()
    

    return { 
            'plant_name'       : plant_name,
            'species_name'     : species_name,
            'current_height'   : height,
            'current_width'    : width,
            'current_location' : location,
            'origin'           : origin,
            'obtained'         : obtained,
            'plant_notes'      : plant_notes 
            }
