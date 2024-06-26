function filter_plants() {
  plant_name_filter   = document.getElementById('plant_search_name').value.toLowerCase();
  species_name_filter = document.getElementById('plant_search_species').value.toLowerCase();
  max_temp_updown     = document.getElementById('max_temp_updown').value;
  max_temp            = document.getElementById('plant_search_max_temp').value;
  min_temp_updown     = document.getElementById('min_temp_updown').value;
  min_temp            = document.getElementById('plant_search_min_temp').value;
  
  plant_items = document.getElementsByClassName('plant_list_item')
  for(var i=0; i<plant_items.length; i++){
    plant_info = {}
    for(var j=0;j<plant_items[i].children.length;j++){
      current_child = plant_items[i].children[j]
      if(current_child.className == 'plant_link') plant_info['name'] = current_child.textContent.toLowerCase();
      if(current_child.className == 'species_link') plant_info['species'] = current_child.textContent.toLowerCase();
      if(current_child.className == 'temp_max') plant_info['temp_max'] = Number(current_child.textContent);
      if(current_child.className == 'temp_min') plant_info['temp_min'] = Number(current_child.textContent);
    }
    new_visibility = 'block';

    if(!(plant_info['name'].includes(plant_name_filter)))new_visibility = 'none';
    if(!(plant_info['species'].includes(species_name_filter))) new_visibility = 'none';
    if(max_temp != '') {
      max_temp = Number(max_temp)
      if(max_temp_updown == '+' && plant_info['temp_max'] < max_temp) new_visibility = 'none';
      if(max_temp_updown == '-' && plant_info['temp_max'] > max_temp) new_visibility = 'none';
    }
    if(min_temp != ''){
      if(min_temp_updown == '+' && plant_info['temp_min'] < min_temp) new_visibility = 'none';
      if(min_temp_updown == '-' && plant_info['temp_min'] > min_temp) new_visibility = 'none';
    }
    


    plant_items[i].style.display=new_visibility;
  }
}
