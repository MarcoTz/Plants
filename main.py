import load_json 
import classes.HTMLRenderer

plants = load_json.load_all_plants() 
renderer = classes.HTMLRenderer.HTMLRenderer()

renderer.render_all_species(plants)
