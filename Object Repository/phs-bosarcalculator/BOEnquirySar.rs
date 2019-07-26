<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BOEnquirySar</name>
   <tag></tag>
   <elementGuidId>df359a43-dd50-4e1f-a447-70c0b452ff4e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;objid\&quot;: \&quot;${objid}\&quot;,\n  \&quot;clntnum\&quot;: \&quot;${objid}\&quot;,\n  \&quot;currcy\&quot;: \&quot;${currcy}\&quot;,\n  \&quot;chdrnum\&quot;: \&quot;${chdrnum}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-sarcalculator/rest/api/BOEnquirySar</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>56c7db92-5c47-4742-8bfe-6fde0d6b736b</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOEnquirySar').getValue(1, 1)</defaultValue>
      <description></description>
      <id>540cdc48-6955-4b4b-a5d5-eb7ffec8595b</id>
      <masked>false</masked>
      <name>objid</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOEnquirySar').getValue(2, 1)</defaultValue>
      <description></description>
      <id>22093d92-b6d7-43b7-a3c5-36edd6eb0361</id>
      <masked>false</masked>
      <name>clntnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOEnquirySar').getValue(3, 1)</defaultValue>
      <description></description>
      <id>e03dc613-9dfd-4489-9059-a630969d448d</id>
      <masked>false</masked>
      <name>currcy</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOEnquirySar').getValue(4, 1)</defaultValue>
      <description></description>
      <id>59c1ad5f-4948-4491-a525-f5d3799a6061</id>
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
