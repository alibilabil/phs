<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>dataCaptureMajor</name>
   <tag></tag>
   <elementGuidId>46809ee1-0e6c-44fe-a51b-d08a379ca049</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;docId\&quot;: [\n    {\n      \&quot;docId\&quot;: \&quot;${docId}\&quot;,\n      \&quot;docIdType\&quot;: \&quot;${docIdType}\&quot;\n    }\n  ],\n  \&quot;policyNumber\&quot;: \&quot;${policyNumber}\&quot;,\n  \&quot;scanTime\&quot;: \&quot;${scanTime}\&quot;,\n  \&quot;spajNo\&quot;: \&quot;${spajNo}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-datacapture/rest/api/dataCaptureMajor</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>e7e8e707-f2d4-44f2-a73d-0c4d813bfdf8</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-datacapture/dataCaptureMajor').getValue(1, 1)</defaultValue>
      <description></description>
      <id>bdb8b235-7997-49b2-bf81-760b60dc7bff</id>
      <masked>false</masked>
      <name>docId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-datacapture/dataCaptureMajor').getValue(2, 1)</defaultValue>
      <description></description>
      <id>c0e297aa-56e6-4ff1-ba09-4af2009eabd7</id>
      <masked>false</masked>
      <name>docIdType</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-datacapture/dataCaptureMajor').getValue(3, 1)</defaultValue>
      <description></description>
      <id>d6b18c61-d8e8-4725-8524-667dab509621</id>
      <masked>false</masked>
      <name>policyNumber</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-datacapture/dataCaptureMajor').getValue(4, 1)</defaultValue>
      <description></description>
      <id>c8ae6083-a71e-4035-9244-b7f83244efb0</id>
      <masked>false</masked>
      <name>scanTime</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-datacapture/dataCaptureMajor').getValue(5, 1)</defaultValue>
      <description></description>
      <id>fd2ba0cc-a85d-4dc3-a434-3242489073e9</id>
      <masked>false</masked>
      <name>variablespajNo</name>
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
