from common.load_json    import load_plants_species 
from common.load_csv     import load_activities
from common.HTMLRenderer import HTMLRenderer
from common.common import * 

(plants,species) = load_plants_species()
activities = load_activities()
renderer = HTMLRenderer(plants,species,activities)

renderer.render_all()
