from enum import Enum 
import datetime 
from common.constants           import date_format
from update.add_activity        import add_activities
from update.add_growth          import add_growth
from update.add_plant           import add_plant
from update.add_species         import add_species
from update.move_to_graveyard   import move_to_graveyard
from update.update_plant        import update_plant_dict
from update.update_species      import update_species_dict

class BotAction(Enum):
   IDLE         = 1 
   NEW_GROWTH   = 2
   NEW_ACTIVITY = 3
   NEW_PLANT    = 4
   NEW_SPECIES  = 5
   MOVE_GRAVEYARD = 6
   UPDATE_SPECIES = 7
   UPDATE_PLANT = 8
   WATER_TODAY = 9
   FERTILIZE_TODAY = 10

plant_update_fields : list[str] = ['plant_name','species_name','origin','obtained','plant_notes','current_location','plant_health']

species_update_fields : list[str] = [
        "name",
        "scientific_name",
        "species_type",
        "sunlight_requirements",
        "temperature_min",
        "temperature_max",
        "optimal_temperature_min",
        "optimal_temperature_max"   ,
         "plant_distance_cm",
         "ph_min",
         "ph_max",
         "watering_notes",
         "avg_watering_days",
         "fertilizing_notes",
         "avg_fertilizing_days",
         "pruning_notes",
         "companions",
         "additional_notes"]

action_input_information : dict[BotAction,list[tuple[str,str]]] = {
        BotAction.IDLE : [],
        BotAction.NEW_GROWTH : [
             ('Please enter date of update (dd.mm.yyyy)',    'date'),
             ('Please enter plant name',                     'str'),
             ('Please enter height (cm)',                    'float'),
             ('Please enter width (cm)',                     'float'),
             ('Please enter note, write "Done" for no note', 'str'),
        ],
        BotAction.NEW_ACTIVITY : [
            ('Please enter date of update (dd.mm.yyyy)',         'date'), 
            ('Please enter activity',                            'str'),
            ('Please enter affected plants (separate by comma)', 'str'),
            ('Please enter note (write "Done" for no note)',     'str')
            ],
        BotAction.NEW_PLANT : [
            ('Please enter plant name',                                         'str'), 
            ('Please enter species name',                                       'str'),
            ('Please enter current height (cm)',                                'float'),
            ('Please enter current width (cm)',                                 'float'),
            ('Please enter current health (0-5)',                               'int'),
            ('Please enter current location',                                   'str'),
            ('Please enter plant origin',                                       'str'),
            ('Please enter obtained date (dd.mm.yyyy)',                         'date'),
            ('Please enter notes (separate by comma, write "Done" for no note', 'str')
            ],
        BotAction.NEW_SPECIES : [
            ('Please enter species common name',                                             'str'),
            ('Please enter plant scientific name',                                           'str'),
            ('Please enter plant type (cactus/nightshade/etc)',                              'str'),
            ('Please enter sunlight requirements (direct/indirect/shade)',                   'str'),
            ('Please enter minimal survivable temperature (C)',                              'float'),
            ('Please enter maximal survivable temperature (C)',                              'float'),
            ('Please enter minimal optimal temperature (C)',                                 'float'),
            ('Please enter maximal optimal temperature (C)',                                 'float'),
            ('Please enter minimal distance to next plant (cm)',                             'float'),
            ('Please enter minimal pH value',                                                'float'),
            ('Please enter maximal pH value',                                                'float'),
            ('Please enter average days between waterings',                                  'int'),
            ('Please enter watering notes (separate by comma, write "Done" for no notes',    'str'),
            ('Please enter average days between fertilizings',                               'int'),
            ('Please enter fertilizing notes (separate by comma, write "Done" for no notes', 'str'),
            ('Please enter pruning notes (separate by comma, write "Done" for no notes',     'str'),
            ('Please enter companion plants (separate by comma,write "Done" for no notes',   'str'),
            ('Please enter additional notes (separate by comma, write "Done" for no notes',  'str')
            ],
        BotAction.MOVE_GRAVEYARD : [
            ('PLease enter plant name',                 'str'),
            ('Please enter died date (dd.mm.yyyy)',     'date'),
            ('Please enter diead reason',               'str')
            ],
        BotAction.UPDATE_SPECIES : [
            ('Please enter species name',                       'str'),
            ('Please enter field to update (%s)' % ', '.join(species_update_fields),'species_update_field'),
            ('Please enter new value (notes will be appended)', 'str')
            ],
        BotAction.UPDATE_PLANT : [
            ('Please enter plant name',                         'str'),
            ('Please enter field to update (%s)' % ','.join(plant_update_fields),'plant_update_field'),
            ('Please enter new value (notes will be appended)', 'str')
            ],
        BotAction.WATER_TODAY : [
            ('Please enter watered plants (separate by comma)', 'str')
            ],
        BotAction.FERTILIZE_TODAY : [
            ('Please enter fertilized plants (separate by comma)','str')
            ]
    }

def get_ret_msg(action:BotAction, ind:int) -> str:
    try: 
        return action_input_information[action][ind+1][0]
    except IndexError:
        return ''

def get_input_ty(action:BotAction, ind:int) -> str:
    try: 
        return action_input_information[action][ind][1]
    except IndexError:
        return ''

