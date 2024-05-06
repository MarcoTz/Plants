from common.load_json    import load_plants_species 
from common.load_csv     import load_activities,load_growth
from common.HTMLRenderer import HTMLRenderer
from common.Plant        import Plant
from common.common import * 

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

(plants,species) = load_plants_species()
activities = load_activities()
assign_activities(plants,activities)
growth = load_growth()
assign_growth(plants,growth)
renderer = HTMLRenderer(plants,species)
renderer.render_all()
