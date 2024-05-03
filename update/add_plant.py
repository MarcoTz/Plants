import datetime 

def get_notes() -> list[str]:
    new_note:str = input('').strip()
    if new_note == '':
        return []
    else:
        rest_notes :list[str] = get_notes()
        rest_notes.insert(0,new_note)
        return rest_notes

def create_plant():

    print('--Create new Plant--')
    plant_name   : str       = input('Enter Plant name: ').strip()
    species_name : str       = input('Enter plant species: ').strip()
    height       : float     = float(input('Enter current height (in cm): '))
    width        : float     = float(input('Enter current width (in cm): '))
    location     : str       = input('Enter current location: ').strip()
    origin       : str       = input('Enter plant origin: ').strip()
    obtained     : str       = input('Enter obtained date (dd.mm.yyyy): ').strip()
    datetime.datetime.strptime(obtained,'%d.%m.%Y')
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
