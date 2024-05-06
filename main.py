from common.load_json    import load_plants_species 
from common.load_csv     import load_activities,load_growth
from common.HTMLRenderer import HTMLRenderer
from common.Plant        import Plant
from common.common import * 

import os 
from PIL import Image

def assign_activities(plants : list[Plant], activities: dict[str,list[LogItem]]) -> None:
    for plant in plants:
        plant_name  = plant.info['plant_name']
        plant_logs = activities[plant_name] if plant_name in activities else []
        if plant_name not in activities:
            print('No activities for plant %s' % plant_name)
        plant.add_activities(plant_logs)
    plant_names : list[str] = list(map(lambda x:x.info['plant_name'],plants))
    unmatched = { plant:activities[plant] for plant in activities.keys() if plant not in plant_names }
    if not list(unmatched.keys()) == []: 
        print('Could not assign all activities: ')
        print(unmatched)

def assign_growth(plants:list[Plant], growth: dict[str,list[GrowthItem]]) -> None:
    for plant in plants:
        plant_name = plant.info['plant_name']
        plant_growth = growth[plant_name] if plant_name in growth else []
        if plant_name not in growth: 
            print('No growth for plant %s' % plant_name)
        plant.add_growth(plant_growth)
    plant_names : list[str] = list(map(lambda x:x.info['plant_name'],plants))
    unmatched = { plant:growth[plant] for plant in growth.keys() if plant not in plant_names }
    if not list(unmatched.keys()) == []:
        print('Could not assign all growth logs: ')
        print(unmatched)

def load_images(plants:list[Plant]) -> None:
    images_dir = os.path.join(img_dir,img_plants_dir)
    images_names = os.listdir(images_dir)
    found_names = []
    for plant in plants:
        plant_name : str = plant.info['plant_name'].replace(' ','')
        plant_image_names : list[str] = list(filter(lambda x: plant_name in x,images_names))
        date_fun = lambda x: datetime.datetime.strptime(x[x.rfind('_')+1:].replace('.jpg',''),date_format_images)
        plant_images : list[tuple[datetime.datetime,str]] = [(date_fun(image_name),image_name) for image_name in plant_image_names]
        plant.add_images(plant_images)
        found_names.extend(plant_image_names)
    unmatched = [image_name for image_name in images_names if image_name not in found_names]
    if not unmatched == []:
        print('Could not match images to plants: ') 
        print(unmatched) 

def create_preview_images() -> None:
    plant_images_dir : str = os.path.join(img_dir,img_plants_dir)
    species_images_dir : str = os.path.join(img_dir, img_species_dir)
    preview_size = 128,128
    plant_images : list[str] = os.listdir(plant_images_dir)
    plant_images = list(map(lambda x: os.path.join(plant_images_dir,x),plant_images))
    species_images : list[str] = os.listdir(species_images_dir)
    species_images = list(map(lambda x: os.path.join(species_images_dir,x),species_images))
    all_images : list[str] = plant_images
    all_images.extend(species_images)
    for image_path in all_images:
        if os.path.isdir(image_path):
            continue
        image_name = os.path.basename(image_path)
        plant_image_preview_path = image_path.replace(image_name,os.path.join(img_small_dir,image_name))
        if os.path.exists(plant_image_preview_path):
            continue
        plant_image = Image.open(image_path)
        plant_image.thumbnail(preview_size,Image.Resampling.LANCZOS)
        plant_image.save(plant_image_preview_path,'JPEG')


(plants,species) = load_plants_species()
activities = load_activities()
assign_activities(plants,activities)
growth = load_growth()
assign_growth(plants,growth)
load_images(plants)
create_preview_images()
renderer = HTMLRenderer(plants,species)
renderer.render_all()
