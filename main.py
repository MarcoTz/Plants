from common.load_json import load_plants_species 
from common.HTMLRenderer import HTMLRenderer

(plants,species) = load_plants_species()
renderer = HTMLRenderer()

renderer.render_all(species,plants)
