import os 
import json


species_dir= 'Plants'
for species_file_name in os.listdir(species_dir):
    species_file_path = os.path.join(species_dir,species_file_name)
    if os.path.isdir(species_file_path):
        continue
    species_file = open(species_file_path,'r')
    species_json = species_file.read()
    species_file.close()
    species_dict = json.loads(species_json)
    print('%s (%s) ' % (species_dict['plant_name'],species_dict['species_name']))
    
    plant_health_nr : int = -1 
    while plant_health_nr == -1:
        plant_health : str = input('Enter Plant health (0-5)')  
        try: 
            i : int = int(plant_health)
            if -1 < i and i < 6:
                plant_health_nr = i
        except: 
            pass

    species_dict['plant_health'] = plant_health_nr 
    
    species_file = open(species_file_path,'w')
    species_file.write(json.dumps(species_dict))
    species_file.close()

