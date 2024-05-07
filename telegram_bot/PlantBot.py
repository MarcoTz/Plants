from file_io.load_json import load_bot_config
from common.constants import date_format 
from update.add_growth import add_growth
from update.add_activity import add_activities
from update.add_plant import add_plant
from update.add_species import add_species

from telegram.ext import Application,ApplicationBuilder,CommandHandler,ContextTypes,MessageHandler
from telegram import Update,Chat,Message
from enum import Enum
import datetime

class BotAction(Enum):
   IDLE = 1 
   NEW_GROWTH = 2
   NEW_ACTIVITY = 3
   NEW_PLANT = 4

def explain_BotAction(action:BotAction) -> str:
    match action:
        case BotAction.IDLE:
            return 'Currently there is no command running'
        case BotAction.NEW_GROWTH:
            return 'adding new growth item'
        case BotAction.NEW_ACTIVITY:
            return 'adding new activity'
        case BotAction.NEW_PLANT:
            return 'adding new plant'

def ensure_datetime(msg:str) -> bool:
    try:
        datetime.datetime.strptime(msg,date_format)
        return True
    except ValueError:
        return False

def ensure_float(msg:str) -> bool:
    try:
        float(msg)
        return True
    except ValueError:
        return False

class PlantBot: 
    api_key         : str
    white_list      : list[int]
    application     : Application
    handlers        : list[CommandHandler | MessageHandler]
    current_action  : BotAction
    current_inputs  : list[str]

    def __init__(self):
        (api_key,white_list) = load_bot_config()
        self.api_key = api_key
        self.white_list = white_list
        self.handlers = []
        self.current_action = BotAction.IDLE
        self.application = ApplicationBuilder().token(api_key).build()

        new_growth_handler   : CommandHandler = CommandHandler('new_growth', self.new_growth)
        new_activity_handler : CommandHandler = CommandHandler('new_activity',self.new_activity)
        new_plant_handler    : CommandHandler = CommandHandler('new_plant',self.new_plant)
        message_handler      : MessageHandler = MessageHandler(None,self.handle_message)

        self.handlers.append(new_growth_handler)
        self.handlers.append(new_activity_handler)
        self.handlers.append(new_plant_handler)
        self.handlers.append(message_handler)

        self.application.add_handler(new_growth_handler)
        self.application.add_handler(new_activity_handler)
        self.application.add_handler(new_plant_handler)
        self.application.add_handler(message_handler)

    def run(self):
        self.application.run_polling()

    async def guard_access(self,update:Update,context:ContextTypes.DEFAULT_TYPE) -> bool:
        chat_id : int = self.get_chat_id(update)
        can_access:bool = chat_id in self.white_list

        if not can_access:
            return_msg : str = 'This bot is not for you'
            await context.bot.send_message(chat_id=chat_id,text=return_msg)

        return can_access


    def get_chat_id(self,update:Update) -> int:
        if update.effective_chat is None:
            return -1

        chat : Chat = update.effective_chat

        if chat.id is None:
            return -1

        return chat.id

    def get_message_text(self,update:Update) -> str:
        if update.effective_message is None:
            return ''
        
        message : Message = update.effective_message

        if message.text is None:
            return ''

        return message.text


    async def new_growth(self,update,context) -> None: 
        if await self.guard_access(update,context):
            self.current_action = BotAction.NEW_GROWTH
            self.current_inputs = []
            chat_id : int = self.get_chat_id(update)
            return_msg : str = 'Please enter date of update (dd.mm.yyyy)'
            await context.bot.send_message(chat_id=chat_id,text=return_msg)

    async def new_activity(self,update,context) -> None:
        if await self.guard_access(update,context):
            self.current_action = BotAction.NEW_ACTIVITY
            self.current_inputs = [] 
            chat_id : int = self.get_chat_id(update)
            return_msg : str = 'Please enter date of update (dd.mm.yyyy)'
            await context.bot.send_message(chat_id=chat_id,text=return_msg)

    async def new_plant(self,update,context) -> None:
        if await self.guard_access(update,context):
            self.current_action = BotAction.NEW_PLANT
            self.current_inputs = []
            chat_id : int = self.get_chat_id(update)
            return_msg : str = 'Please enter plant name'
            await context.bot.send_message(chat_id=chat_id,text=return_msg)

    async def handle_message(self,update,context) -> None:
        if not await self.guard_access(update,context):
            return
       
        chat_id : int = self.get_chat_id(update)
        message_text : str = self.get_message_text(update)
        match self.current_action:
            case BotAction.IDLE:
                return_msg : str = 'There no command running currently'
                await context.bot.send_message(chat_id=chat_id,text=return_msg)
                return
            case BotAction.NEW_GROWTH:
                return_msg : str = self.handle_growth_input(message_text)
                await context.bot.send_message(chat_id=chat_id,text=return_msg)
                return
            case BotAction.NEW_ACTIVITY:
                return_msg : str = self.handle_activity_input(message_text)
                await context.bot.send_message(chat_id=chat_id,text=return_msg)
            case BotAction.NEW_PLANT:
                return_msg : str = self.handle_plant_input(message_text)
                await context.bot.send_message(chat_id=chat_id,text=return_msg)

    def handle_input_generic(self,max_inputs:int,msg:str,return_msg:str,is_float:bool=False,is_date:bool=False) -> str:
        if is_float and not ensure_float(msg):
            return 'Could not parse number, please try again'

        if is_date and not ensure_datetime(msg):
            return 'Could not parse date, please try again'

        self.current_inputs.append(msg)

        if len(self.current_inputs) == max_inputs:
            action : BotAction = self.current_action
            self.write_output()
            return 'Sucessfully finished %' % explain_BotAction(action)
        elif len(self.current_inputs) < max_inputs:
            return return_msg
        else:
            self.current_action = BotAction.IDLE
            self.current_inputs = []
            return 'Something went wrong, please start again'

    def write_output(self) -> None:
        match self.current_action:
            case BotAction.IDLE:
                return 
            case BotAction.NEW_GROWTH:
                self.write_growth()
            case BotAction.NEW_ACTIVITY:
                self.write_activity()
            case BotAction.NEW_PLANT:
                self.write_plant()

        self.current_action = BotAction.IDLE
        self.current_inputs = []
        return 


    def handle_growth_input(self,msg:str) -> str:
        growth_return_msgs : list[str] = [
                'Please enter plant name',
                'Please enter height (cm)',
                'Please enter width (cm)',
                'Please enter note (enter "Done" to not add note)',
                'Successfully added growth item']
        when_float : list[bool] = [False,False,True,True,False]
        current_ind : int = len(self.current_inputs)
        max_ind : int = len(growth_return_msgs)
        curr_return : str = growth_return_msgs[current_ind]
        is_float : bool = when_float[current_ind]
        return self.handle_input_generic(max_ind,msg,curr_return,is_float=is_float)

    def handle_activity_input(self,msg:str) -> str:
        activity_return_msgs : list[str] = [
                'Please enter activity',
                'Please enter affected plants (separate by comma)',
                'Please enter note ("Done" for no note)',
                'Successfully added activity item']
        when_date : list[bool] = [True,False,False,False]
        current_ind : int = len(self.current_inputs)
        max_ind : int = len(activity_return_msgs)
        curr_return : str = activity_return_msgs[current_ind]
        is_date : bool = when_date[current_ind]
        return self.handle_input_generic(max_ind,msg,curr_return,is_date=is_date)

    def handle_plant_input(self,msg:str) -> str:
        plant_return_msgs : list[str] = [
                'Please enter plant species',
                'Please enter current height (cm)',
                'Please enter current width (cm)',
                'Please enter current location',
                'Please enter plant origin',
                'Please enter obtained date (dd.mm.yyyy)',
                'Successfully added plant']
        when_date : list[bool] = [False,False,False,False,False,True,False]
        when_float : list[bool] = [False,True,True,False,False,False,False]
        current_ind : int = len(self.current_inputs)
        max_ind : int =len(plant_return_msgs)
        curr_return : str = plant_return_msgs[current_ind]
        is_date : bool = when_date[current_ind]
        is_float: bool = when_float[current_ind]
        return self.handle_input_generic(max_ind,msg,curr_return,is_float=is_float,is_date=is_date)

    def handle_species_input(self,msg:str) -> str:
        species_return_msgs : list[str] = [
                'Please enter scientific name',
                'Please enter sunlight requirements (direct/indirect/shade)',
                'Please enter minimal survivable temperature (C)',
                'Please enter maximal survivable temperature (C)',
                'Please enter minimal comfortable temperature (C)',
                'Please enter maxmimal comfortable temperature (C)',
                'Please enter minimal distance to next plant (cm)',
                'Please enter minimal pH value',
                'Please enter maximal pH value',
                'Please enter watering notes (separate by comma)',
                'Please enter fertilizing notes (separate by comma)',
                'Please enter pruning notes (separate by comma)',
                'Please enter companion plants (separate by comma)',
                'Please enter additional notes (separate by comma, write "Done" for no notes',
                'Successfully added Species']
        when_float : list[bool] = [False,False,False,True,True,True,True,True,True,True,False,False,False,False,False]
        curr_ind : int = len(self.current_inputs)
        max_ind : int = len(species_return_msgs)
        curr_return : str = species_return_msgs[curr_ind]
        is_float : bool = when_float[curr_ind]
        return self.handle_input_generic(max_ind,msg,curr_return,is_float=is_float)


    def write_growth(self) -> None:
        growth_note : str = self.current_inputs[4].strip()
        growth_item : dict[str,str] = {
            'growth_date'   : self.current_inputs[0].strip(),
            'growth_plant'  : self.current_inputs[1].strip(),
            'growth_height' : self.current_inputs[2].strip(),
            'growth_width'  : self.current_inputs[3].strip(),
            'growth_note'   : growth_note if growth_note != 'Done' else '' 
            }
        add_growth([growth_item])
    
    def write_activity(self) -> None:
        activity_note : str = self.current_inputs[3].strip()
        activity_item : dict[str,str] = {
            'log_date':     self.current_inputs[0].strip(),
            'log_activity': self.current_inputs[1].strip(),
            'log_plants':   self.current_inputs[2].strip(),
            'log_note':     activity_note if activity_note != 'Done' else ''
         }
        add_activities([activity_item])

    def write_plant(self) -> None:
        plant_notes : list[str] = self.current_inputs[7].split(',')
        plant_notes = list(map(lambda x:x.strip(),plant_notes))
        plant = {
            'plant_name'       : self.current_inputs[0].strip(),
            'species_name'     : self.current_inputs[1].strip(),
            'current_height'   : self.current_inputs[2].strip(),
            'current_width'    : self.current_inputs[3].strip(),
            'current_location' : self.current_inputs[4].strip(),
            'origin'           : self.current_inputs[5].strip(),
            'obtained'         : self.current_inputs[6].strip(),
            'plant_notes'      : plant_notes if plant_notes != ['Done'] else ['']
            }
        add_plant(plant)
    
    def write_species(self) -> None:
        watering_notes : list[str] = self.current_inputs[12].split(',')
        watering_notes = list(map(lambda x:x.strip(),watering_notes)) 

        fertilizing_notes : list[str] = self.current_inputs[12].split(',')
        fertilizing_notes = list(map(lambda x:x.strip(),fertilizing_notes)) 

        pruning_notes : list[str] = self.current_inputs[12].split(',')
        pruning_notes = list(map(lambda x:x.strip(),pruning_notes)) 

        companions : list[str] = self.current_inputs[13].split(',')
        companions = list(map(lambda x:x.strip(),companions))

        species_notes : list[str] = self.current_inputs[14].split(',')
        species_notes = list(map(lambda x:x.strip(),species_notes))

        species = {
            "name"                      : self.current_inputs[0].strip(),
            "scientific_name"           : self.current_inputs[1].strip(),
            "sunlight_requirements"     : self.current_inputs[2].strip(),
            "temperature_min"           : self.current_inputs[3].strip(),
            "temperature_max"           : self.current_inputs[4].strip(),
            "optimal_temperature_min"   : self.current_inputs[5].strip(),
            "optimal_temperature_max"   : self.current_inputs[6].strip(),
            "plant_distance_cm"         : self.current_inputs[7].strip(),
            "ph_min"                    : self.current_inputs[8].strip(),
            "ph_max"                    : self.current_inputs[9].strip(),
            "watering_notes"            : watering_notes,
            "fertilizing_notes"         : fertilizing_notes,
            "pruning_notes"             : pruning_notes,
            "companions"                : companions,
            "additional_notes"          : species_notes if species_notes != ['Done'] else ['']
            }

        add_species(species)

