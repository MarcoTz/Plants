from file_io.load_json import load_bot_config
from common.constants import * 
from telegram_bot.BotActions import * 

from telegram.ext import Application,ApplicationBuilder,CommandHandler,ContextTypes,MessageHandler,filters
from telegram import Update,File
import os 

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
        self.current_inputs = []
        self.application = ApplicationBuilder().token(api_key).build()

        new_growth_handler   : CommandHandler = CommandHandler('new_growth', self.new_growth)
        new_activity_handler : CommandHandler = CommandHandler('new_activity',self.new_activity)
        new_plant_handler    : CommandHandler = CommandHandler('new_plant',self.new_plant)
        help_handler         : CommandHandler = CommandHandler('help',self.get_help)
        show_inputs_handler  : CommandHandler = CommandHandler('show_inputs',self.show_inputs)
        abort_handler        : CommandHandler = CommandHandler('abort',self.abort_action)
        message_handler      : MessageHandler = MessageHandler(filters.TEXT,self.handle_message)
        photo_handler        : MessageHandler = MessageHandler(filters.PHOTO,self.handle_photo)

        self.handlers.append(new_growth_handler)
        self.handlers.append(new_activity_handler)
        self.handlers.append(new_plant_handler)
        self.handlers.append(help_handler)
        self.handlers.append(show_inputs_handler)
        self.handlers.append(abort_handler)
        self.handlers.append(message_handler)
        self.handlers.append(photo_handler)

        self.application.add_handler(new_growth_handler)
        self.application.add_handler(new_activity_handler)
        self.application.add_handler(new_plant_handler)
        self.application.add_handler(help_handler)
        self.application.add_handler(show_inputs_handler)
        self.application.add_handler(abort_handler)
        self.application.add_handler(message_handler)
        self.application.add_handler(photo_handler)

    def run(self):
        self.application.run_polling()

    async def send_message(self,update:Update,context:ContextTypes.DEFAULT_TYPE, msg:str) -> None: 
        if update.effective_chat is None or update.effective_chat.id is None:
            return 
        chat_id : int = update.effective_chat.id
        await context.bot.send_message(chat_id=chat_id,text=msg)

    def get_message_text(self,update:Update) -> str:
        if update.effective_message is None or update.effective_message.text is None:
            return ''
        else:
            return update.effective_message.text

    async def guard_access(self,update:Update,context:ContextTypes.DEFAULT_TYPE) -> bool:
        if update.effective_chat is None or update.effective_chat.id is None:
            return False 

        chat_id : int = update.effective_chat.id 
        can_access:bool = chat_id in self.white_list

        if not can_access:
            return_msg : str = 'This bot is not for you'
            await self.send_message(update,context,return_msg)

        return can_access

    async def abort_action(self,update:Update,context:ContextTypes.DEFAULT_TYPE) -> None:
        if not await self.guard_access(update,context):
            return 
        previous_action : BotAction = self.current_action
        self.current_action = BotAction.IDLE
        self.current_inputs = []
        await self.send_message(update,context,'Aborted %s' % previous_action)

    async def show_inputs(self,update:Update,context:ContextTypes.DEFAULT_TYPE)->None:
        if not await self.guard_access(update,context):
            return 
        current_inputs_str : str = '\n'.join(self.current_inputs) 
        current_inputs_str = 'No inputs' if current_inputs_str == '' else current_inputs_str
        await self.send_message(update,context,current_inputs_str)


    async def get_help(self,update,context) -> None:
        if not await self.guard_access(update,context):
            return

        help_message : str = '''
        Possible commands:
        /new_growth : add new growth update
        /new_activity: add new activity udpate
        /new_plant: add new plant
        /new_species: add new plant species
        /abort : abort current action
        /show_inputs : show inputs of current action
        '''

        await self.send_message(update,context,help_message)

    async def new_action(self,action:BotAction, update:Update,context:ContextTypes.DEFAULT_TYPE) -> None: 
        if not await self.guard_access(update,context):
            return
        self.current_action = action
        self.current_inputs = [] 
        message_text : str = self.get_message_text(update)
        ret_msg : str = self.handle_input(message_text)
        await self.send_message(update,context,ret_msg)

    async def new_growth(self,update,context) -> None: 
        await self.new_action(BotAction.NEW_GROWTH,update,context)

    async def new_activity(self,update,context) -> None:
        await self.new_action(BotAction.NEW_ACTIVITY,update,context)

    async def new_plant(self,update,context) -> None:
        await self.new_action(BotAction.NEW_PLANT,update,context)

    async def handle_message(self,update,context) -> None:
        if not await self.guard_access(update,context):
            return  
        message_text : str = self.get_message_text(update)
        ret_msg : str = self.handle_input(message_text)
        await self.send_message(update,context,ret_msg)

    async def handle_photo(self,update,context) -> None:
        if not await self.guard_access(update,context):
            return
        if update.effective_message.caption is None:
            await self.send_message(update,context,'Please provited plant name as caption')
            return 
        
        file_name_template = '%s_%s.jpg'
        current_date : str = datetime.datetime.now().strftime(date_format_images)
        plant_name : str = update.effective_message.caption.strip().replace(' ','')
        new_file_name : str = file_name_template % (plant_name,current_date)
        photo : str = update.effective_message.photo[-1].file_id
        file  :  File = await context.bot.get_file(photo)
        out_path : str = os.path.join(out_dir,img_dir,img_plants_dir,new_file_name)
        await file.download_to_drive(out_path)
        await self.send_message(update,context,'Saved new image as %s' % new_file_name)

    def handle_input(self,msg:str) -> str:
        if msg.startswith('/'):
            return action_input_information[self.current_action][0][0]

        current_len : int  = len(self.current_inputs)
        needed_len  : int  = get_len_input(self.current_action)
        ret_msg : str = get_ret_msg(self.current_action,current_len) 
        ty : str = get_input_ty(self.current_action, current_len)
        
        if current_len >= needed_len:
            self.current_inputs = [] 
            self.current_action = BotAction.IDLE
            return 'Something went wrong, please start again'
        
        verified : str | None = validate_msg(msg,ty)
        if verified is not None:
            return verified 

        self.current_inputs.append(msg)
        if current_len + 1 == needed_len:
            out_ret = self.write_output()
            if out_ret is None:
                explanation : str = explain_BotAction(self.current_action)
                self.current_action = BotAction.IDLE
                self.current_inputs = []
                return 'Successfully %s' % explanation 
            else: 
                return out_ret

        return ret_msg

    def write_output(self) -> None | str:
        out_dict = {}
        num_fields : int= get_len_input(self.current_action)
        current_action_info = action_output_information[self.current_action]
        try:
            for i in range(num_fields):
                (dict_key,to_split,remove_done) = current_action_info[i] 
                out_val = self.current_inputs[i].strip()
                out_val = '' if remove_done and out_val=='Done' else out_val
                out_val = list(map(lambda x: x.strip(),out_val.split(', '))) if to_split else out_val
                out_dict[dict_key] = out_val
            write_fun = get_action_writer(self.current_action)
            write_fun(out_dict)
        except IndexError:
            return 'Something went wrong. please start again'
