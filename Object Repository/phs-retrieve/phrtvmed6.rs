<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>To retrieve follow up code for medical six months</description>
   <name>phrtvmed6</name>
   <tag></tag>
   <elementGuidId>60469b04-dd2a-4535-b1d4-620f8d959711</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;objid\&quot;: \&quot;${objid}\&quot;,\n  \&quot;clntnum\&quot;: \&quot;${clntnum}\&quot;\n}&quot;,
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
   <restUrl>${endpoint}/phs-retrieve/rest/api/phrtvmed6</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>bb5573e3-524b-4ad0-b92d-0b2c7245b6b0</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-retrieve/phrtvmed6').getValue(1, 1)</defaultValue>
      <description></description>
      <id>902c6128-7661-4a83-b4ee-bdcb62de1c03</id>
      <masked>false</masked>
      <name>objid</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-retrieve/phrtvmed6').getValue(2, 1)</defaultValue>
      <description></description>
      <id>4ab10fa6-e6a9-4a86-8c37-dba3dcf6c11a</id>
      <masked>false</masked>
      <name>clntnum</name>
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
