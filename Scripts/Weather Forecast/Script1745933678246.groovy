import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import org.json.JSONObject as JSONObject
import org.json.JSONTokener as JSONTokener
import org.everit.json.schema.Schema as Schema
import org.everit.json.schema.loader.SchemaLoader as SchemaLoader
import groovy.json.JsonSlurper as JsonSlurper
import internal.GlobalVariable as GlobalVariable
import org.assertj.core.api.Assertions.*

def myOutput = WS.sendRequest(findTestObject('Weather/Get_WeatherForecast'))

WS.verifyResponseStatusCode(myOutput, 200)
WS.verifyElementPropertyValue(myOutput, 'city.name', 'Jagakarsa')

try {
    def schemaPath = 'Include/resources/schemas/weather-schema.json'  // lokasi file schema
    def schemaFile = new File(schemaPath)
    def jsonSchema = new JSONObject(new JSONTokener(schemaFile.newReader()))
    Schema schema = SchemaLoader.load(jsonSchema)

    def responseJson = new JSONObject(myOutput.getResponseText())
    schema.validate(responseJson)  // validasi JSON

    KeywordUtil.logInfo('Response sesuai dengan JSON schema')
} catch (Exception e) {
    KeywordUtil.markFailed("Validasi schema gagal: " + e.getMessage())
}

def slurper = new JsonSlurper()
def parsedResponse = slurper.parseText(myOutput.getResponseText())

def forecastList = parsedResponse.list

// Ambil dan grup berdasarkan tanggal (yyyy-MM-dd)
def groupedByDate = forecastList.groupBy { it.dt_txt.split(' ')[0] }

// Urutkan berdasarkan tanggal
def sortedDates = groupedByDate.keySet().sort()

KeywordUtil.logInfo("Cuaca berdasarkan tanggal dan jam (diurutkan):")

// Loop per tanggal
sortedDates.each { date ->
    KeywordUtil.logInfo("Tanggal: ${date}")
    
    // Ambil list item untuk tanggal ini, urutkan berdasarkan jam
    def items = groupedByDate[date].sort { it.dt_txt }
    
    // Loop dan print jam + deskripsi cuaca
    items.each { item ->
        def time = item.dt_txt.split(' ')[1]
        def weatherDesc = item.weather[0].description
        KeywordUtil.logInfo("  Jam ${time} : Cuaca -> ${weatherDesc}")
    }
}
