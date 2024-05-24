from update.add_species         import create_species,add_species
from update.add_plant           import create_plant,add_plant
from update.add_activity        import create_multiple_activities,add_activities
from update.add_growth          import create_multiple_growth,add_growth
from update.move_to_graveyard   import create_graveyard,move_to_graveyard
import update.update_plant       
import update.update_species
from common.constants           import *

import sys
 

def run_action(action:str) -> None:
    match action:
        case 'add-species':
            species_information = create_species()
            add_species(species_information)
            exit(0)
        case 'add-plant':
            plant_information : dict[str,str] = create_plant()
            add_plant(plant_information)
            exit(0)
        case 'add-activities':
            log_items = create_multiple_activities()
            add_activities(log_items)
            exit(0)
        case 'add-growth':
            log_items = create_multiple_growth()
            add_growth(log_items)
            exit(0)
        case 'move-to-graveyard': 
            graveyard_info : dict[str,str] = create_graveyard()
            move_to_graveyard(graveyard_info)
            exit(0)
        case 'update-plant': 
            (plant_name,update_key,update_value) = update.update_plant.get_update_info()
            update.update_plant.update_plant(plant_name,update_key,update_value)
        case 'update-species':
            (species_name, update_key,update_value) = update.update_species.get_update_info()
            update.update_species.update_species(species_name,update_key,update_value)
            
            
if __name__ == '__main__':
    action : str = sys.argv[1]
    print(sys.argv)
    run_action(action)
