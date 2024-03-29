<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>To retrieve all follow up code from t5661</description>
   <name>phrtvt5661</name>
   <tag></tag>
   <elementGuidId>a752a000-a41f-4735-a19f-027057760cf5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;FUPCDE\&quot;: \&quot;${FUPCDE}\&quot;,\n  \&quot;OBJID\&quot;: \&quot;${OBJID}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-retrieve/rest/api/phrtvt5661</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>5ee029a7-9496-47bf-aa1d-c4d810c02dab</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-retrieve/phrtvt5661').getValue(1, 1)</defaultValue>
      <description></description>
      <id>354aa206-0afe-4867-88fa-8d587968805f</id>
      <masked>false</masked>
      <name>FUPCDE</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-retrieve/phrtvt5661').getValue(2, 1)</defaultValue>
      <description></description>
      <id>b07df789-ca6e-4c5a-afc2-53ca4e07caf8</id>
      <masked>false</masked>
      <name>OBJID</name>
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
