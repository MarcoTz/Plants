from common.PlantSpecies import PlantSpecies
from common.Plant import Plant
from common.types import *
from common.constants import * 
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

class HTMLRenderer: 

    env                       : jinja2.Environment
    species_overview_template : jinja2.Template
    species_details_template  : jinja2.Template
    plant_overview_template   : jinja2.Template
    plant_details_template    : jinja2.Template
    activities_template       : jinja2.Template
    header_template           : jinja2.Template
    footer_template           : jinja2.Template
    graveyard_template        : jinja2.Template
    gallery_template          : jinja2.Template
    image_viewer_template     : jinja2.Template

    plant_list   : list[Plant]
    species_list : list[PlantSpecies]
    graveyard    : list[GraveyardPlant]

    plants_next_waterings    : list[tuple[Plant,datetime.datetime]]
    plants_next_fertilizings : list[tuple[Plant,datetime.datetime]]
    
    def __init__(self,
                 plants     : list[Plant],
                 species    : list[PlantSpecies],
                 graveyard  : list[GraveyardPlant]) -> None:
        self.env = jinja2.Environment(loader=jinja2.FileSystemLoader(template_dir),autoescape=False)
        create_if_not_exists(out_dir)
        create_if_not_exists(os.path.join(out_dir,species_details_out))
        create_if_not_exists(os.path.join(out_dir,plant_details_out))
        self.load_templates()
        self.plant_list   = plants
        self.species_list = species
        self.assigned_growth = []
        self.graveyard = graveyard
        self.plants_next_waterings = []
        self.plants_next_fertilizings = []

        self.plant_list.sort(key = lambda x: x.info['plant_name'])
        self.species_list.sort(key=lambda x: x.info['name'])
    
    def load_templates(self) -> None:
        self.species_overview_template = self.env.get_template(species_overview_template_name)
        self.species_details_template  = self.env.get_template(species_details_template_name)
        self.plant_overview_template   = self.env.get_template(plant_overview_template_name)
        self.plant_details_template    = self.env.get_template(plant_details_template_name)
        self.index_template            = self.env.get_template(index_template_name)
        self.activities_template       = self.env.get_template(activity_log_template_name)
        self.header_template           = self.env.get_template(header_template_name)
        self.footer_template           = self.env.get_template(footer_template_name)
        self.graveyard_template        = self.env.get_template(graveyard_template_name)
        self.gallery_template          = self.env.get_template(gallery_template_name) 
        self.image_viewer_template     = self.env.get_template(image_view_template_name)

    def get_plants_species(self,species:str) -> list[Plant]: 
        species_li : list[Plant] = []
        for plant in self.plant_list:
            if plant.info['species_name'] == species:
                species_li.append(plant)
        return species_li

    def get_species_plant(self,plant:Plant) -> PlantSpecies | None:
        species_name : str = plant.info['species_name']
        for species in self.species_list:
            if species.info['name'] == species_name:
                return species
        return None

    def get_plant_locations(self) -> list[str]:
        locations : list[str] = []
        for plant in self.plant_list:
            plant_location = plant.info['current_location']
            if plant_location not in locations:
                locations.append(plant_location)
        return locations 

    def get_next_dates(self,plant:Plant) -> tuple[datetime.datetime | None,datetime.datetime | None]:
        species : PlantSpecies | None = self.get_species_plant(plant)
        if species is None:
            return (None,None)

        watering_interval    : int = species.info['avg_watering_days']
        next_watering_delta  : datetime.timedelta = datetime.timedelta(days=watering_interval)
        fertilizing_interval : int = species.info['avg_fertilizing_days']
        next_fertilizing_delta : datetime.timedelta = datetime.timedelta(days=fertilizing_interval)

        filter_fun : function = lambda y: lambda x: x['log_activity'].strip() == y 
        plant_activities : list[LogItem] = plant.info['plant_activities']
        watering_activities : list[LogItem] = list(filter(filter_fun('Watering'),plant_activities))
        watering_activities.sort(key=lambda x:x['log_date'])
        fertilizing_activities : list[LogItem] = list(filter(filter_fun('Fetilizing'),plant_activities))
        fertilizing_activities.sort(key=lambda x:x['log_date'])

        last_watering    : LogItem | None = watering_activities[-1]    if watering_activities != []    else None
        last_fertilizing : LogItem | None = fertilizing_activities[-1] if fertilizing_activities != [] else None

        next_watering    : datetime.datetime | None = None 
        next_fertilizing : datetime.datetime | None = None 
        current_date : datetime.datetime = datetime.datetime.now()
        if watering_interval != -1:
            last_date : datetime.datetime = last_watering['log_date'] if last_watering is not None else datetime.datetime.min
            next_watering = last_date + next_watering_delta
            next_watering = next_watering if next_watering > current_date else current_date

        if fertilizing_interval != -1:
            last_date : datetime.datetime = last_fertilizing['log_date'] if last_fertilizing is not None else datetime.datetime.min 
            next_fertilizing = last_date + next_fertilizing_delta
            next_fertilizing = next_fertilizing if next_fertilizing > current_date else current_date

        return (next_watering,next_fertilizing)

    def get_recent_activities_growth(self) -> tuple[list[tuple[str,GrowthItem]],list[tuple[str,LogItem]]]:
        last_week : datetime.datetime = datetime.datetime.now() - datetime.timedelta(weeks=1)
        recent_activities : list[tuple[str,LogItem]] = []
        recent_growth     : list[tuple[str,GrowthItem]] = []
        for plant in self.plant_list:
            plant_activities : list[LogItem] = plant.info['plant_activities']
            plant_activities = list(filter(lambda x: x['log_date'] >= last_week,plant_activities))
            recent_plant_activities : list[tuple[str,LogItem]] = list(map(lambda x: (plant.info['plant_name'],x),plant_activities))
            recent_activities.extend(recent_plant_activities)

            plant_growth     : list[GrowthItem] = plant.info['plant_growth']
            plant_growth = list(filter(lambda x: x['log_date'] >= last_week,plant_growth))
            recent_plant_growth : list[tuple[str,GrowthItem]] = list(map(lambda x: (plant.info['plant_name'],x),plant_growth))
            recent_growth.extend(recent_plant_growth)

        recent_activities.sort(key = lambda x: x[1]['log_date'],reverse=True)
        recent_growth.sort(key=lambda x: x[1]['log_date'],reverse=True)
        return (recent_growth,recent_activities)

    def create_species_li(self,plant:PlantSpecies) -> str:
        li_template       : str = '<div id="plant_list_item"><a href="%s/%s">%s</a>%s</div>'
        details_file_name : str = get_html_name(plant.info['name'])

        species_plants : list[Plant] = self.get_plants_species(plant.info['name'])
        img_str : str = ''
        if len(species_plants)>0:
            for species_plant in species_plants:
                if len(species_plant.images) == 0:
                    continue
                image_name : str = species_plant.images[0][1]
                image_path = os.path.join(img_dir,img_plants_dir,img_small_dir,image_name)
                img_str = '<br/><img id="plant_overview" src="%s"/>' %image_path

        return li_template % (species_details_out,details_file_name,plant.info['name'],img_str)

    def create_plant_li(self,plant:Plant) -> str:
        li_template :str = '<div id="plant_list_item"><a href="%s/%s">%s</a><br/><a href="%s/%s">%s</a>%s</div>'
        
        img_str : str = ''
        if len(plant.images) > 0:
            image_url = os.path.join(img_dir,img_plants_dir)
            image_url = os.path.join(image_url,img_small_dir,plant.images[0][1])
            img_str = '<br/><img id="plant_preview" src="%s"/>' %image_url
        details_file_name :str = get_html_name(plant.info['plant_name'])
        info_tuple : tuple[str,str,str,str,str,str,str] = (
                plant_details_out,
                details_file_name,
                plant.info['plant_name'],
                species_details_out,
                get_html_name(plant.info['species_name']),
                plant.info['species_name'],
                img_str
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
        height_cm = str(log_item['log_height_cm'])+'cm'
        tr += td_template % height_cm 
        width_cm = str(log_item['log_width_cm']) + 'cm'
        tr += td_template % width_cm 
        tr += td_template % log_item['log_note']
        tr += '</tr>'
        return tr
    
    def render_header(self,relative_up:bool) -> str: 
        link_prefix : str = '../' if relative_up else './'
        header_dict : dict[str,str] = { 
            'index_link'            : link_prefix + index_out,
            'plant_overview_link'   : link_prefix + plant_overview_out,
            'species_overview_link' : link_prefix + species_overview_out,
            'gallery_link'          : link_prefix + gallery_out,
            'activities_link'       : link_prefix + activity_log_out,
            'graveyard_link'        :link_prefix+graveyard_out} 
        return self.header_template.render(header_dict)

    def render_footer(self) -> str:
        image_viewer_str : str = self.image_viewer_template.render()
        last_build_date_str : str = datetime.datetime.now().strftime(date_format)
        footer_dict : dict[str,str] = {
                'num_plants':str(len(self.plant_list)),
                'image_viewer':image_viewer_str,
                'last_build_date':last_build_date_str
                }
        return self.footer_template.render(footer_dict)

    def render_species_overview(self) -> None:
        plant_lis :list[str] = []
        for plant in self.species_list:
            plant_li : str = self.create_species_li(plant)
            plant_lis.append(plant_li)
        lis_str : str = '\n'.join(plant_lis)
        header_str : str = self.render_header(False)
        footer_str : str = self.render_footer()
        plant_li : str = self.species_overview_template.render(species_list_items=lis_str,header=header_str,footer=footer_str)
        write_html(species_overview_out,plant_li)

    def render_plant_overview(self) -> None:
        plant_locations : list[str] = self.get_plant_locations()
        location_divs : dict[str,str] = {} 
        plant_lis : dict[str,list[str]] = {}
        for location in plant_locations:
            plant_lis[location] = []
            location_divs[location] = '<div id="location_group"><h2>%s</h2>%s</div>'

        for plant in self.plant_list: 
            plant_li : str = self.create_plant_li(plant)
            plant_location : str = plant.info['current_location']
            plant_lis[plant_location].append(plant_li)
        
        for location in plant_locations: 
            location_lis = '\n'.join(plant_lis[location])
            location_divs[location] = location_divs[location] % (location,location_lis)

        items_str : str = '\n'.join(list(location_divs.values()))
        header_str : str = self.render_header(False)
        footer_str : str = self.render_footer()
        plant_li : str = self.plant_overview_template.render(plant_list_items=items_str,header=header_str,footer=footer_str)
        write_html(plant_overview_out,plant_li)

    def render_species_details(self,plant:PlantSpecies) -> None:
        info_dict:dict[str,str] = plant.get_info_dict()
        header_str : str = self.render_header(True)
        footer_str : str = self.render_footer()
        info_dict['header'] = header_str
        info_dict['footer'] = footer_str

        species_file_name : str = get_html_name(plant.info['name']) 

        species_plants : list[Plant] = self.get_plants_species(plant.info['name'])
        species_plants_str : str = ''
        plants_images_list : list[str] = []
        images_path : str = os.path.join(img_dir,img_plants_dir)
        image_template  : str = '<figure id="plant_image"><img src="../%s"/><figcaption>%s,%s</figcaption></figure>'
        for species_plant in species_plants:
            plant_name = species_plant.info['plant_name']
            species_plants_str += '<a href="../%s/%s">%s</a><br/>' % (plant_details_out,get_html_name(plant_name),plant_name)
            for (image_date,image_name) in species_plant.images:
                image_path : str = os.path.join(images_path,image_name)
                image_date_str : str = image_date.strftime(date_format)
                new_img = image_template % (image_path,plant_name,image_date_str)
                plants_images_list.append(new_img)
        
        info_dict['plant_images'] = '\n'.join(plants_images_list)
        info_dict['species_plants'] = species_plants_str 

        species_html:str = self.species_details_template.render(info_dict)

        species_full_name = os.path.join(species_details_out,species_file_name)
        write_html(species_full_name,species_html)

    def render_plant_details(self,plant:Plant) -> None:
        info_dict:dict[str,str]= plant.get_info_dict()

        header_str : str = self.render_header(True)
        footer_str : str = self.render_footer()
        info_dict['header'] = header_str
        info_dict['footer'] = footer_str
 
        log_trs : list[str] = []
        watering_activities : list[LogItem] = []
        fertilizing_activities : list[LogItem] = []
        for log_item in plant.info['plant_activities']:
            if log_item['log_activity'].strip() == 'Watering':
                watering_activities.append(log_item)
            if log_item['log_activity'].strip() == 'Fertilizing':
                fertilizing_activities.append(log_item)
            log_tr = self.create_activity_tr(log_item,plant.info['plant_name'],False)
            log_trs.append(log_tr)
        info_dict['plant_activities'] = '\n'.join(log_trs)
        
        plant_species : str = info_dict['plant_species_name']
        species : PlantSpecies | None = self.get_species_plant(plant)
        if species is not None:
            a_template = '<a href="../%s/%s">%s</a>'
            species_link_tuple :tuple[str,str,str] = (species_details_out,get_html_name(plant_species),plant_species)
            info_dict['plant_species_name'] = a_template % species_link_tuple

        else: 
            print('Cannot find species %s for plant %s' % (plant_species,info_dict['plant_name']))
            info_dict['next_watering_date'] = ''
            info_dict['next_fertilizing_date'] = ''

        (next_watering_date,next_fertilizing_date) = self.get_next_dates(plant)
        if next_watering_date is not None:
             info_dict['next_watering_date'] = next_watering_date.strftime(date_format)
             self.plants_next_waterings.append((plant,next_watering_date))
        else:
            info_dict['next_watering_date'] = 'N/A'

        if next_fertilizing_date is not None: 
            info_dict['next_fertilizing_date'] = next_fertilizing_date.strftime(date_format)
            self.plants_next_fertilizings.append((plant,next_fertilizing_date))
        else: 
            info_dict['next_fertilizing_date'] = 'N/A' 

        growth_trs : list[str] = []
        for log_item in plant.info['plant_growth']:
            item_tr = self.create_growth_tr(log_item)
            growth_trs.append(item_tr)

        info_dict['plant_growth'] = '\n'.join(growth_trs)

        images_list : list[str] = [] 
        images_path : str = os.path.join(img_dir,img_plants_dir)
        image_template  : str = '<figure id="plant_image"><img src="../%s"/><figcaption>%s</figcaption></figure>'
        for (image_date,image_name) in plant.images:
            image_path : str = os.path.join(images_path,image_name)
            image_date_str : str = image_date.strftime(date_format)
            new_img = image_template % (image_path,image_date_str)
            images_list.append(new_img)
        if images_list == []:
            print('Could not find images for plant %s' % plant.info['plant_name'])
        info_dict['plant_images'] = '\n'.join(images_list)

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
        header_str : str = self.render_header(False)
        footer_str : str = self.render_footer()
        log_html : str = self.activities_template.render(activity_log_rows=tr_str,header=header_str,footer=footer_str)
        write_html(activity_log_out,log_html)

    def render_graveyard(self) -> None:
        header_str : str = self.render_header(False)
        footer_str : str = self.render_footer()
        rows_trs : list[str] = []
        for graveyard_plant in self.graveyard:
            new_tr : str = '<tr>'
            new_tr += '<td>%s</td>' % graveyard_plant['graveyard_plant']
            species : str = graveyard_plant['graveyard_species']
            species_link : str = '<a href="%s/%s">%s</a>' % (species_details_out,get_html_name(species),species)
            new_tr += '<td>%s</td>' % species_link 
            new_tr += '<td>%s</td>' % graveyard_plant['graveyard_planted'].strftime(date_format)
            new_tr += '<td>%s</td>' % graveyard_plant['graveyard_died'].strftime(date_format)
            new_tr += '<td>%s</td>' % graveyard_plant['graveyard_reason']
            new_tr += '</tr>'
            rows_trs.append(new_tr)

        graveyard_dict : dict[str,str] = {
                'header':header_str,
                'footer':footer_str,
                'graveyard_rows':'\n'.join(rows_trs)
                }
        graveyard_html : str = self.graveyard_template.render(graveyard_dict)
        write_html(graveyard_out,graveyard_html)

    def render_gallery(self) -> None:
        all_plant_images : list[tuple[datetime.datetime,str,str]] = []

        for plant in self.plant_list:
            plant_images : list[tuple[datetime.datetime,str]] = plant.images
            tuple_fun : function = lambda x: (x[0],plant.info['plant_name'],x[1])
            plant_images_with_name : list[tuple[datetime.datetime,str,str]] = list(map(tuple_fun,plant_images))
            all_plant_images.extend(plant_images_with_name)

        all_plant_images.sort(key=lambda x: x[0],reverse=True)
        plant_image_strs : list[str] = []
        image_dir : str = os.path.join(img_dir,img_plants_dir)
        image_template  : str = '<figure id="plant_image"><img src="%s/%s"/><figcaption>%s</figcaption></figure>'

        for plant_image in all_plant_images:
            plant_link : str = '<a href="%s/%s">%s</a>' % (plant_details_out,get_html_name(plant_image[1]),plant_image[1])
            caption : str = plant_link + ' '+ plant_image[0].strftime(date_format)
            plant_image_strs.append(image_template % (image_dir,plant_image[2],caption))

        header_str : str = self.render_header(False)
        footer_str : str = self.render_footer()

        gallery_dict : dict[str,str] = {
                'header': header_str,
                'footer': footer_str,
                'gallery_items': '\n'.join(plant_image_strs)
                }

        gallery_html : str = self.gallery_template.render(gallery_dict)
        write_html(gallery_out,gallery_html)


    def render_index(self) -> None:
        header_str : str = self.render_header(False)
        footer_str : str = self.render_footer()
        recent_growth_str : str = ''
        recent_activities_str : str = ''
        (recent_growth,recent_activities) = self.get_recent_activities_growth()

        for (plant_name,recent_growth_item) in recent_growth:
            recent_growth_str += '<tr>'
            recent_growth_str += '<td>%s</td>' % recent_growth_item['log_date'].strftime(date_format)
            recent_growth_str += '<td><a href="%s/%s">%s</a></td>' % (plant_details_out,get_html_name(plant_name),plant_name)
            recent_growth_str += '<td>%s</td>' % (str(recent_growth_item['log_height_cm']) + 'cm')
            recent_growth_str += '<td>%s</td>' % (str(recent_growth_item['log_width_cm']) + 'cm')
            recent_growth_str += '<td>%s</td>' % recent_growth_item['log_note']
            recent_growth_str += '</tr>'

        for (plant_name,recent_activity_item) in recent_activities:
            recent_activities_str += '<tr>'
            recent_activities_str += '<td>%s</td>' % recent_activity_item['log_date'].strftime(date_format)
            recent_activities_str += '<td>%s</td>' % recent_activity_item['log_activity']
            recent_activities_str += '<td><a href="%s/%s">%s</td>' % (plant_details_out,get_html_name(plant_name),plant_name)
            recent_activities_str += '<td>%s</td>' % recent_activity_item['log_note']
            recent_activities_str += '</tr>'
        
        current_date : datetime.datetime = datetime.datetime.now()
        next_week    : datetime.datetime = current_date + datetime.timedelta(weeks=1)
        filter_fun   : function = lambda x:x[1] <= next_week
        next_waterings    : list[tuple[Plant,datetime.datetime]] = list(filter(filter_fun,self.plants_next_waterings))
        next_fertilizings : list[tuple[Plant,datetime.datetime]] = list(filter(filter_fun,self.plants_next_fertilizings))

        next_activities : dict[tuple[str,str],list[str]] = {}
        for next_watering in next_waterings:
            activity_key :tuple[str,str] = (next_watering[1].strftime(date_format),'Watering')
            activity_value : str = next_watering[0].info['plant_name']
            if activity_key in next_activities:
                next_activities[activity_key].append(activity_value)
            else:
                next_activities[activity_key] = [activity_value]
        for next_fertilizing in next_fertilizings:
            activity_key = (next_fertilizing[1].strftime(date_format),'Fetilizing')
            activity_value = next_fertilizing[0].info['plant_name']
            if activity_key in next_activities:
                next_activities[activity_key].append(activity_value)
            else:
                 next_activities[activity_key] = [activity_value]
        
        next_activities_strs : list[str] = []
        activity_keys : list[tuple[str,str]] = list(next_activities.keys())
        activity_keys.sort(key=lambda x: x[0])
        next_activity_div : str = '<div class="next_activity">%s<br/>%s<br/>%s</div>'
        for activity_key in activity_keys:
            activity_date : str = activity_key[0]
            activity      : str = activity_key[1]
            activity_plants : list[str] = next_activities[activity_key]
            plant_link_template = '<a href="%s/%s">%s</a>'
            plant_link_fun : function = lambda x: plant_link_template % (plant_details_out,get_html_name(x),x)
            activity_plants_links : list[str] = list(map(plant_link_fun,activity_plants))
            activity_plants_str : str = '<br />'.join(activity_plants_links)
            next_activity_str : str = next_activity_div % (activity_date,activity,activity_plants_str)
            next_activities_strs.append(next_activity_str)
            

        index_dict : dict[str,str] = {
                'header': header_str,
                'footer':footer_str,
                'recent_growth_rows':recent_growth_str,
                'recent_activities_rows':recent_activities_str,
                'next_activity_items':'\n'.join(next_activities_strs)
                }
        index_html = self.index_template.render(index_dict)
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
        self.render_gallery()
        self.render_graveyard()
        self.render_index()
