import load_json 
import classes.HTMLRenderer

(plants,species) = load_json.load_plants_species()
renderer = classes.HTMLRenderer.HTMLRenderer()

renderer.render_all(species,plants)
