// Complete project details: https://randomnerdtutorials.com/esp8266-nodemcu-plot-readings-charts-multiple/

// Get current sensor readings when the page loads
window.addEventListener('load', getReadings);

// Create Temperature Chart
var chartT = new Highcharts.Chart({
  chart:{
    renderTo:'chart-temperature'
  },
  series: [
    {
      name: 'Power1',
      type: 'line',
      color: '#15265b',
      marker: {
        symbol: 'circle',
        radius: 3,
        fillColor: '#101D42',
      }
    },
    {
      name: 'Power2',
      type: 'line',
      color: '#f6001d',
      marker: {
        symbol: 'square',
        radius: 3,
        fillColor: '#00A6A6',
      }
    },
  ],
  title: {
    text: undefined
  },
  xAxis: {
    type: 'datetime',
    dateTimeLabelFormats: { second: '%H:%M:%S' }
  },
  yAxis: {
    title: {
      text: 'Power (mW)'
    }
  },
  credits: {
    enabled: false
  }
});

//Plot temperature in the temperature chart
function plotTemperature(jsonValue) {
  var keys = Object.keys(jsonValue);
  for (var i = 0; i < keys.length; i++){
    var x = (new Date()).getTime();
    const key = keys[i];
    var y = Number(jsonValue[key]);
    if(chartT.series[i].data.length > 40) {
      chartT.series[i].addPoint([x, y], true, true, true);
    } else {
      chartT.series[i].addPoint([x, y], true, false, true);
    }
  }

  // Here, you can add code to display the luminosity data as text
  // Assuming you have the luminosity data in a variable called luminosityData

  // Get the sensor luminosity data text div element
  const sensorLuminosityDiv = document.getElementById("sensor-luminosity");

  // Replace the placeholder value with your actual luminosity data
  const luminosityData = 500; // Replace this with your actual luminosity data value

  // Update the content of the div with the luminosity data
  sensorLuminosityDiv.innerHTML = `<p>Luminosity: ${luminosityData} Lux</p>`;
}

// Function to get current readings on the webpage when it loads for the first time
function getReadings(){
  var xhr = new XMLHttpRequest();
  xhr.onreadystatechange = function() {
    if (this.readyState == 4 && this.status == 200) {
      console.log(this.responseText);
    }
  };
  xhr.open("GET", "/readings", true);
  xhr.send();
}

if (!!window.EventSource) {
  var source = new EventSource('/events');

  source.addEventListener('open', function(e) {
    console.log("Events Connected");
  }, false);

  source.addEventListener('error', function(e) {
    if (e.target.readyState != EventSource.OPEN) {
      console.log("Events Disconnected");
    }
  }, false);

  source.addEventListener('message', function(e) {
    console.log("message", e.data);
  }, false);

  source.addEventListener('new_readings', function(e) {
    console.log("new_readings", e.data);
    var myObj = JSON.parse(e.data);
    console.log(myObj);
    plotTemperature(myObj);
  }, false);
}
