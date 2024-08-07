date_format = '%d.%m.%Y'
date_format_images = '%d%m%Y'

template_dir                   : str = 'html_templates'
species_overview_template_name : str = 'species_overview.html'
species_details_template_name  : str = 'species_details.html'
plant_overview_template_name   : str = 'plant_overview.html'
plant_details_template_name    : str = 'plant_details.html'
index_template_name            : str = 'index.html'
activity_log_template_name     : str = 'activity_logs.html'
header_template_name           : str = 'header.html'
footer_template_name           : str = 'footer.html'
graveyard_template_name        : str = 'graveyard.html'
gallery_template_name          : str = 'gallery.html'
image_view_template_name       : str = 'image_view.html'

out_dir               : str = 'html_out'
species_details_out   : str = 'plant_species'
plant_details_out     : str = 'plants'
species_overview_out  : str = 'species_overview.html'
plant_overview_out    : str = 'plant_overview.html'
index_out             : str = 'index.html'
activity_log_out      : str = 'activity_logs.html'
graveyard_out         : str = 'graveyard.html'
gallery_out           : str = 'gallery.html'

img_dir         : str = 'img'
img_species_dir : str = 'species'
img_plants_dir  : str = 'plants'
img_small_dir   : str = 'small'

species_dir             : str = 'PlantSpecies'
plants_dir              : str = 'Plants'
log_dir                 : str = 'Logs'
activity_log_file_name  : str = 'Activities.csv'
growth_log_file_name    : str = 'Growth.csv'
graveyard_file_name     : str = 'Graveyard.csv'

bot_dir                 : str = 'telegram_bot'
bot_config_name         : str = 'conf.json'

weekday_strs : list[str] = [
        'Mon',
        'Tue',
        'Wed',
        'Thu',
        'Fri',
        'Sat',
        'Sun']

water_img           : str = '🌊'
fertilize_img       : str = '💩'
growth_img          : str = '📏'
is_autowatered_img  : str = '✅'
not_autowatered_img : str = '❌'

def get_activity_header_str(activity_type:str) -> str:
    match activity_type:
        case 'Watering':
            ty_imgs : tuple[str,str] = (water_img,water_img)
        case 'Fertilizing':
            ty_imgs : tuple[str,str] = (fertilize_img,fertilize_img)
        case 'Watering + Fertilizing':
            ty_imgs : tuple[str,str] = (water_img,fertilize_img)
        case _:
            ty_imgs : tuple[str,str] = ('','')

    return '%s %s %s' % (ty_imgs[0],activity_type,ty_imgs[1])

