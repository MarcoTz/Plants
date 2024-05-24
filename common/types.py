from common.constants import date_format
from typing import TypedDict
import datetime
 
class SpeciesInformation(TypedDict):
  name : str
  scientific_name           : str
  species_type              : str
  sunlight_requirements     : str 
  temperature_min           : float
  temperature_max           : float
  optimal_temperature_min   : float
  optimal_temperature_max   : float
  plant_distance_cm         : int
  ph_min                    : float
  ph_max                    : float
  watering_notes            : list[str]
  avg_watering_days         : int
  fertilizing_notes         : list[str]
  avg_fertilizing_days      : int
  pruning_notes             : list[str]
  companions                : list[str]
  additional_notes          : list[str]

def coalesce_species(json_dict:dict[str,str | list[str]]) -> SpeciesInformation:
    species_info : SpeciesInformation = {
            'name'                      : str(json_dict['name']),
            'scientific_name'           : str(json_dict['scientific_name']),
            'species_type'              : str(json_dict['species_type']),
            'sunlight_requirements'     : str(json_dict['sunlight_requirements']),
            'temperature_min'           : float(str(json_dict['temperature_min'])),
            'temperature_max'           : float(str(json_dict['temperature_max'])),
            'optimal_temperature_min'   : float(str(json_dict['optimal_temperature_min'])),
            'optimal_temperature_max'   : float(str(json_dict['optimal_temperature_max'])),
            'plant_distance_cm'         : int(float(str(json_dict['plant_distance_cm']))),
            'ph_min'                    : float(str(json_dict['ph_min'])),
            'ph_max'                    : float(str(json_dict['ph_max'])),
            'watering_notes'            : list(json_dict['watering_notes']),
            'avg_watering_days'         : int(float(str(json_dict['avg_watering_days']))),
            'fertilizing_notes'         : list(json_dict['fertilizing_notes']),
            'avg_fertilizing_days'      : int(float(str(json_dict['avg_fertilizing_days']))),
            'pruning_notes'             : list(json_dict['pruning_notes']),
            'companions'                : list(json_dict['companions']),
            'additional_notes'          : list(json_dict['additional_notes'])
    }
    return species_info 


class LogItem(TypedDict):
    log_activity : str
    log_date     : datetime.datetime
    log_note     : str

class GrowthItem(TypedDict):
    log_date        : datetime.datetime
    log_height_cm   : float 
    log_width_cm    : float
    log_note        : str

class PlantInformation(TypedDict):
    plant_name       : str
    species_name     : str
    current_location : str
    origin           : str
    obtained         : datetime.datetime
    plant_notes      : list[str]
    plant_activities : list[LogItem]
    plant_growth     : list[GrowthItem]

def coalesce_plant(json_dict:dict[str,str | list[str]]) -> PlantInformation:

    plant_info : PlantInformation = {
    'plant_name'       : str(json_dict['plant_name']),
    'species_name'     : str(json_dict['species_name']),
    'current_location' : str(json_dict['current_location']),
    'origin'           : str(json_dict['origin']),
    'obtained'         : datetime.datetime.strptime(str(json_dict['obtained']),date_format),
    'plant_notes'      : list(json_dict['plant_notes']),
    'plant_activities' : [],
    'plant_growth'     : []
    } 
    return plant_info



class GraveyardPlant(TypedDict):
    graveyard_plant     : str
    graveyard_species   : str
    graveyard_planted   : datetime.datetime
    graveyard_died      : datetime.datetime
    graveyard_reason    : str
