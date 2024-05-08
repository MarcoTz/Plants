from update.parse_input import get_float,get_lines
from file_io.write_json import write_json
from common.constants   import species_dir

def create_species():
    print('--Create new Plant Species--')
    species_name        : str           = input('Enter species (common) name:').strip()
    scientific_name     : str           = input('Enter scientific name:').strip()
    sunlight_str        : str           = input('Enter sunlight requirements (direct/indirect/shade):').strip()
    min_temp            : float         = get_float('Enter minimal (survivable) temperature (in C): ')
    max_temp            : float         = get_float('Enter maximal (survivable) temperature (in C): ')
    opt_min_temp        : float         = get_float('Enter optimal temperature lower end (in C): ')
    opt_max_temp        : float         = get_float('Enter optimal temperature upper end (in C): ')
    plant_distance      : float         = get_float('Enter minimal distance between plants (in cm): ')
    ph_min              : float         = get_float('enter minimal pH value: ')
    ph_max              : float         = get_float('Enter maximal pH value: ')
    avg_watering_days   : int           = int(get_float('Enter average days between waterings'))
    print('Enter watering notes (leave line blank to finish)')
    watering_notes      : list[str]     = get_lines()
    avg_fertilizing_days: int           = int(get_float('Enter average days between fertilizing'))
    print('Enter fertilizing notes (leave line blank to finish)')
    fertilizing_notes   : list[str]     = get_lines()
    print('Enter pruning notes (leave line blank to finish)')
    pruning_notes       : list[str]     = get_lines()
    print('Enter companion plants (one per line, leave blank to finish)')
    companions          : list[str]     = get_lines()
    print('Enter any additional notes (leave line blank to finish')
    notes               : list[str]     = get_lines()

    return {
            "name"                      : species_name,
            "scientific_name"           : scientific_name,
            "sunlight_requirements"     : sunlight_str,
            "temperature_min"           : min_temp,
            "temperature_max"           : max_temp,
            "optimal_temperature_min"   : opt_min_temp,
            "optimal_temperature_max"   : opt_max_temp,
            "plant_distance_cm"         : plant_distance,
            "ph_min"                    : ph_min,
            "ph_max"                    : ph_max,
            "watering_notes"            : watering_notes,
            "avg_watering_days"         : avg_watering_days,
            "fertilizing_notes"         : fertilizing_notes,
            "avg_fertilizing_days"      : avg_fertilizing_days,
            "pruning_notes"             : pruning_notes,
            "companions"                : companions,
            "additional_notes"          : notes 
            }

def add_species(species_information:dict[str,str]) -> None:
    species_file_name = species_information['name'].replace(' ','')+ '.json'
    write_json(species_information,species_dir,species_file_name)

