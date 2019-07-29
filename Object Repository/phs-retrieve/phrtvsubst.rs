<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>To retrieve substandard flag</description>
   <name>phrtvsubst</name>
   <tag></tag>
   <elementGuidId>bd0f8205-5447-4e4c-9fe6-aae936cc841b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;objid\&quot;: \&quot;${objid}\&quot;,\n  \&quot;chdrno\&quot;: \&quot;${chdrno}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-retrieve/rest/api/phrtvsubst</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>d4b418c1-4f72-4cd9-b05c-334d74f8fb0b</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-retrieve/phrtvsubst').getValue(1, 1)</defaultValue>
      <description></description>
      <id>ab8fc669-5295-42bf-a875-879129b81c48</id>
      <masked>false</masked>
      <name>objid</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-retrieve/phrtvsubst').getValue(2, 1)</defaultValue>
      <description></description>
      <id>2298fe56-a9d2-462e-92b7-4b66b71db856</id>
      <masked>false</masked>
      <name>chdrno</name>
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
