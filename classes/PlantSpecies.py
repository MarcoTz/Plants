import enum

class SunlightType(enum.Enum):
    DIRECT=None
    INDIRECT=None
    SHADE=None

def strToSunlightType(ty:str) -> SunlightType:
    match ty.lower(): 
        case 'direct':   return SunlightType.DIRECT
        case 'indirect': return SunlightType.INDIRECT
        case 'shade':    return SunlightType.SHADE
        case _ :         raise ValueError

def showSunlightType(sn:SunlightType) -> str:
    match sn:
      case SunlightType.DIRECT:   return 'direct'
      case SunlightType.INDIRECT: return 'indirect'
      case SunlightType.SHADE:    return 'shade'
      case _:                     raise ValueError


class PlantSpecies:

    species_name            :str
    scientific_name         :str
    sunlight_requirements   :SunlightType
    temperature_min         :float
    temperature_max         :float
    optimal_temperature_min :float
    optimal_temperature_max :float
    plant_distance_cm       :int
    ph_min                  :float 
    ph_max                  :float
    watering_notes          :list[str]
    fertilizing_notes       :list[str]
    pruning_notes           :list[str]
    companions              :list[str]
    additional_notes        :list[str]

    def __init__(self,json_dict:dict[str,str]) -> None:
        self.species_name            = json_dict['name']
        self.scientific_name         = json_dict['scientific_name'] 
        self.sunlight_requirements   = strToSunlightType(json_dict['sunlight_requirements'])
        self.temperature_min         = float(json_dict['temperature_min'])
        self.temperature_max         = float(json_dict['temperature_max'])
        self.optimal_temperature_min = float(json_dict['optimal_temperature_min'])
        self.optimal_temperature_max = float(json_dict['optimal_temperature_max'])
        self.plant_distance_cm       = int(json_dict['plant_distance_cm'])
        self.ph_min                  = float(json_dict['ph_min'])
        self.ph_max                  = float(json_dict['ph_max'])
        self.watering_notes          = list(json_dict['watering_notes'])
        self.fertilizing_notes       = list(json_dict['fertilizing_notes'])
        self.pruning_notes           = list(json_dict['pruning_notes'])
        self.companions              = list(json_dict['companions'])
        self.additional_notes        = list(json_dict['additional_notes'])
                                
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
        sunlight_str = showSunlightType(self.sunlight_requirements)
        contents_tuple = (
                self.species_name, 
                self.scientific_name,
                sunlight_str,
                str(self.temperature_min),
                str(self.temperature_max),
                str(self.optimal_temperature_min),
                str(self.optimal_temperature_max),
                str(self.plant_distance_cm),
                str(self.ph_min),
                str(self.ph_max),
                '\n  '.join(self.watering_notes),
                '\n  '.join(self.fertilizing_notes),
                '\n  '.join(self.pruning_notes),
                ', '.join(self.companions),
                '\n  '.join(self.additional_notes),
                )
        return out_str % contents_tuple
