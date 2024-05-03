def get_notes() -> list[str]:
    new_note:str = input('').strip()
    if new_note == '':
        return []
    else:
        rest_notes :list[str] = get_notes()
        rest_notes.insert(0,new_note)
        return rest_notes


def create_species():
    print('--Create new Plant Species--')
    species_name        : str           = input('Enter species (common) name:').strip()
    scientific_name     : str           = input('Enter scientific name:').strip()
    sunlight_str        : str           = input('Enter sunlight requirements (direct/indirect/shade):').strip()
    min_temp            : float         = float(input('Enter minimal (survivable) temperature (in C): '))
    max_temp            : float         = float(input('Enter maximal (survivable) temperature (in C): '))
    opt_min_temp        : float         = float(input('Enter optimal temperature lower end (in C): '))
    opt_max_temp        : float         = float(input('Enter optimal temperature upper end (in C): '))
    plant_distance      : int           = int(input('Enter minimal distance between plants (in cm): '))
    ph_min              : float         = float(input('enter minimal pH value: '))
    ph_max              : float         = float(input('Enter maximal pH value: '))
    print('Enter watering notes (leave line blank to finish)')
    watering_notes      : list[str]     = get_notes()
    print('Enter fertilizing notes (leave line blank to finish)')
    fertilizing_notes   : list[str]     = get_notes()
    print('Enter pruning notes (leave line blank to finish)')
    pruning_notes       : list[str]     = get_notes()
    print('Enter companion plants (one per line, leave blank to finish)')
    companions          : list[str]     = get_notes()
    print('Enter any additional notes (leave line blank to finish')
    notes               : list[str]     = get_notes()

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
            "fertilizing_notes"         : fertilizing_notes,
            "pruning_notes"             : pruning_notes,
            "companions"                : companions,
            "additional_notes"          : notes 
            }

