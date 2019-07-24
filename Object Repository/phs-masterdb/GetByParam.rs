<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetByParam</name>
   <tag></tag>
   <elementGuidId>bacf9672-c782-4bb5-9175-1329ef90429c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: ${id},\n  \&quot;param\&quot;: \&quot;${param}\&quot;,\n  \&quot;value\&quot;: ${value}\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${endpoint}/phs-masterdb/rest/api/phsParam/getByParam</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>d23e47d3-3f97-4379-a62f-e491a86320ad</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/getByParam').getValue(1, 1)</defaultValue>
      <description></description>
      <id>f88f632a-9465-41cf-99b6-d361358da4ec</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/getByParam').getValue(2, 1)</defaultValue>
      <description></description>
      <id>7ddab530-5bf8-4990-8fd6-0a18f6f8241a</id>
      <masked>false</masked>
      <name>param</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/getByParam').getValue(3, 1)</defaultValue>
      <description></description>
      <id>7a6efe0e-f02f-4629-bdd7-8f54d3f14a8f</id>
      <masked>false</masked>
      <name>value</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
