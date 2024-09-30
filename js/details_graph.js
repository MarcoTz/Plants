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
