import os 
import json

species_dir= 'PlantSpecies'
for species_file_name in os.listdir(species_dir):
    species_file_path = os.path.join(species_dir,species_file_name)
    if os.path.isdir(species_file_path):
        continue
    species_file = open(species_file_path,'r')
    species_json = species_file.read()
    species_file.close()
    species_dict = json.loads(species_json)
    print(species_dict['name'])

    plant_type : str = input('Please enter plant type (cactus/nightshade/etc)').strip()

    species_dict['species_type'] = plant_type
    

    species_file = open(species_file_path,'w')
    species_file.write(json.dumps(species_dict))
    species_file.close()

