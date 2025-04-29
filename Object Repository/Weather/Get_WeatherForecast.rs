<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get_WeatherForecast</name>
   <tag></tag>
   <elementGuidId>77445ad0-32b7-4c09-86be-81ffd77b5841</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/forecast?lat=${GlobalVariable.lat}&amp;lon=${GlobalVariable.lon}&amp;appid=${GlobalVariable.API_key}&amp;units=metric</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable


import org.everit.json.schema.Schema
import org.everit.json.schema.loader.SchemaLoader
import org.json.JSONObject
import org.json.JSONTokener


RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, 'city.name', 'Jagakarsa')


def schemaPath = 'Include/resources/schemas/AirPollution-schema.json'
def schemaFile = new File(schemaPath)
def jsonSchema = new JSONObject(new JSONTokener(schemaFile.newReader()))
Schema schema = SchemaLoader.load(jsonSchema)
def responseJson = new JSONObject(response.getResponseText())
schema.validate(responseJson)


assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
