function create_growth_graph(){
  new Chart("growth_chart", {
    type: "line",
    data: {
      labels: growth_dates,
      datasets: [{
        data: growth_heights,
        label : "height",
        borderColor: "green",
        fill: false
      },{
        data: growth_widths,
        label : "width",
        borderColor: "blue",
        fill: false
      }]
    },
    options: {
      legend: {display: true},
      scales: {
        yAxes:[{
          'ticks': {fontColor:'rgba(102,204,224,100)'}
        }],
        xAxes:[{
          'ticks':{fontColor:'rgba(102,204,224,100)'}
        }]
      }
    }
  });
}

function create_health_graph(){
  new Chart("health_chart", {
    type: "line",
    data: {
      labels: health_dates,
      datasets: [{
        data: health_healths,
        label : "health",
        borderColor: "red",
        fill: false
      }]
    },
    options: {
      legend: {display: true},
      scales: {
        yAxes:[{
          'ticks': {fontColor:'rgba(102,204,224,100)'}
        }],
        xAxes:[{
          'ticks':{fontColor:'rgba(102,204,224,100)'}
        }]
      }
    }
  });

}

function create_graphs(){
  create_growth_graph();
  create_health_graph();
}
