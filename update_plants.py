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
    print(species_dict['watering_notes'])
    watering_days = int(input('Enter watering interval: '))
    print(species_dict['fertilizing_notes'])
    fertilizing_days = int(input('Enter fertilizing interval: '))
    species_dict['avg_watering_days'] = watering_days
    species_dict['avg_fertilizing_days'] = fertilizing_days
    
    if 'avg_watering_summer_days' in species_dict:
        del species_dict['avg_watering_summer_days']
    if 'avg_watering_winter_days' in species_dict:
        del species_dict['avg_watering_winter_days']
    if 'avg_fertilizing_summer_days' in species_dict:
        del species_dict['avg_fertilizing_summer_days']
    if 'avg_fertilizing_winter_days' in species_dict:
        del species_dict['avg_fertilizing_winter_days']

    species_file = open(species_file_path,'w')
    species_file.write(json.dumps(species_dict))
    species_file.close()