# json_key, split_on_comma, nothing_if_Done
action_output_information : dict[BotAction,list[tuple[str,bool,bool]]] = {
        BotAction.IDLE : 
            [],
        BotAction.NEW_GROWTH : [
             ('growth_date',            False,False),
             ('growth_plant',           False,False),
             ('growth_height',          False,False),
             ('growth_width',           False,False),
             ('growth_note',            False,True)
        ],
        BotAction.NEW_ACTIVITY : [
            ('log_date',                False,False), 
            ('log_activity',            False,False),
            ('log_plants',              False,False),
            ('log_note',                False,True)
            ],
        BotAction.NEW_PLANT : [
            ('plant_name',              False,False), 
            ('species_name' ,           False,False),
            ('current_height',          False,False),
            ('current_width',           False,False),
            ('plant_health',            False,False),
            ('current_location',        False,False),
            ('origin',                  False,False),
            ('obtained',                False,False),
            ('plant_notes',             True,True)
            ],
        BotAction.NEW_SPECIES : [
            ('name',                    False,False),
            ('scientific_name',         False,False),
            ('species_type',            False,False),
            ('sunlight_requirements',   False,False),
            ('temperature_min',         False,False),
            ('temperature_max',         False,False),
            ('optimal_temperature_min', False,False),
            ('optimal_temperature_max', False,False),
            ('plant_distance_cm',       False,False),
            ('ph_min',                  False,False),
            ('ph_max',                  False,False),
            ('avg_watering_days',       False,False),   
            ('watering_notes',          True,True),
            ('avg_fertilizing_days',    False,False),
            ('fertilizing_notes',       True,True),
            ('pruning_notes',           True,True),
            ('companions',              True,True),
            ('additional_notes',        True,True),
            ],
        BotAction.MOVE_GRAVEYARD : [
            ('graveyard_plant',         False,False),
            ('graveyard_died_date',     False,False),
            ('graveyard_reason',        False,False)
            ],
        BotAction.UPDATE_PLANT : [
            ('plant_name',False,False),
            ('update_key',False,False),
            ('update_value',False,False)
            ],
        BotAction.UPDATE_SPECIES : [
            ('species_name',False,False),
            ('update_key',False,False),
            ('update_value',False,False)
            ],
        BotAction.WATER_TODAY: [
            ('log_plants',False,False)
            ],
        BotAction.FERTILIZE_TODAY : [
            ('log_plants',False,False)
            ]
}

def get_action_writer(action:BotAction):
    match action:
        case BotAction.IDLE:
            return (lambda _: None)
        case BotAction.NEW_GROWTH:
            return (lambda x: add_growth([x]))
        case BotAction.NEW_ACTIVITY:
            return (lambda x: add_activities([x]))
        case BotAction.NEW_PLANT:
            return add_plant
        case BotAction.NEW_SPECIES:
            return add_species
        case BotAction.MOVE_GRAVEYARD:
            return move_to_graveyard
        case BotAction.UPDATE_PLANT:
            return update_plant_dict
        case BotAction.UPDATE_SPECIES:
            return update_species_dict
        case BotAction.WATER_TODAY:
            return (lambda x: activity_shortcut('Watering',x))
        case BotAction.FERTILIZE_TODAY: 
            return (lambda x: activity_shortcut('Fertilizing',x))


def activity_shortcut(activity:str, plants_dict):
    activity_dict : dict[str,str] = {}
    activity_dict['log_date'] = datetime.datetime.now().strftime(date_format)
    activity_dict['log_activity'] = activity 
    activity_dict['log_plants'] = plants_dict['log_plants'] 
    activity_dict['log_note'] = ''
    add_activities([activity_dict])


def get_len_input(action:BotAction) -> int:
    return len(action_input_information[action])

def explain_BotAction(action:BotAction) -> str:
    match action:
        case BotAction.IDLE:
            return 'Currently there is no command running'
        case BotAction.NEW_GROWTH:
            return 'added new growth item'
        case BotAction.NEW_ACTIVITY:
            return 'added new activity'
        case BotAction.NEW_PLANT:
            return 'added new plant'
        case BotAction.NEW_SPECIES:
            return 'added new species'
        case BotAction.MOVE_GRAVEYARD:
            return 'moved to graveyard'
        case BotAction.UPDATE_PLANT:
            return 'updated plant'
        case BotAction.UPDATE_SPECIES:
            return 'updated species'
        case BotAction.WATER_TODAY:
            return 'watered plants'
        case BotAction.FERTILIZE_TODAY:
            return 'fertilized plants'

def validate_msg(msg:str,ty:str) -> str | None:
    if ty == 'float' and not ensure_float(msg):
        return 'Could not parse number, please try again'
    if ty == 'date' and not ensure_datetime(msg):
        return 'Could not parse date, please try again'
    if ty == 'int' and not ensure_int(msg):
        return 'Could not parse number, please try again'
    if ty == 'plant_update_field' and not msg in plant_update_fields:
        return 'Not a valid field, please try again'
    if ty == 'species_update_field' and not msg in species_update_fields:
        return 'Not a valid field, please try again'
    return None

def ensure_datetime(msg:str) -> bool:
    try:
        datetime.datetime.strptime(msg,date_format)
        return True
    except ValueError:
        return False

def ensure_int(msg:str) -> bool:
    try:
        int(msg)
        return True
    except ValueError:
        return False

def ensure_float(msg:str) -> bool:
    try:
        float(msg)
        return True
    except ValueError:
        return False
