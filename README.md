# OpenWeatherMap API Automation Test (Katalon Studio)

This project is an API automation test suite built using **Katalon Studio**, designed to test endpoints from the [OpenWeatherMap API](https://openweathermap.org/api), including:

- **5-day Weather Forecast**
- **Current Air Pollution Data**

The project includes status code verification, response data validation, and JSON Schema validation.


## Project Structure

- Test Cases
    - Weather Forecast
    - Air Pollution
- Object Repository
    - Weather
        - Get_WeatherForecast
    - AirPollution
        - Get_AirPollution
- Include
    - resources
        - schemas
            - weather-schema.json
            - AirPollution-schema.json


##  How to Run the Tests

### Requirements

- Katalon Studio
- An OpenWeatherMap API Key 

### Steps

1. **Clone or Download the Repository**
    git clone https://github.com/your-username/openweather-api-test.git

2. **Open Project in Katalon Studio**
    - Launch Katalon Studio
    - Select **File > Open Project** and choose this project directory

3. **Set API Key in Profile**
    - Go to **Profiles > default**
    - Set your API Key and Base URL:
      GlobalVariable.api_key = 'API_Key'
      GlobalVariable.base_url = 'https://api.openweathermap.org/data/2.5'


4. **Run Test Cases**
    - Navigate to **Test Cases**
    - Right-click on `Weather Forecast` or `Air Pollution` and select **Run**

5. **View Console Output**



## How to Get the Report


1. Go to the **Reports** folder 
2. Each test run will have a timestamped folder
3. Inside each folder:
    - `report.html`
