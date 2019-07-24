<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ExtTypeUpdate</name>
   <tag></tag>
   <elementGuidId>67912a26-3a57-40f4-ad27-98eebf433534</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: ${id},\n  \&quot;mime\&quot;: \&quot;${mime}\&quot;,\n  \&quot;status\&quot;: \&quot;${status}\&quot;,\n  \&quot;userId\&quot;: \&quot;${userId}\&quot;,\n  \&quot;value\&quot;: \&quot;${value}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-masterdb/rest/api/extType/update</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>d08eb437-7f38-4cbb-8e40-bebff880e63d</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/extTypeUpdate').getValue(1, 1)</defaultValue>
      <description></description>
      <id>47c9cf97-e103-4d6b-8a7d-266cfbb24629</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/extTypeUpdate').getValue(2, 1)</defaultValue>
      <description></description>
      <id>78bb4823-f6ec-474e-a41a-2e2a20cded9f</id>
      <masked>false</masked>
      <name>mime</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/extTypeUpdate').getValue(3, 1)</defaultValue>
      <description></description>
      <id>9b15576b-9549-4f69-9bfd-765810fc93ed</id>
      <masked>false</masked>
      <name>status</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/extTypeUpdate').getValue(4, 1)</defaultValue>
      <description></description>
      <id>43f71b02-6939-47e3-b13d-e1ece1d593da</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/extTypeSave').getValue(5, 1)</defaultValue>
      <description></description>
      <id>acdc5ad2-b8da-4619-bc56-5277731f3d89</id>
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
