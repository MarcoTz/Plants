import jinja2
import os
from classes.PlantSpecies import PlantSpecies
from classes.Plant import Plant

template_dir                   : str = 'html_templates'
species_overview_template_name : str = 'species_overview.html'
species_details_template_name  : str = 'species_details.html'
plant_overview_template_name   : str = 'plant_overview.html'
plant_details_template_name    : str = 'plant_details.html'
index_template_name            : str = 'index.html'

out_dir               : str = 'html_out'
species_details_out   : str = 'plant_species'
plant_details_out     : str = 'plants'
species_overview_out  : str = 'species_overview.html'
plant_overview_out    : str = 'plant_overview.html'
index_out             : str = 'index.html'

def create_if_not_exists(dir_name:str)->None:
    if not os.path.exists(dir_name):
        os.makedirs(dir_name) 
def write_html(out_file_name:str,html_contents:str) -> None:
    out_file_path = os.path.join(out_dir,out_file_name)
    out_file = open(out_file_path,'w+')
    out_file.write(html_contents)
    out_file.close()

def get_html_name(plant_name:str) -> str:
    return plant_name.replace(' ','_')+'.html'

class HTMLRenderer: 

    env                       : jinja2.Environment
    species_overview_template : jinja2.Template
    species_details_template  : jinja2.Template
    plant_overview_template   : jinja2.Template
    plant_details_template    : jinja2.Template

    def __init__(self) -> None:
        self.env = jinja2.Environment(loader=jinja2.FileSystemLoader(template_dir),autoescape=False)
        create_if_not_exists(out_dir)
        create_if_not_exists(os.path.join(out_dir,species_details_out))
        create_if_not_exists(os.path.join(out_dir,plant_details_out))
        self.load_templates()
    
    def load_templates(self) -> None:
        self.species_overview_template = self.env.get_template(species_overview_template_name)
        self.species_details_template  = self.env.get_template(species_details_template_name)
        self.plant_overview_template   = self.env.get_template(plant_overview_template_name)
        self.plant_details_template    = self.env.get_template(plant_details_template_name)
        self.index_template            = self.env.get_template(index_template_name)

    def create_species_li(self,plant:PlantSpecies) -> str:
        li_template       : str = '<li><a href="%s/%s">%s</a></li>'
        details_file_name : str = get_html_name(plant.info['name'])
        return li_template % (species_details_out,details_file_name,plant.info['name'])

    def create_plant_li(self,plant:Plant) -> str:
        li_template :str = '<li><a href="%s/%s">%s (%s)</a></li>'
        details_file_name :str = get_html_name(plant.info['plant_name'])
        info_tuple : tuple[str,str,str,str] = (
                plant_details_out,
                details_file_name,
                plant.info['plant_name'],
                plant.info['species_name']
                )
        return li_template % info_tuple 

    def render_species_overview(self,plants:list[PlantSpecies]) -> None:
        plant_lis :list[str] = []
        for plant in plants:
            plant_li : str = self.create_species_li(plant)
            plant_lis.append(plant_li)
        lis_str : str = '\n'.join(plant_lis)
        plant_li : str = self.species_overview_template.render(species_list_items=lis_str)
        write_html(species_overview_out,plant_li)

    def render_plant_overview(self,plants:list[Plant]) -> None:
        plant_lis : list[str] = []
        for plant in plants: 
            plant_li : str = self.create_plant_li(plant)
            plant_lis.append(plant_li)
        lis_str : str = '\n'.join(plant_lis)
        plant_li : str = self.plant_overview_template.render(plant_list_items=lis_str)
        write_html(plant_overview_out,plant_li)

    def render_species_details(self,plant:PlantSpecies) -> None:
        info_dict:dict[str,str] = plant.get_info_dict()
        species_html:str = self.species_details_template.render(info_dict)
        species_file_name = get_html_name(plant.info['name']) 
        species_full_name = os.path.join(species_details_out,species_file_name)
        write_html(species_full_name,species_html)

    def render_plant_details(self,plant:Plant) -> None:
        info_dict:dict[str,str]= plant.get_info_dict()
        plant_html:str = self.plant_details_template.render(info_dict)
        plant_file_name = get_html_name(plant.info['plant_name'])
        plant_full_name = os.path.join(plant_details_out,plant_file_name)
        write_html(plant_full_name,plant_html)

    def render_index(self) -> None:
        index_html = self.index_template.render()
        write_html(index_out,index_html)

    def render_all_species(self,plants:list[PlantSpecies]) -> None:
        self.render_species_overview(plants)
        for plant in plants:
            self.render_species_details(plant)

    def render_all_plants(self,plants:list[Plant]) -> None:
        self.render_plant_overview(plants)
        for plant in plants:
            self.render_plant_details(plant)

    def render_all(self,species:list[PlantSpecies],plants:list[Plant]) -> None:
        self.render_all_species(species)
        self.render_all_plants(plants)
        self.render_index()
