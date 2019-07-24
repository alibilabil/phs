<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>saveEvdLifeAssured</name>
   <tag></tag>
   <elementGuidId>e595815d-1e22-4157-ad69-eb7326c76c92</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;genId\&quot;: \&quot;${genId}\&quot;,\n  \&quot;clientNo\&quot;: \&quot;${clientNo}\&quot;,\n  \&quot;role\&quot;: \&quot;${role}\&quot;,\n  \&quot;evdCode\&quot;: \&quot;${evdCode}\&quot;,\n  \&quot;evdDesc\&quot;: \&quot;${evdDesc}\&quot;,\n  \&quot;trxIn\&quot;: ${trxIn},\n  \&quot;resource\&quot;: \&quot;${resource}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-transaction/rest/api/saveEvdLifeAssured</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>3248d393-a2b9-41dc-9c26-8e5ab7bfeeda</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveEvdLifeAssured').getValue(1, 1)</defaultValue>
      <description></description>
      <id>cb8dfd07-2ed1-43b4-9eda-0ec1ad577518</id>
      <masked>false</masked>
      <name>genId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveEvdLifeAssured').getValue(2, 1)</defaultValue>
      <description></description>
      <id>c76390f8-5d3d-4afb-a540-0893de35607d</id>
      <masked>false</masked>
      <name>clientNo</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveEvdLifeAssured').getValue(3, 1)</defaultValue>
      <description></description>
      <id>85f71893-6210-4d13-8aef-52386a7cd9d3</id>
      <masked>false</masked>
      <name>role</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveEvdLifeAssured').getValue(4, 1)</defaultValue>
      <description></description>
      <id>8bdec545-c003-4c8c-bc74-238535d6dfe2</id>
      <masked>false</masked>
      <name>evdCode</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveEvdLifeAssured').getValue(5, 1)</defaultValue>
      <description></description>
      <id>1ce5c728-4259-4e35-b342-81b12e3a2517</id>
      <masked>false</masked>
      <name>evdDesc</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveEvdLifeAssured').getValue(6, 1)</defaultValue>
      <description></description>
      <id>29e3be42-e3ec-4f17-abd6-c46b3222e55a</id>
      <masked>false</masked>
      <name>trxIn</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveEvdLifeAssured').getValue(7, 1)</defaultValue>
      <description></description>
      <id>2c21a6f3-6a2a-49b8-9c34-ac5cb09c9bcb</id>
      <masked>false</masked>
      <name>resource</name>
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
