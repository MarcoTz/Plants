from update.parse_input import get_float,get_lines,get_date
from file_io.write_csv import write_csv
from file_io.write_json import write_json
from common.constants import date_format, plants_dir,log_dir,growth_log_file_name

import datetime 

def create_plant():

    print('--Create new Plant--')
    plant_name   : str       = input('Enter Plant name: ').strip()
    species_name : str       = input('Enter plant species: ').strip()
    height       : float     = get_float('Enter current height (in cm): ')
    width        : float     = get_float('Enter current width (in cm): ')
    location     : str       = input('Enter current location: ').strip()
    origin       : str       = input('Enter plant origin: ').strip()
    obtained     : str       = get_date('Enter obtained date (dd.mm.yyyy): ') 
    print('Enter additional notes (leave line blank to finish)')
    plant_notes  : list[str] = get_lines()
    

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


def add_plant(plant_information:dict[str,str]) -> None:
    plant_name : str = plant_information['plant_name']
    plant_file_name : str = plant_name.replace(' ','')+'.json'
    current_date : str = datetime.datetime.now().strftime(date_format)
    first_growth : dict[str,str] = {
        'log_date':current_date,
        'log_plant':plant_name,
        'log_height_cm':plant_information['current_height'],
        'log_width_cm':plant_information['current_width'],
        'log_note':'Added during plant creation'}

    write_json(plant_information,plants_dir,plant_file_name)
    write_csv([first_growth],log_dir,growth_log_file_name)

