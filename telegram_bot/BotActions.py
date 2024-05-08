from enum import Enum 
import datetime 
from common.constants import date_format
from update.add_activity import add_activities
from update.add_growth import add_growth
from update.add_plant import add_plant
from update.add_species import add_species

class BotAction(Enum):
   IDLE         = 1 
   NEW_GROWTH   = 2
   NEW_ACTIVITY = 3
   NEW_PLANT    = 4
   NEW_SPECIES  = 5

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
            ('Please enter current location',                                   'str'),
            ('Please enter plant origin',                                       'str'),
            ('Please enter obtained date (dd.mm.yyyy)',                         'date'),
            ('Please enter notes (separate by comma, write "Done" for no note', 'str')
            ],
        BotAction.NEW_SPECIES : [
            ('Please enter species common name',                                             'str'),
            ('Please enter plant scientific name',                                           'str'),
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
            ('current_location',        False,False),
            ('origin',                  False,False),
            ('obtained',                False,False),
            ('plant_notes',             True,True)
            ],
        BotAction.NEW_SPECIES : [
            ('name',                    False,False),
            ('scientific_name',         False,False),
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
            ('fertilizing_notes',       True,True),
            ('avg_fertilizing_days',    False,False),
            ('pruning_notes',           True,True),
            ('companions',              True,True),
            ('additional_notes',        True,True),
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

def validate_msg(msg:str,ty:str) -> str | None:
    if ty == 'float' and not ensure_float(msg):
        return 'Could not parse number, please try again'
    if ty == 'date' and not ensure_datetime(msg):
        return 'Could not parse date, please try again'
    if ty == 'int' and not ensure_int(msg):
        return 'Could not parse number, please try again'
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
