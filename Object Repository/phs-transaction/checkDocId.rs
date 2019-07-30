<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>checkDocId</name>
   <tag></tag>
   <elementGuidId>f4a8c6f2-eb00-4216-9cb7-9839f4fbed45</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;trxId\&quot;: ${trxId},\n  \&quot;genId\&quot;: \&quot;${genId}\&quot;,\n  \&quot;policyNo\&quot;: \&quot;${policyNo}\&quot;,\n  \&quot;spajNo\&quot;: \&quot;${spajNo}\&quot;,\n  \&quot;docIdType\&quot;: \&quot;${docIdType}\&quot;,\n  \&quot;statusCm\&quot;: \&quot;${statusCm}\&quot;,\n  \&quot;statusBpm\&quot;: \&quot;${statusBpm}\&quot;,\n  \&quot;statusDatacap\&quot;: \&quot;${statusDatacap}\&quot;,\n  \&quot;scanTimestamp\&quot;: \&quot;${scanTimestamp}\&quot;,\n  \&quot;count\&quot;: ${count},\n  \&quot;alterNumber\&quot;: \&quot;${alterNumber}\&quot;,\n  \&quot;trxIn\&quot;: ${trxIn},\n  \&quot;transactionType\&quot;: \&quot;${transactionType}\&quot;,\n  \&quot;statusProcess\&quot;: \&quot;${statusProcess}\&quot;,\n  \&quot;statusDelete\&quot;: ${statusDelete},\n  \&quot;alterDate\&quot;: ${alterDate},\n  \&quot;statusKtg\&quot;: \&quot;${statusKtg}\&quot;,\n  \&quot;clientNoOwner\&quot;: \&quot;${clientNoOwner}\&quot;,\n  \&quot;isUnitLink\&quot;: \&quot;${isUnitLink}\&quot;,\n  \&quot;verify\&quot;: ${verify}\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-transaction/rest/api/checkDocId</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>3d16b5f6-b6f7-491a-93c0-726df807d3fc</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(1, 1)</defaultValue>
      <description></description>
      <id>b610c1ec-9aa1-4d13-806c-b7a6b125f298</id>
      <masked>false</masked>
      <name>trxId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(2, 1)</defaultValue>
      <description></description>
      <id>f6221093-b2ae-4a2a-b3c8-3bd6147f8e3c</id>
      <masked>false</masked>
      <name>genId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(3, 1)</defaultValue>
      <description></description>
      <id>3a159fff-7830-4bbe-8d33-c0d587187220</id>
      <masked>false</masked>
      <name>policyNo</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(4, 1)</defaultValue>
      <description></description>
      <id>9e755428-dea8-4d28-9b8e-a310b8442f4a</id>
      <masked>false</masked>
      <name>spajNo</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(5, 1)</defaultValue>
      <description></description>
      <id>e8e2a14d-a751-4c47-b68f-1664ae0778b6</id>
      <masked>false</masked>
      <name>docIdType</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(6, 1)</defaultValue>
      <description></description>
      <id>1c88842b-1344-43db-a3fe-2d91d43b1d31</id>
      <masked>false</masked>
      <name>statusCm</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(7, 1)</defaultValue>
      <description></description>
      <id>66799895-90c3-43df-82e7-ef29070ed2a5</id>
      <masked>false</masked>
      <name>statusBpm</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(8, 1)</defaultValue>
      <description></description>
      <id>e7946776-a92a-4280-954f-1e88de701bc3</id>
      <masked>false</masked>
      <name>statusDatacap</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(9, 1)</defaultValue>
      <description></description>
      <id>1396d654-a726-454b-bf96-22cd45206aef</id>
      <masked>false</masked>
      <name>scanTimestamp</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(10, 1)</defaultValue>
      <description></description>
      <id>2001d450-a8b6-44f0-8a30-3964b40a408a</id>
      <masked>false</masked>
      <name>count</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(11, 1)</defaultValue>
      <description></description>
      <id>b3173cb7-3823-499c-90e5-20c5c6b70c36</id>
      <masked>false</masked>
      <name>alterNumber</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(12, 1)</defaultValue>
      <description></description>
      <id>97937122-659a-4251-8ff5-f51c9aed2cf8</id>
      <masked>false</masked>
      <name>trxIn</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(13, 1)</defaultValue>
      <description></description>
      <id>62da407b-515e-4eb7-9dc1-387bf325ada0</id>
      <masked>false</masked>
      <name>transactionType</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(14, 1)</defaultValue>
      <description></description>
      <id>66b86634-eaf6-4fc6-9a81-2bb1fbf5114b</id>
      <masked>false</masked>
      <name>statusProcess</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(15, 1)</defaultValue>
      <description></description>
      <id>827174da-f186-49a9-817c-f0bb310972d2</id>
      <masked>false</masked>
      <name>statusDelete</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(16, 1)</defaultValue>
      <description></description>
      <id>0c13c92e-24ad-43c6-87cf-c88100fa54b2</id>
      <masked>false</masked>
      <name>alterDate</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(17, 1)</defaultValue>
      <description></description>
      <id>f27b719a-2a8f-4218-a1cb-bc52b52ecfa7</id>
      <masked>false</masked>
      <name>statusKtg</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(18, 1)</defaultValue>
      <description></description>
      <id>b89a6036-0de3-460c-bec0-2fa6a2dae056</id>
      <masked>false</masked>
      <name>clientNoOwner</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(19, 1)</defaultValue>
      <description></description>
      <id>472f1219-60b0-4e78-a67e-cbc66484b3de</id>
      <masked>false</masked>
      <name>isUnitLink</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/checkDocId').getValue(20, 1)</defaultValue>
      <description></description>
      <id>9c78476c-9a4a-479b-95cd-c0236785c072</id>
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
