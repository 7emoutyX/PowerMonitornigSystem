# Solar Panel Monitoring System

![Project Image](https://github.com/7emoutyX/PowerMonitornigSystem/assets/110437117/e7882d66-13a1-452a-991a-a40cd58b6f5f) 

## Project Overview

This project showcases a comprehensive Solar Panel Monitoring System, consisting of both hardware and software components. The hardware design was created using Altium Designer, and it's designed to monitor power efficiency for solar panels.

## Getting Started

To set up and run this project locally, follow these steps:

- **Sensor Integration**: All sensors in this system work with the I2C protocol. Familiarity with the Wire.h library and oscilloscope analysis is recommended. You should take special care when using the INA219 module; change the address of one of the modules by applying soldering to the address pins, typically resulting in an address of 0x58.

- **Server-Side Storage**: The server-side code (JavaScript, HTML, and CSS) is located in the ESP32's internal flash memory. You can achieve this by using SPIFFS or LittleFS to allocate memory for importing the server-side code. Calculate the size of the three files and allocate the required memory. Memory allocation using serial communication with the memory bus is critical for this setup.

- **Programming Language Choice**: The choice of Rust for building this project was influenced by memory constraints. Using C++ could lead to memory allocation issues and inefficient memory usage.

## Project Visuals

- **Schematic View**:
  ![Schematic View](https://github.com/7emoutyX/PowerMonitornigSystem/assets/110437117/268079d6-3249-47e8-a7c4-717cad6717f7)

- **PCB View**:
  ![PCB View](https://github.com/7emoutyX/PowerMonitornigSystem/assets/110437117/771c7a3f-80ce-4d7c-80d5-b4f9bccd90c3)

- **Final Result View**:
  ![Final Result](https://github.com/7emoutyX/PowerMonitornigSystem/assets/110437117/e7882d66-13a1-452a-991a-a40cd58b6f5f)

## Key Features

- **Ina219 Power Consumption Module**: The system integrates the Ina 219 power consumption module, enabling precise measurement of power usage by individual solar panels.

- **Real-Time Data Visualization**: A web-based interface has been developed to display real-time data on power consumption for each solar panel. This interface also correlates data with time and luminosity, offering valuable insights for optimization.

- **Data Storage**: To ensure data integrity, a robust data storage system has been implemented. Collected data is stored both in an SQL database and an Excel file on a memory card. This redundancy ensures that data remains secure and accessible for analysis.

- **ESP32-Based Server**: The project hosts its server on the ESP32 itself, using SPIFFS to allocate FLASH memory for HTML and JavaScript code.

## Clone this Repository

To get started, clone this repository using the following command:

```bash
git clone https://github.com/7emoutyX/PowerMonitornigSystem.git
