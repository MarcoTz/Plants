function setup_img_events() {
  img_elements = document.getElementsByTagName('img')
  for(var i=0;i<img_elements.length;i++){
    img_src = img_elements[i].src
    if(img_src.includes('small')){ 
      img_elements[i].style.cursor = 'default'; 
    } else {
      img_elements[i].addEventListener('click',open_image_viewer);
    }
  }
/*  image_viewer = document.getElementById('image_viewer');
  image_viewer.addEventListener('click',close_image_viewer);
  image_box = document.getElementById('image_box');
  image_box.addEventListener('click',close_image_viewer);*/

  left_arrows = document.getElementsByClassName('left_arrow');
  for(var i=0; i<left_arrows.length; i++){
    left_arrows[i].addEventListener('click',previous_img);
  }
  right_arrows = document.getElementsByClassName('right_arrow');
  for(var i=0; i<right_arrows.length; i++){
    right_arrows[i].addEventListener('click',next_img);
  }


}

function open_image_viewer(evt){
  img_src = evt.currentTarget.src
  image_viewer_img = document.getElementById('image_viewer_image')
  image_viewer_img.src = img_src

  image_viewer = document.getElementById('image_viewer');
  image_viewer.style.display='block';
}

function close_image_viewer(){
  image_viewer = document.getElementById('image_viewer')
  image_viewer.style.display='none';
}

function previous_img(evt){
  change_image(evt,1);
}
function next_img(evt){
  change_image(evt,-1);
}

function change_image(evt,sgn) { 
  img_div = evt.currentTarget.parentElement.previousElementSibling;
  img_elems = img_div.children;
  for(var i=0; i<img_elems.length;i++){
    display_property = window.getComputedStyle(img_elems[i], null).display;
    next_ind = i + sgn;
    next_ind_exists = 0 <= next_ind && next_ind < img_elems.length;
    if(display_property == 'block' && next_ind_exists){
      img_div.children[i].style.display   = 'none';
      img_div.children[next_ind].style.display = 'block';
      return
    }
  }

} 
