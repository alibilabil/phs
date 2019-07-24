<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>updateAlterNumber</name>
   <tag></tag>
   <elementGuidId>23d58d5a-067f-4d83-9735-75d168e0486d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;trxId\&quot;: ${trxId},\n  \&quot;genId\&quot;: \&quot;${genId}\&quot;,\n  \&quot;count\&quot;: ${count},\n  \&quot;alterNumber\&quot;: \&quot;${alterNumber}\&quot;,\n  \&quot;statusDelete\&quot;: ${statusDelete},\n  \&quot;alterDate\&quot;: ${alterDate},\n  \&quot;isUnitLink\&quot;: \&quot;${isUnitLink}\&quot;,\n  \&quot;verify\&quot;: ${verify}\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-transaction/rest/api/updateAlterNumber</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>a91e0eea-044c-4994-b068-b0a95b3b21e6</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/updateAlterNumber').getValue(1, 1)</defaultValue>
      <description></description>
      <id>c949c20c-c0a1-4bd6-8aad-aba45418bb26</id>
      <masked>false</masked>
      <name>trxId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/updateAlterNumber').getValue(2, 1)</defaultValue>
      <description></description>
      <id>44a2c1db-0748-439a-a8fa-be39d7e5a409</id>
      <masked>false</masked>
      <name>genId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/updateAlterNumber').getValue(3, 1)</defaultValue>
      <description></description>
      <id>21e82aab-a66d-4b62-b130-311e92895633</id>
      <masked>false</masked>
      <name>count</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/updateAlterNumber').getValue(4, 1)</defaultValue>
      <description></description>
      <id>045812e2-b0d2-4414-87e1-d9c236e9de2a</id>
      <masked>false</masked>
      <name>alterNumber</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/updateAlterNumber').getValue(5, 1)</defaultValue>
      <description></description>
      <id>66c95025-cd61-47ce-b521-0ddada8d38db</id>
      <masked>false</masked>
      <name>statusDelete</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/updateAlterNumber').getValue(6, 1)</defaultValue>
      <description></description>
      <id>3de90fd8-a85c-4a33-9b84-ed6d7394b564</id>
      <masked>false</masked>
      <name>alterDate</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/updateAlterNumber').getValue(7, 1)</defaultValue>
      <description></description>
      <id>01971100-00bc-486a-9664-61ae02b58b5f</id>
      <masked>false</masked>
      <name>isUnitLink</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/updateAlterNumber').getValue(8, 1)</defaultValue>
      <description></description>
      <id>2939a92a-06e2-4438-84d7-495df6c9958f</id>
      <masked>false</masked>
      <name>verify</name>
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
