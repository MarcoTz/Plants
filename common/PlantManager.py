from file_io.load_json      import load_species, load_plants
from file_io.load_csv       import load_activities,load_growth,load_graveyard
from common.Plant           import Plant
from common.PlantSpecies    import PlantSpecies
from common.types           import * 
from common.constants       import * 

import os 
from PIL import Image

class PlantManager: 

    plants    : list[Plant]
    species   : list[PlantSpecies]
    graveyard : list[GraveyardPlant]

    def __init__(self) -> None:
        self.species : list[PlantSpecies] = load_species()
        self.species.sort(key=lambda x: x.info['name'])

        self.graveyard : list[GraveyardPlant] = load_graveyard()

        growth_items : dict[str,list[GrowthItem]] = load_growth()
        activities   : dict[str,list[LogItem]]    = load_activities()
        plants_info  : list[PlantInformation]     = load_plants()
        self.plants  : list[Plant]                = []
        self.load_plants(plants_info,growth_items,activities)
        self.plants.sort(key = lambda x: x.info['plant_name'])

        self.load_plant_images()
        self.create_preview_images()

    def load_plants(self,
                    plants_info:list[PlantInformation],
                    growth_items:dict[str,list[GrowthItem]],
                    activities:dict[str,list[LogItem]]) -> None:

        for plant_info in plants_info:
            plant_name : str = plant_info['plant_name']

            plant_species : PlantSpecies | None = None
            for species in self.species:
                if species.info['name'] == plant_info['species_name']:
                    plant_species = species
                    break

            plant_activities : list[LogItem]
            if plant_name in activities:
                plant_activities : list[LogItem] = activities[plant_name]
                activities.pop(plant_name)
            else:
                plant_activities : list[LogItem] = [] 
                print('No activities for %s' % plant_name)

            plant_growth : list[GrowthItem]
            if plant_name in growth_items:
                plant_growth : list[GrowthItem] = growth_items[plant_name]
                growth_items.pop(plant_name)
            else: 
                plant_growth : list[GrowthItem] = []
                print('No growth for %s' % plant_name)

            new_plant : Plant = Plant(
                    plant_info,
                    plant_species,
                    plant_activities,
                    plant_growth)

            self.plants.append(new_plant)

        if list(growth_items.keys()) != []:
            print('Could not assign all growth items %s' % str(growth_items.items()))

        if list(activities.keys()) != []:
            print('Could not assign all activities %s' % str(activities.items()))

    def load_plant_images(self) -> None:
        images_dir = os.path.join(out_dir,img_dir,img_plants_dir)
        images_names : list[str] = os.listdir(images_dir)
        images_names = list(filter(lambda x:not os.path.isdir(os.path.join(images_dir,x)),images_names))
        found_names = []
        for plant in self.plants:
            plant_name : str = plant.info['plant_name'].replace(' ','')
            plant_image_names : list[str] = list(filter(lambda x: plant_name in x,images_names))
            date_fun = lambda x: datetime.datetime.strptime(x[x.rfind('_')+1:].replace('.jpg',''),date_format_images)
            plant_images : list[tuple[datetime.datetime,str]] = []
            try:
                plant_images  = [(date_fun(image_name),image_name) for image_name in plant_image_names]
            except ValueError:
                print('Could not load images')
                print(plant_image_names)
                return
            if len(plant_images)==0:
                print('No images for plant %s' % plant.info['plant_name'])
            else:
                plant.add_images(plant_images)
            found_names.extend(plant_image_names)
        unmatched = [image_name for image_name in images_names if image_name not in found_names]
        if not unmatched == []:
            print('Could not match images to plants: ') 
            print(unmatched) 

    def create_preview_images(self) -> None:
        plant_images_dir : str = os.path.join(out_dir,img_dir,img_plants_dir)
        preview_size     : tuple[int,int]  = 128,128
        plant_images     : list[str] = os.listdir(plant_images_dir)
        dir_fun          : function = lambda x: os.path.join(plant_images_dir,x)
        plant_images     : list[str]  = list(map(dir_fun,plant_images))
        for image_path in plant_images:
            if os.path.isdir(image_path):
                continue
            image_name      : str = os.path.basename(image_path)
            img_small_path  : str = os.path.join(img_small_dir,image_name)
            plant_image_preview_path = image_path.replace(image_name,img_small_path)
            if os.path.exists(plant_image_preview_path):
                continue
            plant_image = Image.open(image_path)
            plant_image.thumbnail(preview_size,Image.Resampling.LANCZOS)
            plant_image.save(plant_image_preview_path,'JPEG')
    
    def get_plants_species(self,species:str) -> list[Plant]: 
        species_li : list[Plant] = []
        for plant in self.plants:
            if plant.info['species_name'] == species:
                species_li.append(plant)
        return species_li

    
    def get_plant_locations(self) -> list[str]:
        locations : list[str] = []
        for plant in self.plants:
            plant_location = plant.info['current_location']
            if plant_location not in locations:
                locations.append(plant_location)
        return locations 


    def get_old_growth(self) -> list[Plant]:
        plant_date_list : list[Plant] = [] 
        two_weeks : datetime.datetime = datetime.datetime.now() - datetime.timedelta(weeks=2)

        for plant in self.plants:
            # skip dormant plants
            if plant.info['plant_health'] == 0: 
                continue
            plant_growth : list[GrowthItem] = plant.growth
            plant_growth.sort(key=lambda x: x['log_date'])
            last_growth_update : GrowthItem = plant_growth[-1]
            if last_growth_update['log_date'] < two_weeks: 
                plant_date_list.append(plant)

        return plant_date_list 

    def get_next_activity_dates(self) -> list[tuple[Plant,str,datetime.datetime]]:
        activities_list : list[tuple[Plant,str,datetime.datetime]] = []
        current_date  : datetime.datetime = datetime.datetime.now()
        next_week     : datetime.datetime = current_date + datetime.timedelta(weeks=1)

        for plant in self.plants:
            watering_date : datetime.datetime | None = plant.next_watering
            if watering_date is not None and watering_date > next_week:
                watering_date : datetime.datetime | None = None 

            fertilizing_date : datetime.datetime | None = plant.next_fertilizing
            if fertilizing_date is not None and fertilizing_date > next_week:
                fertilizing_date : datetime.datetime | None = None 

            if watering_date is None and fertilizing_date is None:
                continue 

            if watering_date is None and fertilizing_date is not None:
                activity_tuple : tuple[Plant,str,datetime.datetime] = (plant,'Fertilizing',fertilizing_date)
                activities_list.append(activity_tuple)
                continue
            if fertilizing_date is None and watering_date is not None:
                activity_tuple : tuple[Plant,str,datetime.datetime] = (plant,'Watering',watering_date)
                activities_list.append(activity_tuple)
                continue 

            if watering_date is not None and fertilizing_date is not None:
                if (watering_date.day == fertilizing_date.day 
                    and watering_date.month == fertilizing_date.month 
                    and watering_date.year == fertilizing_date.year):
                    activity_tuple : tuple[Plant,str,datetime.datetime] = (plant,'Watering + Fertilizing',watering_date)
                    activities_list.append(activity_tuple)
                    continue 

                watering_tuple    : tuple[Plant,str,datetime.datetime] = (plant,'Watering',watering_date)
                fertilizing_tuple : tuple[Plant,str,datetime.datetime] = (plant,'Fertilizing',fertilizing_date)
                activities_list.append(watering_tuple)
                activities_list.append(fertilizing_tuple)
            

        return activities_list
