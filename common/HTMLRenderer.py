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

    water_img           : str = '🌊'
    fertilize_img       : str = '💩'
    growth_img          : str = '📏'
    is_autowatered_img  : str = '✅'
    not_autowatered_img : str = '❌'

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
        self.plant_list               : list [Plant]                         = plants
        self.species_list             : list[PlantSpecies]                   = species
        self.graveyard                : list[GraveyardPlant]                 = graveyard
        self.plants_next_waterings    : list[tuple[Plant,datetime.datetime]] = []
        self.plants_next_fertilizings : list[tuple[Plant,datetime.datetime]] = []

        self.plant_list.sort(key = lambda x: x.info['plant_name'])
        self.species_list.sort(key=lambda x: x.info['name'])
    
    def load_templates(self) -> None:
        self.species_overview_template  : jinja2.Template  = self.env.get_template(species_overview_template_name)
        self.species_details_template   : jinja2.Template  = self.env.get_template(species_details_template_name)
        self.plant_overview_template    : jinja2.Template  = self.env.get_template(plant_overview_template_name)
        self.plant_details_template     : jinja2.Template  = self.env.get_template(plant_details_template_name)
        self.index_template             : jinja2.Template  = self.env.get_template(index_template_name)
        self.activities_template        : jinja2.Template  = self.env.get_template(activity_log_template_name)
        self.header_template            : jinja2.Template  = self.env.get_template(header_template_name)
        self.footer_template            : jinja2.Template  = self.env.get_template(footer_template_name)
        self.graveyard_template         : jinja2.Template  = self.env.get_template(graveyard_template_name)
        self.gallery_template           : jinja2.Template  = self.env.get_template(gallery_template_name) 
        self.image_viewer_template      : jinja2.Template  = self.env.get_template(image_view_template_name)

    def get_plants_species(self,species:str) -> list[Plant]: 
        species_li : list[Plant] = []
        for plant in self.plant_list:
            if plant.info['species_name'] == species:
                species_li.append(plant)
        return species_li

    def get_plant_locations(self) -> list[str]:
        locations : list[str] = []
        for plant in self.plant_list:
            plant_location = plant.info['current_location']
            if plant_location not in locations:
                locations.append(plant_location)
        return locations 

    def get_old_growth(self) -> list[Plant]:
        plant_date_list : list[Plant] = [] 
        two_weeks : datetime.datetime = datetime.datetime.now() - datetime.timedelta(weeks=2)

        for plant in self.plant_list:
            # skip dormant plants
            if plant.info['plant_health'] == 0: 
                continue
            plant_growth : list[GrowthItem] = plant.info['plant_growth']
            plant_growth.sort(key=lambda x: x['log_date'])
            last_growth_update : GrowthItem = plant_growth[-1]
            if last_growth_update['log_date'] < two_weeks: 
                plant_date_list.append(plant)

        return plant_date_list 

    def get_recent_activities_growth(self) -> tuple[
            list[tuple[str,GrowthItem]],
            dict[tuple[datetime.datetime,str],list[tuple[Plant,str]]]
            ]:
        last_week : datetime.datetime = datetime.datetime.now() - datetime.timedelta(weeks=1)
        recent_activities : dict[tuple[datetime.datetime,str],list[tuple[Plant,str]]] = {}
        recent_growth     : list[tuple[str,GrowthItem]] = []
        for plant in self.plant_list:
            plant_activities : list[LogItem] = plant.info['plant_activities']
            filter_fun : function = lambda x: x['log_date'] >= last_week
            plant_activities = list(filter(filter_fun,plant_activities))
            for plant_activity in plant_activities:
                activity_key = (plant_activity['log_date'],plant_activity['log_activity'])
                new_tuple : tuple[Plant,str] = (plant,plant_activity['log_note'])
                if activity_key in recent_activities.keys():
                    recent_activities[activity_key].append(new_tuple)
                else:
                    recent_activities[activity_key] = [new_tuple]

            plant_growth     : list[GrowthItem] = plant.info['plant_growth']
            plant_growth = list(filter(lambda x: x['log_date'] >= last_week,plant_growth))
            map_fun : function = lambda x: (plant.info['plant_name'],x)
            recent_plant_growth : list[tuple[str,GrowthItem]] = list(map(map_fun,plant_growth))
            recent_growth.extend(recent_plant_growth)

        recent_growth.sort(key=lambda x: x[1]['log_date'],reverse=True)
        return (recent_growth,recent_activities)

    def create_species_li(self,plant:PlantSpecies) -> str:
        li_template :str = '<div id="plant_list_item"><a href="%s/%s">%s</a>%s</div>'
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
        li_template       : str = '''
            <div class="plant_list_item">
                <a class="plant_link" href="%s/%s">%s</a>
                <br/>
                <div class="species_link">%s</div>
                %s
                <div class="temp_max">%s</div>
                <div class="temp_min">%s</div>
            </div>'''

        species_link : str = '<a href="%s/%s">%s</a>'
        
        max_temp : float = float('inf')
        min_temp : float = float('-inf')
        plant_species = plant.species
        if plant_species is not None : 
            species_name : str = plant.info['species_name']
            species_html_name : str = get_html_name(species_name)
            species_link : str = species_link % (species_details_out,species_html_name,species_name)
            max_temp = plant_species.info['temperature_max']
            min_temp = plant_species.info['temperature_min']
        else: 
            species_link = plant.info['species_name']
        
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
                species_link,
                img_str,
                str(max_temp),
                str(min_temp)
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
            location_divs[location] = '<div class="location_group"><h2>%s</h2>%s</div>'

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
        gallery_str : str = ''
        species_plants_str : str = ''
        for species_plant in species_plants:
            gallery_str += self.get_plant_gallery(species_plant,True)

            plant_name = species_plant.info['plant_name']
            species_plants_str += '<a href="../%s/%s">%s</a>&nbsp;' % (plant_details_out,get_html_name(plant_name),plant_name)

         
        info_dict['plant_images'] = gallery_str 
        info_dict['species_plants'] = species_plants_str 

        species_html:str = self.species_details_template.render(info_dict)

        species_full_name = os.path.join(species_details_out,species_file_name)
        write_html(species_full_name,species_html)

    def render_plant_details(self,plant:Plant) -> None:
        info_dict:dict[str,str]= plant.get_info_dict()
        plant_name : str = plant.info['plant_name'] 

        header_str : str = self.render_header(True)
        footer_str : str = self.render_footer()
        info_dict['header'] = header_str
        info_dict['footer'] = footer_str

        plant_health_div : str = '<div class="health%s health">%s</div>'
        health_str : str = str(info_dict['plant_health'])

        info_dict['plant_health'] =  plant_health_div % (health_str,health_str)
        info_dict['plant_autowater'] = self.is_autowatered_img if info_dict['plant_autowater'] else self.not_autowatered_img
 
        log_trs : list[tuple[datetime.datetime,str]] = []
        watering_activities    : list[LogItem] = []
        fertilizing_activities : list[LogItem] = []
        for log_item in plant.info['plant_activities']:
            if log_item['log_activity'].strip() == 'Watering':
                watering_activities.append(log_item)
            elif log_item['log_activity'].strip() == 'Fertilizing':
                fertilizing_activities.append(log_item)
            else:
                log_tr : str = self.create_activity_tr(log_item,plant_name,False)
                log_trs.append((log_item['log_date'],log_tr))
        if len(watering_activities) > 0:
            watering_activities.sort(key=lambda x:x['log_date'],reverse=True)
            last_watering : LogItem = watering_activities[0]
            watering_note : str = last_watering['log_note']
            if len(watering_activities) > 1: 
                watering_note += ', ' if watering_note.strip() != '' else ''
            date_str : str = last_watering['log_date'].strftime(date_format)
            info_dict['last_watering_date'] = date_str
            last_watering['log_note'] = watering_note
        else:
            info_dict['last_watering_date'] = 'N/A'
            print('Never watered plant %s' % plant.info['plant_name'])

        if len(fertilizing_activities) > 0:
            fertilizing_activities.sort(key=lambda x:x['log_date'],reverse=True)
            last_fertilizing : LogItem = fertilizing_activities[0]
            fertilizing_note : str = last_fertilizing['log_note']
            if len(fertilizing_activities) > 1: 
                fertilizing_note += ', ' if fertilizing_note.strip() != '' else ''
            date_str : str = last_fertilizing['log_date'].strftime(date_format)
            info_dict['last_fertilizing_date'] = date_str

            last_fertilizing['log_note'] = fertilizing_note
        else: 
            info_dict['last_fertilizing_date'] = 'N/A' 
            print('Never fertilized plant %s' % plant.info['plant_name'])



        log_trs.sort(key=lambda x:x[0],reverse=True)

        map_fun : function = lambda x : x[1]
        info_dict['plant_activities'] = '\n'.join(list(map(map_fun,log_trs)))
        
        plant_species : str = info_dict['plant_species_name']
        species : PlantSpecies | None = plant.species 
        if species is not None:
            a_template = '<a href="../%s/%s">%s</a>'
            species_link_tuple :tuple[str,str,str] = (species_details_out,get_html_name(plant_species),plant_species)
            info_dict['plant_species_name'] = a_template % species_link_tuple
            (next_watering_date,next_fertilizing_date) = plant.get_next_dates()
            if next_watering_date is not None and plant.info['plant_health'] != 0:
                 info_dict['next_watering_date'] = next_watering_date.strftime(date_format)
                 self.plants_next_waterings.append((plant,next_watering_date))
            else:
                info_dict['next_watering_date'] = 'N/A'
    
            if next_fertilizing_date is not None and plant.info['plant_health'] != 0: 
                info_dict['next_fertilizing_date'] = next_fertilizing_date.strftime(date_format)
                self.plants_next_fertilizings.append((plant,next_fertilizing_date))
            else: 
                info_dict['next_fertilizing_date'] = 'N/A' 

        else: 
            print('Cannot find species %s for plant %s' % (plant_species,info_dict['plant_name']))
            info_dict['next_watering_date'] = ''
            info_dict['next_fertilizing_date'] = ''
        

        growth_dates   : list[str] = []
        growth_widths  : list[str] = []
        growth_heights : list[str] = []
        for log_item in plant.info['plant_growth']:
            growth_dates.append('"%s"' % log_item['log_date'].strftime(date_format))
            growth_widths.append(str(log_item['log_width_cm']))
            growth_heights.append(str(log_item['log_height_cm']))

        growth_dates.reverse()
        growth_heights.reverse()
        growth_widths.reverse()
        info_dict['plant_growth_dates'] = '[%s]' % (', '.join(growth_dates))
        info_dict['plant_growth_heights'] = '[%s]' % (', '.join(growth_heights))
        info_dict['plant_growth_widths'] = '[%s]' % (', '.join(growth_widths))
            
        info_dict['plant_images'] = self.get_plant_gallery(plant,True)

        plant_html:str = self.plant_details_template.render(info_dict)
        plant_file_name = get_html_name(plant.info['plant_name'])
        plant_full_name = os.path.join(plant_details_out,plant_file_name)
        write_html(plant_full_name,plant_html)

    def render_activity_log(self) -> None: 
        tr_list : list[str] = []
        all_activities : list[tuple[LogItem,list[str]]] = []
        for plant in self.plant_list:
            new_activities : list[LogItem] = plant.info['plant_activities']
            for new_activity in new_activities:
                same_activity : list[tuple[LogItem,list[str]]] = list(filter(lambda x: x[0]==new_activity,all_activities))
                if len(same_activity) > 0:
                    ind = all_activities.index(same_activity[0])
                    curr_note : str = same_activity[0][0]['log_note']
                    new_note : str = new_activity['log_note'].strip()
                    new_note = curr_note + ', '+  new_note if new_note != '' and new_note not in curr_note else curr_note
                    same_activity[0][0]['log_note'] = new_note
                    same_activity[0][1].append(plant.info['plant_name'])
                    del all_activities[ind]
                    all_activities.append(same_activity[0])
                else:
                    all_activities.append((new_activity,[plant.info['plant_name']]))
                    
        all_activities.sort(key = lambda x:x[0]['log_date'],reverse=True)
        
        for (log_item,plants) in all_activities:
            item_tr = self.create_activity_tr(log_item,', '.join(plants),True)
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
    
    def get_plant_gallery(self,plant:Plant,relative_up:bool=False) -> str:
        plant_div_template : str = '''
            <div class="image_plant_container">
                <h2><a href="%s/%s">%s</a></h2>
                <div class="images_plant">%s</div>
                <div class="img_controls">
                <div class="left_arrow">&#9754;</div>
                <div class="right_arrow">&#9755;</div>
                </div>
            </div>'''
        img_template : str = '''
            <figure class="plant_image">
                <img src="%s"/>
                <figcaption>
                    <div class="img_date">%s</div> <div class="img_nr">%s/%s</div>
                </figcaption>
            </figure>'''
        images_strs : list[str] = []
        plant_images : list[tuple[datetime.datetime,str]] = plant.images
        plant_images.sort(key=lambda x: x[0],reverse=True)

        for (img_date,img_name) in plant.images:
            img_path : str = os.path.join(img_dir,img_plants_dir,img_name)
            img_path : str = os.path.join('..',img_path) if relative_up else img_path
            current_ind : int = plant.images.index((img_date,img_name))+1
            img_date_str : str = img_date.strftime(date_format)
            current_img : str = img_template % (img_path,img_date_str,str(current_ind),str(len(plant_images)))
            images_strs.append(current_img)
        plant_name : str = plant.info['plant_name']
        plant_html_name : str = get_html_name(plant_name)
        images_str : str = '\n'.join(images_strs)
        current_plant_div : str = plant_div_template % (plant_details_out,plant_html_name,plant_name, images_str)

        return current_plant_div

    def render_gallery(self) -> None:
        
        plant_divs : list[str] = [] 
        for plant in self.plant_list:
            current_plant_div = self.get_plant_gallery(plant)
            plant_divs.append(current_plant_div)

        header_str : str = self.render_header(False)
        footer_str : str = self.render_footer()

        gallery_dict : dict[str,str] = {
                'header': header_str,
                'footer': footer_str,
                'gallery_items': '\n'.join(plant_divs)
                }

        gallery_html : str = self.gallery_template.render(gallery_dict)
        write_html(gallery_out,gallery_html)

    def get_recent_activities_str(self,recent_activities) -> str:
        recent_activities_str : str = ''

        for (log_date, log_activity) in recent_activities:
            recent_activities_str += '<tr>'
            recent_activities_str += '<td>%s</td>' % log_date.strftime(date_format)
            recent_activities_str += '<td>%s</td>' % log_activity 
            plant_strs : list[str] = []
            notes_strs : list[str] = []
            for (plant,note) in recent_activities[(log_date,log_activity)]:
                plant_name : str = plant.info['plant_name']
                plant_strs.append('<a href="%s/%s">%s</a>' % (plant_details_out,get_html_name(plant_name),plant_name))
                add_note : bool = not note.strip() == ''
                add_note = add_note and note not in notes_strs
                add_note = add_note and 'Last Watering' not in note
                add_note = add_note and 'Last Fertilizing' not in note
                if add_note:
                    notes_strs.append(note)
            recent_activities_str += '<td>%s</td>' % (', '.join(plant_strs))
            recent_activities_str += '<td>%s</td>' % (', '.join(notes_strs)) 
            recent_activities_str += '</tr>'

        return recent_activities_str

    def get_recent_growth_str(self,recent_growth) -> str:
        recent_growth_str : str = '' 
        for (plant_name,recent_growth_item) in recent_growth:
            recent_growth_str += '<tr>'
            recent_growth_str += '<td>%s</td>' % recent_growth_item['log_date'].strftime(date_format)
            recent_growth_str += '<td><a href="%s/%s">%s</a></td>' % (plant_details_out,get_html_name(plant_name),plant_name)
            recent_growth_str += '<td>%s</td>' % (str(recent_growth_item['log_height_cm']) + 'cm')
            recent_growth_str += '<td>%s</td>' % (str(recent_growth_item['log_width_cm']) + 'cm')
            recent_growth_str += '<td>%s</td>' % recent_growth_item['log_note']
            recent_growth_str += '</tr>'

        return recent_growth_str


    def get_next_activities_str(self) -> str:
        current_date : datetime.datetime = datetime.datetime.now()
        next_week    : datetime.datetime = current_date + datetime.timedelta(weeks=1)
        filter_fun   : function = lambda x:x[1] <= next_week


        next_waterings      : list[tuple[Plant,datetime.datetime]] = list(filter(filter_fun,self.plants_next_waterings))
        next_fertilizings   : list[tuple[Plant,datetime.datetime]] = list(filter(filter_fun,self.plants_next_fertilizings))
        next_growth_updates : list[tuple[Plant,datetime.datetime]] = list(map(lambda x: (x,current_date),self.get_old_growth()))
        next_activities_list : list[tuple[Plant,datetime.datetime,str]] = []

        map_fun : function = lambda x: lambda y: (y[0],y[1],x)
        watering_fun = map_fun('%s Watering %s' % (self.water_img,self.water_img))
        next_waterings_list = list(map(watering_fun,next_waterings))
        next_activities_list.extend(next_waterings_list)
        fertilizing_fun = map_fun('%s Fertilizing %s' % (self.fertilize_img,self.fertilize_img))
        next_fertilizings_list = list(map(fertilizing_fun,next_fertilizings))
        next_activities_list.extend(next_fertilizings_list)
        growth_fun = map_fun('%s Growth %s' % (self.growth_img,self.growth_img))
        next_growth_list = list(map(growth_fun,next_growth_updates))
        next_activities_list.extend(next_growth_list)


        next_activities : dict[tuple[datetime.date,str],list[Plant]] = {}

        for (plant,next_date,next_type) in next_activities_list:
            key_tuple : tuple[datetime.date,str] = (next_date.date(),next_type)
            if key_tuple in next_activities.keys():
                next_activities[key_tuple].append(plant)
            else:
                next_activities[key_tuple] = [plant]

        next_activity_template : str = '<div class="next_activity_item"><div class="activity_header">%s<br/>%s</div>%s</div>'
        plant_link_template = '<a href="%s/%s">%s</a>'
        next_activity_strs : list[str] = [] 
        next_activity_keys : list[tuple[datetime.date,str]] = list(next_activities.keys())
        next_activity_keys.sort(key = lambda x: x[0])
        for activity_key in next_activity_keys:
            next_date = activity_key[0]
            weekday_str : str = weekday_strs[next_date.weekday()]
            date_str : str = weekday_str + ', ' + next_date.strftime(date_format)

            plant_links : list[str] = [] 
            for plant in next_activities[activity_key]:
                plant_name : str = plant.info['plant_name']
                new_plant_link : str = plant_link_template % (plant_details_out,get_html_name(plant_name),plant_name)
                plant_links.append(new_plant_link)
            
            next_activity : str = next_activity_template % (date_str,activity_key[1], '\n'.join(plant_links))
            next_activity_strs.append(next_activity)

        return '\n'.join(next_activity_strs)

    def get_hall_of_fame(self) -> dict[str,str]:
        plant_link_template : str = '<div class="hall_of_fame_item">%s<br/><a href="%s">%s</a><br/>%s</div>'
        def get_plant_str(plant:Plant,winner_str:str,winning_stat:str):
            plant_name : str = plant.info['plant_name']
            plant_link : str = plant_details_out + '/' + get_html_name(plant_name)
            return plant_link_template % (winner_str,plant_link,plant_name,winning_stat) 

        plants_by_height : list[Plant] = self.plant_list.copy()
        plants_by_height.sort(key=lambda x:x.current_height)

        tallest_plant  : Plant = plants_by_height[-1]
        tallest_height_str : str = str(tallest_plant.current_height) + 'cm'
        tallest_plant_str : str = get_plant_str(tallest_plant,'Tallest Plant',tallest_height_str)

        shortest_plant : Plant = plants_by_height[0]
        shortest_height_str : str = str(shortest_plant.current_width) + 'cm'
        shortest_plant_str : str = get_plant_str(shortest_plant,'Shortest Plant',shortest_height_str) 

        plants_by_width : list[Plant] = self.plant_list.copy()
        plants_by_width.sort(key=lambda x:x.current_width)

        widest_plant : Plant = plants_by_width[-1]
        widest_width_str : str = str(widest_plant.current_width) + 'cm'
        widest_plant_str : str = get_plant_str(widest_plant,'Widest Plant',widest_width_str)
        thinnest_plant : Plant = plants_by_width[0]
        thinnest_width_str : str = str(thinnest_plant.current_width) + 'cm'
        thinnest_plant_str : str = get_plant_str(thinnest_plant,'Thinnest Plant',thinnest_width_str)

        plants_by_growth : list[Plant] = self.plant_list.copy()
        plants_by_growth = list(filter(lambda x: len(x.info['plant_growth'])>2,plants_by_growth))

        def get_growth_diff(plant:Plant):
            plant.info['plant_growth'].sort(key=lambda x:x['log_date'])
            x : GrowthItem = plant.info['plant_growth'][0]
            y : GrowthItem = plant.info['plant_growth'][-1]
            height_diff : float = x['log_height_cm'] - y['log_height_cm']
            width_diff : float = x['log_width_cm'] - y['log_width_cm']
            growth_diff : float = height_diff + width_diff
            time_diff : float = float((x['log_date'] - y['log_date']).days)
            if time_diff == 0.0:
                return 0
            return growth_diff/time_diff
        plants_by_growth.sort(key=get_growth_diff)
        fastest_plant : Plant = plants_by_growth[-1]
        fastest_growth_diff : str = '{:.2f}'.format(get_growth_diff(fastest_plant)) + 'cm/day'
        fastest_plant_str : str = get_plant_str(fastest_plant,'Fastest Growing Plant',fastest_growth_diff)
        slowest_plant : Plant = plants_by_growth[0]
        slowest_growth_diff : str = '{:.2f}'.format(get_growth_diff(slowest_plant)) + 'cm/day'
        slowest_plant_str : str = get_plant_str(slowest_plant,'Slowest Growing Plant',slowest_growth_diff)

        return {
                'tallest_plant':tallest_plant_str,
                'shortest_plant':shortest_plant_str,
                'widest_plant':widest_plant_str,
                'thinnest_plant':thinnest_plant_str,
                'fastest_plant':fastest_plant_str,
                'slowest_plant':slowest_plant_str
                }

    def render_index(self) -> None:
        header_str : str = self.render_header(False)
        footer_str : str = self.render_footer()

        (recent_growth,recent_activities) = self.get_recent_activities_growth()
        recent_growth_str : str = self.get_recent_growth_str(recent_growth)
        recent_activities_str : str = self.get_recent_activities_str(recent_activities)
        next_activities_str : str = self.get_next_activities_str()
        
        autowatered_plants : list[Plant] = list(filter(lambda x:x.info['auto_water'],self.plant_list))
        autowater_str : str = ''
        autowater_div : str = '<div class="autowater_item"><a href="%s">%s</a></div>'
        for auto_plant in autowatered_plants:
            plant_name : str = auto_plant.info['plant_name']
            plant_link : str = plant_details_out + '/'+get_html_name(plant_name)
            autowater_str += autowater_div % (plant_link, plant_name)

        hall_of_fame_dict : dict[str,str] = self.get_hall_of_fame()

        index_dict : dict[str,str] = {
                'header': header_str,
                'footer':footer_str,
                'recent_growth_rows':recent_growth_str,
                'recent_activities_rows':recent_activities_str,
                'autowatered_plants':autowater_str,
                'next_activities':next_activities_str,
                }

        index_dict = index_dict | hall_of_fame_dict
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
