from typing import TypedDict
from common.common import SpeciesInformation

class PlantSpecies:

    info : SpeciesInformation

    def __init__(self,json_dict:SpeciesInformation) -> None:
        self.info = json_dict

    def get_info_dict(self) -> dict[str,str]:
       info_dict = { 
         'species_name':self.info['name'],
         'species_scientific_name':self.info['scientific_name'],
         'species_sunlight':self.info['sunlight_requirements'],
         'species_temp_min':self.info['temperature_min'],
         'species_temp_max':self.info['temperature_max'],
         'species_opt_temp_min':self.info['optimal_temperature_min'],
         'species_opt_temp_max':self.info['optimal_temperature_max'],
         'species_dist':self.info['plant_distance_cm'],
         'species_ph_min':self.info['ph_min'],
         'species_ph_max':self.info['ph_max'],
         'species_watering_notes':', '.join(self.info['watering_notes']),
         'species_fertilizing_notes':', '.join(self.info['fertilizing_notes']),
         'species_pruning_notes':', '.join(self.info['pruning_notes']),
         'species_companions':', '.join(self.info['companions']),
         'species_additional_notes':', '.join(self.info['additional_notes']) 
        }
       return info_dict
                                
    def show(self) -> str:
        out_str = '''Plant %s
          Scientific Name:%s
          Sunlight Requirements: %s
          Temperature Range: %s-%s C
          Optimal Temperature Range: %s-%s C
          Distance to nearest plant: %s cm
          pH Range: %s-%s
          Watering: 
            %s 
          Fertilizing: 
            %s
          Pruning: 
            %s
          Companion Plants:
            %s 
          Notes:
            %s
          '''
        sunlight_str = self.info['sunlight_requirements']
        contents_tuple = (
                self.info['name'], 
                self.info['scientific_name'],
                sunlight_str,
                str(self.info['temperature_min']),
                str(self.info['temperature_max']),
                str(self.info['optimal_temperature_min']),
                str(self.info['optimal_temperature_max']),
                str(self.info['plant_distance_cm']),
                str(self.info['ph_min']),
                str(self.info['ph_max']),
                '\n  '.join(self.info['watering_notes']),
                '\n  '.join(self.info['fertilizing_notes']),
                '\n  '.join(self.info['pruning_notes']),
                ', '.join(self.info['companions']),
                '\n  '.join(self.info['additional_notes']),
                )
        return out_str % contents_tuple
