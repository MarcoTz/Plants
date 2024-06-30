from file_io.load_csv       import load_activities,load_growth,load_graveyard
from common.HTMLRenderer    import HTMLRenderer
from common.types           import * 
from common.constants       import * 

from common.PlantManager import PlantManager

if __name__ == '__main__':
    manager : PlantManager = PlantManager()
    renderer : HTMLRenderer = HTMLRenderer(manager)
    renderer.render_all()
