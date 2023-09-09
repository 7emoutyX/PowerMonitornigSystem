# Solar Panel Monitoring System

## Project Overview
Hardware Design build made by Altuim Designer
## Getting Started
If you would like to set up and run this project locally, please follow these steps:

- All the sensors work with I2C protocl , u should take caution while using the INA219 module , change the adress of one of the modules by applying some soldering in the adress pins usually it should be 0x58 after applying the soldering .
- The server side ( Javascript / html and css codes ) are located in the esp32 internal flash memroy , u can do that by using spiffs or little FS , anyway by using this serial communication with memory bus , u can locate some memory enough for importing the server side codes . u just need to calculate the size of the 3 files and allocate the memory that u need , i used between 0x0755134 and 0x0790231 .
- One of the reasons why i used RUST to build this code is the momory problem , if I used Cpp i'm gonna have problem with allocating the pointers and memory waste and that gonna block the memory serial allocating .

- Schematic view :
![Screenshot 2023-09-09 005037](https://github.com/7emoutyX/PowerMonitornigSystem/assets/110437117/268079d6-3249-47e8-a7c4-717cad6717f7)

- PCB view :
![Screenshot 2023-09-09 004545](https://github.com/7emoutyX/PowerMonitornigSystem/assets/110437117/771c7a3f-80ce-4d7c-80d5-b4f9bccd90c3)

- Final Result view :
![WhatsApp Image 2023-09-09 at 02 41 21](https://github.com/7emoutyX/PowerMonitornigSystem/assets/110437117/e7882d66-13a1-452a-991a-a40cd58b6f5f)




## Key Features

- **Ina219 Power Consumption Module**: Integration of the Ina 219 power consumption module allows precise measurement of power usage by each solar panel.

- **Real-Time Data Visualization**: We developed a web-based interface that displays real-time data of power consumption for each solar panel. This interface also correlates the data with time and luminosity, providing valuable insights for optimization.

- **Data Storage**: To ensure data integrity, we implemented a system that stores the collected data in both an SQL database and an Excel file on a memory card. This redundancy ensures that our data is secure and accessible for analysis.

- **Server on ESP32**: The server is hosted on the ESP32 itself, using Spiffs to allocate the FLASH Memory for the HTML and JavaScript codes.



. Clone this repository:

   ```bash
   git clone https://github.com/your-username/solar-panel-monitoring.git

