from typing import TypedDict
import datetime
 
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

class GraveyardPlant(TypedDict):
    graveyard_plant     : str
    graveyard_species   : str
    graveyard_planted   : datetime.datetime
    graveyard_died      : datetime.datetime
    graveyard_reason    : str
