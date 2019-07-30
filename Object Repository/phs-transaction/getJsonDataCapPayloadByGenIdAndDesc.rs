<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>getJsonDataCapPayloadByGenIdAndDesc</name>
   <tag></tag>
   <elementGuidId>c83ec606-9356-4c61-9841-a94c1beaf64a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;genId\&quot;:\&quot;${genId}\&quot;,\n  \&quot;description\&quot;:\&quot;${description}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-transaction/rest/api/getJsonDataCapPayloadByGenIdAndDesc</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>6bef9e52-4884-474a-9c26-39477d7d5909</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/getJsonDataCapPayloadByGenIdAndDesc').getValue(1, 1)</defaultValue>
      <description></description>
      <id>c5a97a16-cb44-4da1-bd65-0e3e94e66245</id>
      <masked>false</masked>
      <name>genId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/getJsonDataCapPayloadByGenIdAndDesc').getValue(2, 1)</defaultValue>
      <description></description>
      <id>821df83b-f5e7-42d3-afa4-42fff767de6b</id>
      <masked>false</masked>
      <name>description</name>
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
