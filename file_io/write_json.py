import os 
import json 

def write_json(json_dict:dict[str,str],out_dir:str,out_file_name:str) -> None:
    out_file_path = os.path.join(out_dir,out_file_name)
    out_file = open(out_file_path,'w+')

    out_file.write(json.dumps(json_dict))
    out_file.close()
    print('Saved %s'%out_file_name)
