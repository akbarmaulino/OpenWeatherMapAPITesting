<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get_AirPollution</name>
   <tag></tag>
   <elementGuidId>637057b1-a3a2-4298-80ed-7493fabb6462</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/air_pollution?lat=${GlobalVariable.lat}&amp;lon=${GlobalVariable.lon}&amp;appid=${GlobalVariable.API_key}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*&#xd;
&#xd;
import com.kms.katalon.core.testobject.RequestObject&#xd;
import com.kms.katalon.core.testobject.ResponseObject&#xd;
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS&#xd;
import com.kms.katalon.core.webservice.verification.WSResponseManager&#xd;
&#xd;
import groovy.json.JsonSlurper&#xd;
import internal.GlobalVariable as GlobalVariable&#xd;
&#xd;
&#xd;
import org.everit.json.schema.Schema&#xd;
import org.everit.json.schema.loader.SchemaLoader&#xd;
import org.json.JSONObject&#xd;
import org.json.JSONTokener&#xd;
&#xd;
&#xd;
RequestObject request = WSResponseManager.getInstance().getCurrentRequest()&#xd;
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()&#xd;
&#xd;
&#xd;
WS.verifyResponseStatusCode(response, 200)&#xd;
assertThat(response.getStatusCode()).isEqualTo(200)&#xd;
&#xd;
&#xd;
WS.verifyElementPropertyValue(response, 'coord.lon', 106.8294)&#xd;
WS.verifyElementPropertyValue(response, 'coord.lat', -6.3611)&#xd;
&#xd;
def schemaPath = 'Include/resources/schemas/AirPollution-schema.json'&#xd;
def schemaFile = new File(schemaPath)&#xd;
def jsonSchema = new JSONObject(new JSONTokener(schemaFile.newReader()))&#xd;
Schema schema = SchemaLoader.load(jsonSchema)&#xd;
def responseJson = new JSONObject(response.getResponseText())&#xd;
schema.validate(responseJson)&#xd;
&#xd;
assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
