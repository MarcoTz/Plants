from common.load_json    import load_plants_species 
from common.load_csv     import load_activities,load_growth
from common.HTMLRenderer import HTMLRenderer
from common.common import * 

(plants,species) = load_plants_species()
activities = load_activities()
growth = load_growth()
renderer = HTMLRenderer(plants,species,activities,growth)
renderer.render_all()
