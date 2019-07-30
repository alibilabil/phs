<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>getErrorLogByGenId</name>
   <tag></tag>
   <elementGuidId>c73f7914-103b-4bc9-8e42-821fbcaf6ea3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
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
   <restUrl>${endpoint}/phs-transaction/rest/api/getErrorLogByGenId</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>7ed0bc7d-208a-4b9d-aed8-c12e93bfb9b5</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/getErrorLogByGenId').getValue(1, 1)</defaultValue>
      <description></description>
      <id>d2c9cd7a-9fc0-4351-b8c6-1e062bd9d4c9</id>
      <masked>false</masked>
      <name>genId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/getErrorLogByGenId').getValue(2, 1)</defaultValue>
      <description></description>
      <id>e3e73c33-eabf-46db-a14a-d83dc034749c</id>
      <masked>false</masked>
      <name>message</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/getErrorLogByGenId').getValue(3, 1)</defaultValue>
      <description></description>
      <id>3764daa8-d244-4459-b9fe-e99e09a4bb31</id>
      <masked>false</masked>
      <name>taskName</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/getErrorLogByGenId').getValue(4, 1)</defaultValue>
      <description></description>
      <id>90fd5fb8-d28a-435b-bc66-e855bccab0ad</id>
      <masked>false</masked>
      <name>trxIn</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/getErrorLogByGenId').getValue(5, 1)</defaultValue>
      <description></description>
      <id>6ada7b99-e780-497e-930a-5c3c4ec945d5</id>
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
