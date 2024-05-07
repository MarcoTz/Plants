import os 
import csv 


def write_csv(csv_items:list[dict[str,str]],out_dir:str,out_file_name:str,overwrite:bool=False) -> None:
    out_file_path : str = os.path.join(out_dir,out_file_name)
    write_mode : str = 'w' if overwrite else 'a' 
    out_file = open(out_file_path,write_mode)
    csv_fields : list[str] = list(csv_items[0].keys())
    
    writer : csv.DictWriter = csv.DictWriter(out_file,delimiter=';',fieldnames=csv_fields)
    if overwrite:
        writer.writeheader()

    for csv_item in csv_items:
        writer.writerow(csv_item)
    print('Wrote log to %s' % out_file_name)
