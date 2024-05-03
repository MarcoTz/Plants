import datetime 
from typing import TypedDict
from common.common import * 
from common.common import date_format,PlantInformation

class GrowthItem(TypedDict):
    log_date : datetime.datetime
    log_height_cm : float 
    log_width_cm : float

def show_GrowthItem(it:GrowthItem) -> str:
    date_str : str = it['log_date'].strftime(date_format)
    return '%s: height: %scm, width: %scm' % (date_str,it['log_height_cm'],it['log_width_cm'])


class Plant: 

    info : PlantInformation
    
    def __init__(self,json_dict:PlantInformation) -> None:
        self.info = json_dict

    def get_info_dict(self) -> dict[str,str]:
        info_dict = {
                'plant_name': self.info['plant_name'],
                'plant_species_name':self.info['species_name'],
                'plant_height':self.info['current_height'],
                'plant_width':self.info['current_height'],
                'plant_location': self.info['current_location'],
                'plant_origin': self.info['origin'],
                'plant_obtained' : self.info['obtained'].strftime(date_format),
                'plant_notes': '\n'.join(self.info['plant_notes'])
                }
        return info_dict

    def show(self) -> str: 
        out_str :str = '''
        Plant: %s, 
        Species: %s,
        Current dimensions: %scm x %scm
        Activities: %s 
        Growth: %s 
        Notes : %s'''

        info_tuple :tuple[str,str,str,str,str]= (
                self.info['plant_name'],
                self.info['species_name'],
                str(self.info['current_height']),
                str(self.info['current_width']),
                '\n'.join(self.info['plant_notes'])
                )

        return out_str % info_tuple
