import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import org.json.JSONObject
import org.json.JSONTokener
import org.everit.json.schema.Schema
import org.everit.json.schema.loader.SchemaLoader
import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
import org.assertj.core.api.Assertions.*
import static org.assertj.core.api.Assertions.assertThat
import static java.util.Arrays.asList
import com.kms.katalon.core.util.KeywordUtil



myOutput = WS.sendRequest(findTestObject('AirPollution/Get_AirPollution'))


WS.verifyResponseStatusCode(myOutput, 200)


WS.verifyElementPropertyValue(myOutput, 'coord.lon', 106.8294)
WS.verifyElementPropertyValue(myOutput, 'coord.lat', -6.3611)

try {
    // Verifikasi status code
    def statusCode = myOutput.getStatusCode()
    assertThat(statusCode).isIn(Arrays.asList(200, 201, 202))
    KeywordUtil.logInfo("Status code valid")

    // Validasi JSON schema
    def schemaPath = 'Include/resources/schemas/AirPollution-schema.json'
    def schemaFile = new File(schemaPath)
    def jsonSchema = new JSONObject(new JSONTokener(schemaFile.newReader()))
    Schema schema = SchemaLoader.load(jsonSchema)
    def responseJson = new JSONObject(myOutput.getResponseText())
    schema.validate(responseJson)
    KeywordUtil.logInfo("Response matches the JSON schema")

} catch (Exception e) {
    KeywordUtil.markFailed("Validation failed: " + e.getMessage())
}






