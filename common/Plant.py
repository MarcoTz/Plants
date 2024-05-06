from common.common import * 
from common.common import date_format,PlantInformation

class Plant: 

    info           : PlantInformation
    current_height : float
    current_width  : float
    images         : list[tuple[datetime.datetime,str]]
    
    def __init__(self,json_dict:PlantInformation) -> None:
        self.info = json_dict
        self.current_height = float('nan')
        self.current_width = float('nan') 
        self.images = []

    def get_info_dict(self) -> dict[str,str]:
        info_dict = {
                'plant_name': self.info['plant_name'],
                'plant_species_name':self.info['species_name'],
                'plant_location': self.info['current_location'],
                'plant_height' : self.current_height,
                'plant_width' : self.current_width,
                'plant_origin': self.info['origin'],
                'plant_obtained' : self.info['obtained'].strftime(date_format),
                'plant_notes': '\n'.join(self.info['plant_notes']),
                }
        return info_dict

    def add_activities(self,new_logs:list[LogItem]) -> None:
        self.info['plant_activities'].extend(new_logs)
        self.info['plant_activities'].sort(key=lambda x: x['log_date'],reverse=True)

    def add_growth(self,new_growth:list[GrowthItem])->None:
        self.info['plant_growth'].extend(new_growth)
        self.info['plant_growth'].sort(key=lambda x:x['log_date'],reverse=True)
        self.update_size()

    def add_images(self,images:list[tuple[datetime.datetime,str]]) -> None:
        self.images.extend(images)
        self.images.sort(key=lambda x:x[0],reverse=True)

    def update_size(self) -> None:
        self.current_height = self.info['plant_growth'][-1]['log_height_cm']
        self.current_width  = self.info['plant_growth'][-1]['log_width_cm']

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
                str(self.current_height),
                str(self.current_width),
                '\n'.join(self.info['plant_notes'])
                )

        return out_str % info_tuple
