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

