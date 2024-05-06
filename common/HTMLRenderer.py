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
    activities_template       : jinja2.Template

    plant_list   : list[Plant]
    species_list : list[PlantSpecies]
    
    def __init__(self,
                 plants     : list[Plant],
                 species    : list[PlantSpecies]) -> None:
        self.env = jinja2.Environment(loader=jinja2.FileSystemLoader(template_dir),autoescape=False)
        create_if_not_exists(out_dir)
        create_if_not_exists(os.path.join(out_dir,species_details_out))
        create_if_not_exists(os.path.join(out_dir,plant_details_out))
        self.load_templates()
        self.plant_list   = plants
        self.species_list = species

        self.assigned_growth = []
    
    def load_templates(self) -> None:
        self.species_overview_template = self.env.get_template(species_overview_template_name)
        self.species_details_template  = self.env.get_template(species_details_template_name)
        self.plant_overview_template   = self.env.get_template(plant_overview_template_name)
        self.plant_details_template    = self.env.get_template(plant_details_template_name)
        self.index_template            = self.env.get_template(index_template_name)
        self.activities_template       = self.env.get_template(activity_log_template_name)

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

    def create_activity_tr(self,log_item:LogItem,plant_name:str,include_plant:bool) -> str: 
        td_template : str = '<td>%s</td>'
        tr : str = '<tr>'
        tr += td_template % log_item['log_date'].strftime(date_format)
        tr += td_template % log_item['log_activity']
        plant_link_template : str = '<td><a href="%s/%s">%s</a></td>'
        plant_link_tuple : tuple[str,str,str] = (plant_details_out,get_html_name(plant_name),plant_name)
        tr += plant_link_template % plant_link_tuple if include_plant else ''
        tr += td_template % log_item['log_note']
        tr += '</tr>'
        return tr 

    def create_growth_tr(self,log_item:GrowthItem) -> str:
        td_template : str = '<td>%s</td>'
        tr : str = '<tr>'
        tr += td_template % log_item['log_date'].strftime(date_format)
        tr += td_template % log_item['log_height_cm'] + 'cm'
        tr += td_template % log_item['log_width_cm'] + 'cm'
        tr += td_template % log_item['log_note']
        tr += '</tr>'
        return tr

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
        else: 
            print('Cannot find species %s for plant %s' % (plant_species,info_dict['plant_name']))

        log_trs : list[str] = []
        for log_item in plant.info['plant_activities']:
            log_tr = self.create_activity_tr(log_item,plant.info['plant_name'],False)
            log_trs.append(log_tr)
        info_dict['plant_activities'] = '\n'.join(log_trs)

        growth_trs : list[str] = []
        for log_item in plant.info['plant_growth']:
            item_tr = self.create_growth_tr(log_item)
            growth_trs.append(item_tr)
        info_dict['plant_growth'] = '\n'.join(growth_trs)

        plant_html:str = self.plant_details_template.render(info_dict)
        plant_file_name = get_html_name(plant.info['plant_name'])
        plant_full_name = os.path.join(plant_details_out,plant_file_name)
        write_html(plant_full_name,plant_html)

    def render_activity_log(self) -> None: 
        tr_list : list[str] = []
        all_activities : list[tuple[str,LogItem]] = [] 
        for plant in self.plant_list:
            new_activities : list[LogItem] = plant.info['plant_activities']
            activity_tuples = list(map(lambda x: (plant.info['plant_name'],x),new_activities))
            all_activities.extend(activity_tuples)

        for log_item in all_activities:
            item_tr = self.create_activity_tr(log_item[1],log_item[0],True)
            tr_list.append(item_tr)

        tr_str : str = '\n'.join(tr_list)
        log_html : str = self.activities_template.render(activity_log_rows=tr_str)
        write_html(activity_log_out,log_html)

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
        self.render_activity_log()
        self.render_index()
