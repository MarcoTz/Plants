from common.types        import * 
from common.constants    import date_format
from common.PlantSpecies import PlantSpecies

def find_species(species_name:str,species_list:list[PlantSpecies]) -> PlantSpecies | None:
    for species in species_list:
        if species_name.strip() == species.info['name']:
            return species
    return None

class Plant: 

    info           : PlantInformation
    current_height : float
    current_width  : float
    species        : PlantSpecies | None
    images         : list[tuple[datetime.datetime,str]]
    
    def __init__(self,json_dict:PlantInformation,species_list:list[PlantSpecies]) -> None:
        self.info : PlantInformation = json_dict
        self.current_height : float  = float('nan')
        self.current_width : float = float('nan') 
        self.images : list[tuple[datetime.datetime,str]] = []
        self.species : PlantSpecies | None = find_species(self.info['species_name'],species_list)
        

    def get_info_dict(self):
        info_dict = {
                'plant_name': self.info['plant_name'],
                'plant_species_name':self.info['species_name'],
                'plant_health':self.info['plant_health'],
                'plant_location': self.info['current_location'],
                'plant_height' : self.current_height,
                'plant_width' : self.current_width,
                'plant_origin': self.info['origin'],
                'plant_obtained' : self.info['obtained'].strftime(date_format),
                'plant_autowater':self.info['auto_water'],
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
        self.current_height = self.info['plant_growth'][0]['log_height_cm']
        self.current_width  = self.info['plant_growth'][0]['log_width_cm']

    def get_future_activity(self,activity_interval:int,activity_str:str) -> datetime.datetime | None:
        next_activity_delta : datetime.timedelta = datetime.timedelta(days=activity_interval)
        current_date        : datetime.datetime  = datetime.datetime.now()
        activity_filter_fun : function           = lambda x: x['log_activity'].strip == activity_str

        activities : list[LogItem] = list(filter(activity_filter_fun,self.info['plant_activities']))
        activities.sort(key=lambda x:x['log_date'])

        if len(activities) == 0: 
            return None

        last_activity_date : datetime.datetime = activities[-1]['log_date']
        next_activity_date : datetime.datetime = last_activity_date + next_activity_delta 

        if next_activity_date <= current_date:
            next_activity_date = current_date

        return next_activity_date

    def get_next_watering(self) -> datetime.datetime | None:
        if self.species is None:
            return None 
        if self.info['auto_water']:
            return None

        watering_interval : int = self.species.info['avg_watering_days']
        if watering_interval == -1:
             return None 
        activity_str : str = 'Watering'
        return self.get_future_activity(watering_interval,activity_str)

    def get_next_fertilizing(self) -> datetime.datetime | None:
        if self.species is None:
             return None
        
        fertilizing_interval : int = self.species.info['avg_fertilizing_days']
        if fertilizing_interval == -1:
            return None 
        activity_str : str = 'Fertilizing'
        return self.get_future_activity(fertilizing_interval,activity_str)



    def get_next_dates(self) -> tuple[datetime.datetime | None,datetime.datetime | None]:
        next_watering : datetime.datetime | None = self.get_next_watering()
        next_fertilizing : datetime.datetime | None = self.get_next_fertilizing()
        return (next_watering, next_fertilizing)
