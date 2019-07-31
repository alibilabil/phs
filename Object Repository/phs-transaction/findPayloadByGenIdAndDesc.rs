<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>findPayloadByGenIdAndDesc</name>
   <tag></tag>
   <elementGuidId>485f0c17-f877-43ff-933e-5ad3416c4a4c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;genId\&quot;: \&quot;${genId}\&quot;,\n  \&quot;description\&quot;: \&quot;${description}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-transaction/rest/api/findPayloadByGenIdAndDesc</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>7859c4e8-14c9-4272-bec1-eaaa277849f4</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/findPayloadByGenIdAndDesc').getValue(1, 1)</defaultValue>
      <description></description>
      <id>d7b58fc5-c8ec-46a1-8ca9-f1ed5c2a405b</id>
      <masked>false</masked>
      <name>genId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/findPayloadByGenIdAndDesc').getValue(2, 1)</defaultValue>
      <description></description>
      <id>448e7be3-fb9f-433b-9f94-68ba3bb6bced</id>
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
