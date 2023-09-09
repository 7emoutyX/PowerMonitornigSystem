![Screenshot 2023-09-09 005037](https://github.com/7emoutyX/PowerMonitornigSystem/assets/110437117/c0cc4c81-a512-41dd-a39a-97930d320659)# PowerMonitornigSystem
# Solar Panel Monitoring System

## Project Overview
the electonic build made by Altuim Designer
![Screenshot 2023-09-09 005037](https://github.com/7emoutyX/PowerMonitornigSystem/assets/110437117/268079d6-3249-47e8-a7c4-717cad6717f7)
All the sensors work with I2C protocl , u should take caution while using the INA219 module , change the adress of one of the modules by applying some soldering in the adress pins usually it should be 0x58 after applying the soldering .


## Key Features

- **Ina219 Power Consumption Module**: Integration of the Ina 219 power consumption module allows precise measurement of power usage by each solar panel.

- **Real-Time Data Visualization**: We developed a web-based interface that displays real-time data of power consumption for each solar panel. This interface also correlates the data with time and luminosity, providing valuable insights for optimization.

- **Data Storage**: To ensure data integrity, we implemented a system that stores the collected data in both an SQL database and an Excel file on a memory card. This redundancy ensures that our data is secure and accessible for analysis.

- **Server on ESP32**: The server is hosted on the ESP32 itself, using Spiffs to allocate the FLASH Memory for the HTML and JavaScript codes.

## Getting Started

If you would like to set up and run this project locally, please follow these steps:

1. Clone this repository:

   ```bash
   git clone https://github.com/your-username/solar-panel-monitoring.git

