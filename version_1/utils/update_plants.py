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
    
    autowatered : bool = False 
    is_autowatered : str = input('Is plant automatically watered? (y/n)')  
    if is_autowatered.lower().strip() == 'y':
        autowatered = True

    species_dict['auto_watering'] = autowatered 
    
    species_file = open(species_file_path,'w')
    species_file.write(json.dumps(species_dict))
    species_file.close()

