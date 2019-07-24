<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>saveErrorLog</name>
   <tag></tag>
   <elementGuidId>5b0810da-84ab-469e-84f9-af227d7dd615</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;genId\&quot;: \&quot;${genId}\&quot;,\n  \&quot;message\&quot;: \&quot;${message}\&quot;,\n  \&quot;taskName\&quot;: \&quot;${taskName}\&quot;,\n  \&quot;trxIn\&quot;: \&quot;${trxIn}\&quot;,\n  \&quot;type\&quot;: \&quot;${type}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-transaction/rest/api/saveErrorLog</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>85344186-7088-459c-b259-9b5e87e3616b</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveErrorLog').getValue(1, 1)</defaultValue>
      <description></description>
      <id>0bb2c1a0-ef28-4688-a9e1-e00570bf14b0</id>
      <masked>false</masked>
      <name>genId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveErrorLog').getValue(2, 1)</defaultValue>
      <description></description>
      <id>8ace8231-7be4-46e8-8055-3ff98f4f0dfe</id>
      <masked>false</masked>
      <name>message</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveErrorLog').getValue(3, 1)</defaultValue>
      <description></description>
      <id>32ac84cf-3e81-4f20-b475-ebd9f1ae6a67</id>
      <masked>false</masked>
      <name>taskName</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveErrorLog').getValue(4, 1)</defaultValue>
      <description></description>
      <id>b12f8721-a992-42ad-b1f0-5e1102690338</id>
      <masked>false</masked>
      <name>trxIn</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveErrorLog').getValue(5, 1)</defaultValue>
      <description></description>
      <id>9d5c28e8-ef1c-4384-a43e-1934429ef011</id>
      <masked>false</masked>
      <name>type</name>
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
