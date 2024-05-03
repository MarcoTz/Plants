from typing import TypedDict 
import datetime 

date_time_format = '%d.%m.%Y %H:%M:%S'
date_format = '%d.%m.%Y'

class SpeciesInformation(TypedDict):
  name : str
  scientific_name : str
  sunlight_requirements: str 
  temperature_min : float
  temperature_max : float
  optimal_temperature_min : float
  optimal_temperature_max : float
  plant_distance_cm : int
  ph_min:float
  ph_max:float
  watering_notes:list[str]
  fertilizing_notes:list[str]
  pruning_notes:list[str]
  companions:list[str]
  additional_notes:list[str]


class PlantInformation(TypedDict):
    plant_name       : str
    species_name     : str
    current_height   : float
    current_width    : float
    current_location : str
    origin           : str
    obtained         : datetime.datetime
    plant_notes      : list[str]
