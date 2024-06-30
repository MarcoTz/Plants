from common.types        import * 
from common.constants    import date_format
from common.PlantSpecies import PlantSpecies

def find_species(species_name:str,species_list:list[PlantSpecies]) -> PlantSpecies | None:
    for species in species_list:
        if species_name.strip() == species.info['name']:
            return species
    return None

class Plant: 

    info             : PlantInformation
    current_height   : float
    current_width    : float
    species          : PlantSpecies | None
    images           : list[tuple[datetime.datetime,str]]
    activities       : list[LogItem]
    growth           : list[GrowthItem]
    next_watering    : datetime.datetime | None
    next_fertilizing : datetime.datetime | None
    
    def __init__(self,
                 json_dict:PlantInformation,
                 species:PlantSpecies | None,
                 activities : list[LogItem],
                 growth : list[GrowthItem]) -> None:

        self.info           : PlantInformation = json_dict
        self.current_height : float  = float('nan')
        self.current_width  : float = float('nan') 
        self.images         : list[tuple[datetime.datetime,str]] = []
        self.species        : PlantSpecies | None = species
        self.activities     : list[LogItem] = activities
        self.update_next_watering()
        self.update_next_fertilizing()
        self.growth         : list[GrowthItem] = growth
        
    def get_info_dict(self):
        next_watering_date_str    : str = 'N/A'
        next_fertilizing_date_str : str = 'N/A'
        last_watering_str    : str = 'N/A'
        last_fertilizing_str : str = 'N/A'
        
        if self.next_watering is not None:
            next_watering_date_str      : str = self.next_watering.strftime(date_format)
        if self.next_fertilizing is not None:
            next_fertilizing_date_str   : str = self.next_fertilizing.strftime(date_format)

        filter_fun : function = lambda y: lambda x: x['log_activity'] == y 

        water_filter        : function = filter_fun('Watering')
        watering_activities : list[LogItem] = list(filter(water_filter,self.activities))
        if len(watering_activities) != 0: 
            last_watering_str : str = watering_activities[0]['log_date'].strftime(date_format)
        else:
            print('Never watered plant %s' % self.info['plant_name'])

        fertilizing_filter      : function = filter_fun('Fertilizing')
        fertilizing_activities  : list[LogItem] = list(filter(fertilizing_filter,self.activities))
        if len(fertilizing_activities) != 0:
            last_fertilizing_str : str = fertilizing_activities[0]['log_date'].strftime(date_format)
        else:
            print('Never fertilized plant %s' % self.info['plant_name'])

        info_dict = {
                'plant_name'            : self.info['plant_name'],
                'plant_species_name'    : self.info['species_name'],
                'plant_health'          : self.info['plant_health'],
                'plant_location'        : self.info['current_location'],
                'plant_height'          : self.current_height,
                'plant_width'           : self.current_width,
                'plant_origin'          : self.info['origin'],
                'plant_obtained'        : self.info['obtained'].strftime(date_format),
                'plant_autowater'       : self.info['auto_water'],
                'plant_notes'           : '\n'.join(self.info['plant_notes']),
                'next_watering_date'    : next_watering_date_str,
                'next_fertilizing_date' : next_fertilizing_date_str,
                'last_watering_date'    : last_watering_str,
                'last_fertilizing_date' : last_fertilizing_str
                }
        return info_dict

    def update_next_watering(self) -> None:
        if self.species is None:
            self.next_watering : datetime.datetime | None =  None 
            return 
        if self.info['auto_water'] or self.info['plant_health'] == 0:
            self.next_watering : datetime.datetime | None = None
            return

        watering_interval : int = self.species.info['avg_watering_days']
        if watering_interval == -1:
            self.next_watering : datetime.datetime | None = None
            return
        activity_str : str = 'Watering'
        self.next_watering : datetime.datetime | None = self.get_future_activity(watering_interval,activity_str)

    def update_next_fertilizing(self) -> None:
        if self.species is None:
            self.next_fertilizing : datetime.datetime | None = None
            return
        
        fertilizing_interval : int = self.species.info['avg_fertilizing_days']
        if fertilizing_interval == -1:
            self.next_fertilizing : datetime.datetime | None = None
            return
        activity_str : str = 'Fertilizing'
        self.next_fertilizing : datetime.datetime | None = self.get_future_activity(fertilizing_interval,activity_str)


    def update_next_dates(self) -> None:
        self.update_next_watering()
        self.update_next_fertilizing()

    def add_activity(self,new_log:LogItem) -> None:
        self.activities.append(new_log)
        self.activities.sort(key=lambda x: x['log_date'],reverse=True)
        self.update_next_dates()

    def add_growth(self,new_growth:GrowthItem)->None:
        self.growth.append(new_growth)
        self.growth.sort(key=lambda x:x['log_date'],reverse=True)
        self.update_size()

    def add_images(self,images:list[tuple[datetime.datetime,str]]) -> None:
        self.images.extend(images)
        self.images.sort(key=lambda x:x[0],reverse=True)

    def update_size(self) -> None:
        self.current_height : float = self.growth[0]['log_height_cm']
        self.current_width  : float = self.growth[0]['log_width_cm']

    def get_future_activity(self,activity_interval:int,activity_str:str) -> datetime.datetime | None:
        next_activity_delta : datetime.timedelta = datetime.timedelta(days=activity_interval)
        current_date        : datetime.datetime  = datetime.datetime.now()
        activity_filter_fun : function           = lambda x: x['log_activity'].strip() == activity_str
        
        activities : list[LogItem] = self.activities.copy()
        activities : list[LogItem] = list(filter(activity_filter_fun,activities))
        activities.sort(key=lambda x:x['log_date'])

        if len(activities) == 0: 
            return None

        last_activity_date : datetime.datetime = activities[-1]['log_date']
        next_activity_date : datetime.datetime = last_activity_date + next_activity_delta 

        if next_activity_date <= current_date:
            next_activity_date = current_date

        return next_activity_date
