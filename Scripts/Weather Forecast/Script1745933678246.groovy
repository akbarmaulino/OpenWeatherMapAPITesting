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

// Kirim request dan ambil output
def myOutput = WS.sendRequest(findTestObject('Weather/Get_WeatherForecast'))

// Verifikasi status code
WS.verifyResponseStatusCode(myOutput, 200)

// Verifikasi nama kota
WS.verifyElementPropertyValue(myOutput, 'city.name', 'Jagakarsa')



try {
	def schemaPath = 'Include/resources/schemas/weather-schema.json'
	
	def schemaFile = new File(schemaPath)
	
	def jsonSchema = new JSONObject(new JSONTokener(schemaFile.newReader()))
	
	Schema schema = SchemaLoader.load(jsonSchema)
	
	def responseJson = new JSONObject(myOutput.getResponseText())
	
	schema.validate(responseJson)
	
	KeywordUtil.logInfo('Response matches the JSON schema')

} catch (Exception e) {
	KeywordUtil.markFailed("Validation failed: " + e.getMessage())
}

