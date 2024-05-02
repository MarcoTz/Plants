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

    species_name:str
    scientific_name:str
    sunlight_requirements:SunlightType

    def __init__(self,json_dict:dict[str,str]) -> None:
        self.species_name = json_dict['name']
        self.scientific_name = json_dict['scientific_name'] 
        self.sunlight_requirements = strToSunlightType(json_dict['sunlight_requirements'])

    def show(self) -> str:
        out_str = '''Plant %s
          Scientific Name:%s
          Sunlight Requirements: %s
          '''
        sunlight_str = showSunlightType(self.sunlight_requirements)
        return out_str % (self.species_name, self.scientific_name,sunlight_str)
