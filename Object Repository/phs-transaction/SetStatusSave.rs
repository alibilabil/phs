<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetStatusSave</name>
   <tag></tag>
   <elementGuidId>fc9b4f4f-889e-4c94-bf70-875d587154d9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;reqType\&quot;: \&quot;${reqType}\&quot;,\n  \&quot;sftpstat\&quot;: {\n    \&quot;id\&quot;: ${id},\n    \&quot;status\&quot;: ${status},\n    \&quot;trxIn\&quot;: ${trxIn},\n    \&quot;userId\&quot;: ${userId},\n    \&quot;trxUpdate\&quot;: ${trxUpdate},\n    \&quot;userUpdate\&quot;: ${userUpdate}\n  }\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-transaction/rest/api/setStatus</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>3fd89c18-40ac-4d71-8577-2df3cb309400</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/setStatusSave').getValue(1, 1)</defaultValue>
      <description></description>
      <id>19199ee4-3b20-4c42-8313-b49fae398978</id>
      <masked>false</masked>
      <name>reqType</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/setStatusSave').getValue(2, 1)</defaultValue>
      <description></description>
      <id>f92ef74a-c09b-4b26-a99a-171c8bef6fef</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/setStatusSave').getValue(3, 1)</defaultValue>
      <description></description>
      <id>cbd31f06-9d08-4b18-8a4c-5e049897b638</id>
      <masked>false</masked>
      <name>status</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/setStatusSave').getValue(4, 1)</defaultValue>
      <description></description>
      <id>b8e1998f-13a5-45eb-90b3-70ff6235b7a2</id>
      <masked>false</masked>
      <name>trxIn</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/setStatusSave').getValue(5, 1)</defaultValue>
      <description></description>
      <id>52bf3986-9c02-4b23-b0b2-78a2fd199574</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/setStatusSave').getValue(6, 1)</defaultValue>
      <description></description>
      <id>48af46ff-28f4-457c-b3c3-7ef4fbf936e5</id>
      <masked>false</masked>
      <name>trxUpdate</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/setStatusSave').getValue(7, 1)</defaultValue>
      <description></description>
      <id>12d55d66-a2c9-4559-9071-05ff0b5ceb16</id>
      <masked>false</masked>
      <name>userUpdate</name>
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
