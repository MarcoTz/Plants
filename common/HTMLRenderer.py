from common.PlantSpecies import PlantSpecies
from common.Plant import Plant
from common.common import *
import jinja2
import os


def create_if_not_exists(dir_name:str)->None:
    if not os.path.exists(dir_name):
        os.makedirs(dir_name) 
def write_html(out_file_name:str,html_contents:str) -> None:
    out_file_path = os.path.join(out_dir,out_file_name)
    out_file = open(out_file_path,'w+')
    out_file.write(html_contents)
    out_file.close()
def get_html_name(plant_name:str) -> str:
    return plant_name.replace(' ','')+'.html'

def species_exists(species_name:str) -> bool:
    species_file_name = species_name.replace(' ','')+'.json'
    species_path = os.path.join(species_dir,species_file_name)
    return os.path.exists(species_path)

class HTMLRenderer: 

    env                       : jinja2.Environment
    species_overview_template : jinja2.Template
    species_details_template  : jinja2.Template
    plant_overview_template   : jinja2.Template
    plant_details_template    : jinja2.Template

    plant_list   : list[Plant]
    species_list : list[PlantSpecies]
    activity_log : list[LogItem]

    def __init__(self,plants:list[Plant],species:list[PlantSpecies],activities:list[LogItem]) -> None:
        self.env = jinja2.Environment(loader=jinja2.FileSystemLoader(template_dir),autoescape=False)
        create_if_not_exists(out_dir)
        create_if_not_exists(os.path.join(out_dir,species_details_out))
        create_if_not_exists(os.path.join(out_dir,plant_details_out))
        self.load_templates()
        self.plant_list   = plants
        self.species_list = species
        self.activity_log = activities
    
    def load_templates(self) -> None:
        self.species_overview_template = self.env.get_template(species_overview_template_name)
        self.species_details_template  = self.env.get_template(species_details_template_name)
        self.plant_overview_template   = self.env.get_template(plant_overview_template_name)
        self.plant_details_template    = self.env.get_template(plant_details_template_name)
        self.index_template            = self.env.get_template(index_template_name)

    def get_plant_logs(self,plant_name:str) -> list[LogItem]:
        return list(filter(lambda x: x['log_plant'] == plant_name,self.activity_log))

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

    def create_activity_tr(self,log_item:LogItem) -> str: 
        tr_template = '<tr><td>%s</td><td>%s</td><td>%s</td></tr>'
        tr_tuple = (log_item['log_date'].strftime(date_format), log_item['log_activity'], log_item['log_note'])
        return tr_template % tr_tuple

    def render_species_overview(self) -> None:
        plant_lis :list[str] = []
        for plant in self.species_list:
            plant_li : str = self.create_species_li(plant)
            plant_lis.append(plant_li)
        lis_str : str = '\n'.join(plant_lis)
        plant_li : str = self.species_overview_template.render(species_list_items=lis_str)
        write_html(species_overview_out,plant_li)

    def render_plant_overview(self) -> None:
        plant_lis : list[str] = []
        for plant in self.plant_list: 
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

        plant_species : str = info_dict['plant_species_name']
        if species_exists(info_dict['plant_species_name']):
            a_template = '<a href="../%s/%s">%s</a>'
            info_dict['plant_species_name'] = a_template % (species_details_out,get_html_name(plant_species),plant_species)

        plant_log : list[LogItem] = self.get_plant_logs(info_dict['plant_name'])
        log_trs : list[str] = []
        for log_item in plant_log:
            log_tr = self.create_activity_tr(log_item)
            log_trs.append(log_tr)
        info_dict['plant_activities'] = '\n'.join(log_trs)

        plant_html:str = self.plant_details_template.render(info_dict)
        plant_file_name = get_html_name(plant.info['plant_name'])
        plant_full_name = os.path.join(plant_details_out,plant_file_name)
        write_html(plant_full_name,plant_html)

    def render_index(self) -> None:
        index_html = self.index_template.render()
        write_html(index_out,index_html)

    def render_all_species(self) -> None:
        self.render_species_overview()
        for plant in self.species_list:
            self.render_species_details(plant)

    def render_all_plants(self) -> None:
        self.render_plant_overview()
        for plant in self.plant_list:
            self.render_plant_details(plant)

    def render_all(self) -> None:
        self.render_all_species()
        self.render_all_plants()
        self.render_index()
