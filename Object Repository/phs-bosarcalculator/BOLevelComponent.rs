<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BOLevelComponent</name>
   <tag></tag>
   <elementGuidId>eb6ed3e2-0dcb-4536-8d3c-e3391643e252</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;objid\&quot; : \&quot;${objid}\&quot;,\n  \&quot;clntnum\&quot; : \&quot;${clntnum}\&quot;,\n  \&quot;currcd\&quot; : \&quot;${currcd}\&quot;,\n  \&quot;chdrnum\&quot; : \&quot;${chdrnum}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-bosarcalculator/rest/api/BOLevelComponent</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>a2ecfdf3-19bf-46c8-b75a-c347c0e266b4</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOLevelComponent').getValue(1, 1)</defaultValue>
      <description></description>
      <id>679ce1be-0723-4cff-9eb0-818dce0ce2ae</id>
      <masked>false</masked>
      <name>objid</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOLevelComponent').getValue(2, 1)</defaultValue>
      <description></description>
      <id>9164130c-3950-47e8-9d38-f12cfa09717b</id>
      <masked>false</masked>
      <name>clntnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOLevelComponent').getValue(3, 1)</defaultValue>
      <description></description>
      <id>2194abac-7fa9-4bba-a2e9-2033c3f8e844</id>
      <masked>false</masked>
      <name>currcd</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOLevelComponent').getValue(4, 1)</defaultValue>
      <description></description>
      <id>b64d4a69-47df-4233-ae06-725651f3ab00</id>
      <masked>false</masked>
      <name>chdrnum</name>
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
