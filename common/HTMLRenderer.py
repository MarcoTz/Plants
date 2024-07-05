from common.PlantSpecies import PlantSpecies
from common.Plant import Plant
from common.PlantManager import PlantManager
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

    manager : PlantManager

    def __init__(self, manager:PlantManager) -> None:
        self.env = jinja2.Environment(loader=jinja2.FileSystemLoader(template_dir),autoescape=False)

        create_if_not_exists(out_dir)
        create_if_not_exists(os.path.join(out_dir,species_details_out))
        create_if_not_exists(os.path.join(out_dir,plant_details_out))

        self.load_templates()
        self.manager                  : PlantManager = manager

    
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


    def create_species_li(self,plant:PlantSpecies) -> str:
        li_template :str = '<div id="plant_list_item"><a href="%s/%s">%s</a>%s</div>'
        details_file_name : str = get_html_name(plant.info['name'])

        species_plants : list[Plant] = self.manager.get_plants_species(plant.info['name'])
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

    def create_activity_tr(self,log_item:LogItem,plant_name:str,include_plant:bool,include_activity:bool=True) -> str: 
        td_template : str = '<td>%s</td>'
        tr : str = '<tr>'
        tr += td_template % log_item['log_date'].strftime(date_format)
        if include_activity:
            tr += td_template % log_item['log_activity']
        if include_plant:
            plant_link_template : str = '<td><a href="%s/%s">%s</a></td>'
            plant_link_tuple : tuple[str,str,str] = (plant_details_out,get_html_name(plant_name),plant_name)
            tr += plant_link_template % plant_link_tuple 
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
                'num_plants':str(len(self.manager.plants)),
                'image_viewer':image_viewer_str,
                'last_build_date':last_build_date_str
                }
        return self.footer_template.render(footer_dict)

    def render_species_overview(self) -> None:
        plant_lis :list[str] = []
        for plant in self.manager.species:
            plant_li : str = self.create_species_li(plant)
            plant_lis.append(plant_li)
        lis_str : str = '\n'.join(plant_lis)
        header_str : str = self.render_header(False)
        footer_str : str = self.render_footer()
        plant_li : str = self.species_overview_template.render(species_list_items=lis_str,header=header_str,footer=footer_str)
        write_html(species_overview_out,plant_li)

    def render_plant_overview(self) -> None:
        plant_locations : list[str] = self.manager.get_plant_locations()
        location_divs : dict[str,str] = {} 
        plant_lis : dict[str,list[str]] = {}
        for location in plant_locations:
            plant_lis[location] = []
            location_divs[location] = '<div class="location_group"><h2 id="%s">%s</h2>%s</div>'

        for plant in self.manager.plants: 
            plant_li : str = self.create_plant_li(plant)
            plant_location : str = plant.info['current_location']
            plant_lis[plant_location].append(plant_li)
        
        for location in plant_locations: 
            location_lis = '\n'.join(plant_lis[location])
            location_divs[location] = location_divs[location] % (location,location,location_lis)

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

        species_plants : list[Plant] = self.manager.get_plants_species(plant.info['name'])
        gallery_str : str = ''
        species_plants_str : str = ''
        for species_plant in species_plants:
            gallery_str += self.get_plant_gallery(species_plant,True)
            plant_name : str = species_plant.info['plant_name']
            plant_html_name : str = get_html_name(plant_name)
            species_plants_str += '<a href="../%s/%s">%s</a>&nbsp;' % (plant_details_out,plant_html_name,plant_name)

         
        info_dict['plant_images'] = gallery_str 
        info_dict['species_plants'] = species_plants_str 

        species_html:str = self.species_details_template.render(info_dict)

        species_full_name = os.path.join(species_details_out,species_file_name)
        write_html(species_full_name,species_html)

    def get_plant_activities(self,plant:Plant) -> dict[str,str]:
        plant_name : str = plant.info['plant_name']

        activities_list : list[LogItem] = plant.activities.copy()
        activities_list.sort(key=lambda x:x['log_date'],reverse=True)

        water_trs : list[str] = []
        fertilize_trs : list[str] = []
        activity_trs : list[str] = []
        for log_item in plant.activities:
            match log_item['log_activity']:
                case 'Watering':
                    log_tr : str = self.create_activity_tr(log_item,plant_name,False,False)
                    water_trs.append(log_tr)
                case 'Fertilizing':
                    log_tr : str = self.create_activity_tr(log_item,plant_name,False,False)
                    fertilize_trs.append(log_tr)
                case _:
                    log_tr : str = self.create_activity_tr(log_item,plant_name,False,True)
                    activity_trs.append(log_tr)

        return { 
                'watering_table':'\n'.join(water_trs),
                'fertilizing_table':'\n'.join(fertilize_trs),
                'activity_table':'\n'.join(activity_trs)
                }

    def get_growth_log_graph(self,plant:Plant) -> dict[str,str]: 
        info_dict : dict[str,str] = {} 

        growth_dates   : list[str] = []
        growth_widths  : list[str] = []
        growth_heights : list[str] = []
        for log_item in plant.growth:
            growth_dates.append('"%s"' % log_item['log_date'].strftime(date_format))
            growth_widths.append(str(log_item['log_width_cm']))
            growth_heights.append(str(log_item['log_height_cm']))

        info_dict['plant_growth_dates'] = '[%s]' % (', '.join(growth_dates))
        info_dict['plant_growth_heights'] = '[%s]' % (', '.join(growth_heights))
        info_dict['plant_growth_widths'] = '[%s]' % (', '.join(growth_widths))

        return info_dict

    def render_plant_details(self,plant:Plant) -> None:
        info_dict:dict[str,str]= plant.get_info_dict()

        header_str : str = self.render_header(True)
        footer_str : str = self.render_footer()
        info_dict['header'] = header_str
        info_dict['footer'] = footer_str

        plant_health_div : str = '<div class="health%s health">%s</div>'
        health_str : str = str(info_dict['plant_health'])

        info_dict['plant_health'] =  plant_health_div % (health_str,health_str)
        info_dict['plant_autowater'] = is_autowatered_img if info_dict['plant_autowater'] else not_autowatered_img
        location_link : str = '<a href="../%s#%s">%s</a>'
        current_location : str = info_dict['plant_location']
        info_dict['plant_location'] = location_link % (plant_overview_out,current_location,current_location)

        info_dict = info_dict | self.get_plant_activities(plant)
        
        plant_species : str = info_dict['plant_species_name']
        species : PlantSpecies | None = plant.species 
        if species is not None:
            a_template = '<a href="../%s/%s">%s</a>'
            species_link_tuple :tuple[str,str,str] = (species_details_out,get_html_name(plant_species),plant_species)
            info_dict['plant_species_name'] = a_template % species_link_tuple
        else: 
            print('Cannot find species %s for plant %s' % (plant_species,info_dict['plant_name']))

        info_dict = info_dict | self.get_growth_log_graph(plant) 
        info_dict['plant_images'] = self.get_plant_gallery(plant,True)

        plant_html:str = self.plant_details_template.render(info_dict)
        plant_file_name = get_html_name(plant.info['plant_name'])
        plant_full_name = os.path.join(plant_details_out,plant_file_name)
        write_html(plant_full_name,plant_html)

    def render_activity_log(self) -> None: 
        tr_list : list[str] = []
        all_activities : list[tuple[LogItem,list[str]]] = []
        for plant in self.manager.plants:
            new_activities : list[LogItem] = plant.activities
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
        for graveyard_plant in self.manager.graveyard:
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
        plant_path : str = plant_details_out
        plant_path : str = os.path.join('..',plant_path) if relative_up else plant_path
        current_plant_div : str = plant_div_template % (plant_path,plant_html_name,plant_name, images_str)

        return current_plant_div

    def render_gallery(self) -> None:
        
        plant_divs : list[str] = [] 
        for plant in self.manager.plants:
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
        
        next_activity_list : list[tuple[Plant,str,datetime.datetime]] = self.manager.get_next_activity_dates()

        next_growth_updates : list[tuple[Plant,datetime.datetime]] = list(map(lambda x: (x,current_date),self.manager.get_old_growth()))

        growth_fun : function = lambda x: (x[0],'Growth',x[1])
        next_growth_list = list(map(growth_fun,next_growth_updates))
        next_activity_list.extend(next_growth_list)

        next_activities : dict[datetime.date,dict[str,list[Plant]]] = {}

        for (plant,next_type,next_date,) in next_activity_list:
            next_key : datetime.date = next_date.date()
            if next_key in next_activities.keys(): 
                if next_type in next_activities[next_key].keys():
                    next_activities[next_key][next_type].append(plant)
                else:
                    next_activities[next_key][next_type]=[plant]
            else:
                next_activities[next_key] = {}
                next_activities[next_key][next_type] = [plant]

        next_dates : list[datetime.date] = list(next_activities.keys())
        next_dates.sort()
        
        next_activity_template : str = '<div class="next_activity_item">%s<br/>%s</div>'
        next_activity_type_template : str = '<div class="activity_header">%s<br/>%s</div>'
        plant_link_template = '<a href="%s/%s">%s</a>'
        next_activities_strs : list[str] = []

        for next_date in next_dates:
            date_activities : list[str] = []
            next_types : list[str] = list(next_activities[next_date].keys())
            def sort_types(ty:str) -> int:
                match ty: 
                    case 'Watering':
                        return 0
                    case 'Fertilizing':
                        return 1
                    case _:
                        return 2
            next_types.sort(key=sort_types)

            for next_type in next_types:
                header_str : str = get_activity_header_str(next_type)
                activity_plants : list[Plant] = next_activities[next_date][next_type]
                plant_links : list[str] = []
                for plant in activity_plants: 
                    plant_name = plant.info['plant_name']
                    plant_link : str = plant_link_template % (plant_details_out,get_html_name(plant_name),plant_name)
                    plant_links.append(plant_link)
                plant_links_str : str = ', '.join(plant_links)

                next_header : str = next_activity_type_template % (header_str,plant_links_str)
                date_activities.append(next_header)

            weekday_str : str = weekday_strs[next_date.weekday()]
            date_str : str = weekday_str + ', ' + next_date.strftime(date_format)

            next_activity_div : str = next_activity_template % (date_str, '<br/>'.join(date_activities))
            next_activities_strs.append(next_activity_div)

        return '\n'.join(next_activities_strs)

    def get_hall_of_fame(self) -> dict[str,str]:
        plant_link_template : str = '<div class="hall_of_fame_item">%s<br/><a href="%s">%s</a><br/>%s</div>'
        def get_plant_str(plant:Plant,winner_str:str,winning_stat:str):
            plant_name : str = plant.info['plant_name']
            plant_link : str = plant_details_out + '/' + get_html_name(plant_name)
            return plant_link_template % (winner_str,plant_link,plant_name,winning_stat) 

        plants_by_height : list[Plant] = self.manager.plants.copy()
        plants_by_height.sort(key=lambda x:x.current_height)

        tallest_plant  : Plant = plants_by_height[-1]
        tallest_height_str : str = str(tallest_plant.current_height) + 'cm'
        tallest_plant_str : str = get_plant_str(tallest_plant,'Tallest Plant',tallest_height_str)

        shortest_plant : Plant = plants_by_height[0]
        shortest_height_str : str = str(shortest_plant.current_width) + 'cm'
        shortest_plant_str : str = get_plant_str(shortest_plant,'Shortest Plant',shortest_height_str) 

        plants_by_width : list[Plant] = self.manager.plants.copy()
        plants_by_width.sort(key=lambda x:x.current_width)

        widest_plant : Plant = plants_by_width[-1]
        widest_width_str : str = str(widest_plant.current_width) + 'cm'
        widest_plant_str : str = get_plant_str(widest_plant,'Widest Plant',widest_width_str)
        thinnest_plant : Plant = plants_by_width[0]
        thinnest_width_str : str = str(thinnest_plant.current_width) + 'cm'
        thinnest_plant_str : str = get_plant_str(thinnest_plant,'Thinnest Plant',thinnest_width_str)

        plants_by_growth : list[Plant] = self.manager.plants.copy()
        plants_by_growth = list(filter(lambda x: len(x.growth)>2,plants_by_growth))

        plants_by_growth.sort(key=lambda x: x.get_growth_diff())
        fastest_plant : Plant = plants_by_growth[-1]
        fastest_growth_diff : str = '{:.2f}'.format(fastest_plant.get_growth_diff()) + 'cm/day'
        fastest_plant_str : str = get_plant_str(fastest_plant,'Fastest Growing Plant',fastest_growth_diff)
        slowest_plant : Plant = plants_by_growth[0]
        slowest_growth_diff : str = '{:.2f}'.format(slowest_plant.get_growth_diff()) + 'cm/day'
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

        next_activities_str : str = self.get_next_activities_str()
        
        autowatered_plants : list[Plant] = list(filter(lambda x:x.info['auto_water'],self.manager.plants))
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
                'autowatered_plants':autowater_str,
                'next_activities':next_activities_str,
                }

        index_dict = index_dict | hall_of_fame_dict
        index_html = self.index_template.render(index_dict)
        write_html(index_out,index_html)

    def render_all_species(self) -> None:
        self.render_species_overview()
        for plant in self.manager.species:
            self.render_species_details(plant)

    def render_all_plants(self) -> None:
        self.render_plant_overview()
        for plant in self.manager.plants:
            self.render_plant_details(plant)

    def render_all(self) -> None:
        self.render_all_species()
        self.render_all_plants()
        self.render_activity_log()
        self.render_gallery()
        self.render_graveyard()
        self.render_index()
