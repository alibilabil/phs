<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BOAddCompSarCal</name>
   <tag></tag>
   <elementGuidId>c188f084-43b9-40cb-a5e3-ba0a7fd0fa9d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;objid\&quot;: \&quot;${objid}\&quot;,\n  \&quot;clntnum\&quot;: \&quot;${clntnum}\&quot;,\n  \&quot;currcy\&quot;: \&quot;${currcy}\&quot;,\n  \&quot;crtable01\&quot;: \&quot;${crtable01}\&quot;,\n  \&quot;rcesagenew\&quot;: \&quot;${rcesagenew}\&quot;,\n  \&quot;rcestrmnew\&quot;: \&quot;${rcestrmnew}\&quot;,\n  \&quot;suminsnew\&quot;: \&quot;${suminsnew}\&quot;,\n  \&quot;zsprm\&quot;: \&quot;${zsprm}\&quot;,\n  \&quot;chdrnum\&quot;: \&quot;${chdrnum}\&quot;,\n  \&quot;rider\&quot;: \&quot;${rider}\&quot;,\n  \&quot;crtable02\&quot;: \&quot;${crtable02}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-sarcalculator/rest/api/BOAddCompSarCal</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>de3fa071-bb03-47a1-92c4-6cd2569f17e8</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(1, 1)</defaultValue>
      <description></description>
      <id>1726ec66-926b-473e-ac4e-c749c834107b</id>
      <masked>false</masked>
      <name>objid</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(2, 1)</defaultValue>
      <description></description>
      <id>82150119-5e29-4a4f-96ca-e18f7328cdc0</id>
      <masked>false</masked>
      <name>clntnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(3, 1)</defaultValue>
      <description></description>
      <id>29b28ace-9dc7-4cf6-b44b-ae763ce9470d</id>
      <masked>false</masked>
      <name>currcy</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(4, 1)</defaultValue>
      <description></description>
      <id>e92c57b7-9c69-4895-b48f-09dda63c331d</id>
      <masked>false</masked>
      <name>crtable01</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(5, 1)</defaultValue>
      <description></description>
      <id>c40f023d-bbdb-48d8-95a4-bff6617b2533</id>
      <masked>false</masked>
      <name>rcesagenew</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(6, 1)</defaultValue>
      <description></description>
      <id>b19aa3ad-19bc-4e71-af69-d5b3f27e24b4</id>
      <masked>false</masked>
      <name>rcestrmnew</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(7, 1)</defaultValue>
      <description></description>
      <id>d8bfe7d1-fa4b-49d0-b74a-d46efb2e1da4</id>
      <masked>false</masked>
      <name>suminsnew</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(8, 1)</defaultValue>
      <description></description>
      <id>d1d526c0-1460-44d2-837d-dec5879e3edf</id>
      <masked>false</masked>
      <name>zsprm</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(9, 1)</defaultValue>
      <description></description>
      <id>abdd4f6c-01dc-4b91-b7c1-0489ea013730</id>
      <masked>false</masked>
      <name>chdrnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(10, 1)</defaultValue>
      <description></description>
      <id>3f165a5e-fa57-446c-8384-b870a0ff1fc3</id>
      <masked>false</masked>
      <name>rider</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(11, 1)</defaultValue>
      <description></description>
      <id>680d3026-7c87-4363-a71a-9fc9a1158b31</id>
      <masked>false</masked>
      <name>crtable02</name>
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
