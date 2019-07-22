<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>update</name>
   <tag></tag>
   <elementGuidId>ee3dd257-e6f8-4150-aa78-ddfc266c6f76</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;docType\&quot;: \&quot;${docType1}\&quot;,\n  \&quot;param\&quot;: [\n    {\n      \&quot;id\&quot;: ${id},\n      \&quot;docId\&quot;: \&quot;${docId}\&quot;,\n      \&quot;docType\&quot;: \&quot;${docType}\&quot;\n    }\n  ]\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-masterdb/rest/api/docType/update</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>4728165c-6b04-471b-86b9-50b42c87c21c</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/update').getValue(1, 1)</defaultValue>
      <description></description>
      <id>7d99dea8-82b6-4eac-8668-1a4ea30b85e3</id>
      <masked>false</masked>
      <name>docType1</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/update').getValue(2, 1)</defaultValue>
      <description></description>
      <id>98b3580c-215c-4775-9441-d55dfa0454b9</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/update').getValue(3, 1)</defaultValue>
      <description></description>
      <id>3bc27502-bc05-4b84-b5ad-7a9c8921c0ec</id>
      <masked>false</masked>
      <name>docId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/update').getValue(4, 1)</defaultValue>
      <description></description>
      <id>d3261dde-4d15-4552-8e75-9b90ecd5b707</id>
      <masked>false</masked>
      <name>docType</name>
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
