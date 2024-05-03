from typing import TypedDict 
import datetime 

date_time_format = '%d.%m.%Y %H:%M:%S'
date_format = '%d.%m.%Y'

template_dir                   : str = 'html_templates'
species_overview_template_name : str = 'species_overview.html'
species_details_template_name  : str = 'species_details.html'
plant_overview_template_name   : str = 'plant_overview.html'
plant_details_template_name    : str = 'plant_details.html'
index_template_name            : str = 'index.html'
activity_log_template_name     : str = 'activity_logs.html'

out_dir               : str = 'html_out'
species_details_out   : str = 'plant_species'
plant_details_out     : str = 'plants'
species_overview_out  : str = 'species_overview.html'
plant_overview_out    : str = 'plant_overview.html'
index_out             : str = 'index.html'
activity_log_out      : str = 'activity_logs.html'

species_dir             : str = 'PlantSpecies'
plants_dir              : str = 'Plants'
log_dir                 : str = 'Logs'
activity_log_file_name  : str = 'Activities.csv'
growth_log_file_name    : str = 'Growth.csv'

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

class LogItem(TypedDict):
    log_activity : str
    log_date     : datetime.datetime
    log_note     : str
    log_plant    : str


class GrowthItem(TypedDict):
    log_date        : datetime.datetime
    log_plant       : str
    log_height_cm   : float 
    log_width_cm    : float
    log_note        : str
