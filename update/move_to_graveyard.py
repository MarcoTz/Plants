import datetime
import os

def get_died_date() -> str:
    log_date    : str       = input('Enter died date (dd.mm.yyyy), leave blank for today: ').strip()
    if log_date == '':
        return datetime.datetime.now().strftime('%d.%m.%Y')
    try:
        datetime.datetime.strptime(log_date,'%d.%m.%Y')
        return log_date
    except ValueError:
        print('Could not parse date, please try again')
        return get_died_date()

def move_to_graveyard() -> dict[str,str]:
    print('-- Move plant to graveyard --')

    graveyard_plant  : str = input('Enter plant name: ')
    died_date        : str = get_died_date()
    graveyard_reason : str = input('Enter reason of death: ')

    return {
            'graveyard_plant':graveyard_plant,
            'graveyard_died_date' : died_date,
            'graveyard_reason': graveyard_reason
    }

def remove_plant_images(plant_name:str,img_dir:str,img_small_dir:str) -> None:
    plant_imgs       : list[str] = os.listdir(img_dir)
    plant_imgs = list(map(lambda x: os.path.join(img_dir,x),plant_imgs))
    plant_imgs_small : list[str] = os.listdir(img_small_dir)
    plant_imgs_small = list(map(lambda x: os.path.join(img_small_dir,x),plant_imgs_small))
    all_imgs : list[str] = plant_imgs
    all_imgs.extend(plant_imgs_small)
    for img_path in all_imgs:
        img_name_split : list[str] = img_path.split('_')
        if plant_name in img_name_split[0] and len(img_name_split) == 2:
            os.remove(img_path)
