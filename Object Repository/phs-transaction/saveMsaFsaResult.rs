<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>saveMsaFsaResult</name>
   <tag></tag>
   <elementGuidId>4ad99e4f-ed76-4738-bcb7-1c4d1ec96edd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: ${id},\n  \&quot;genId\&quot;: \&quot;${genId}\&quot;,\n  \&quot;trxIn\&quot;: ${trxIn},\n  \&quot;compareMsaTotal\&quot;: ${compareMsaTotal},\n  \&quot;compareFsaTotal\&quot;: ${compareFsaTotal},\n  \&quot;highestValueWithWop\&quot;: ${highestValueWithWop},\n  \&quot;highestValueWithoutWop\&quot;: ${highestValueWithoutWop},\n  \&quot;compareFsadeathAddccWithwop\&quot;: ${compareFsadeathAddccWithwop},\n  \&quot;compareNewFsadeathFsaaddccNonwop\&quot;: ${compareNewFsadeathFsaaddccNonwop},\n  \&quot;increasingMsaDeath\&quot;: \&quot;${increasingMsaDeath}\&quot;,\n  \&quot;increasingMsaAddCC\&quot;: \&quot;${increasingMsaAddCC}\&quot;,\n  \&quot;increasingFsaDeath\&quot;: \&quot;${increasingFsaDeath}\&quot;,\n  \&quot;increasingFsaAddCC\&quot;: \&quot;${increasingFsaAddCC}\&quot;,\n  \&quot;addCCLimit\&quot;: \&quot;${addCCLimit}\&quot;,\n  \&quot;clientNo\&quot;: \&quot;${clientNo}\&quot;,\n  \&quot;role\&quot;: \&quot;${role}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-transaction/rest/api/saveMsaFsaResult</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>44798d5b-eba7-4de7-b01a-880bbb66aec5</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(1, 1)</defaultValue>
      <description></description>
      <id>c169da01-1b78-4c10-b728-b95c1482199a</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(2, 1)</defaultValue>
      <description></description>
      <id>b21ef585-2c50-4d64-ab0e-2fd28c9639d6</id>
      <masked>false</masked>
      <name>genId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(3, 1)</defaultValue>
      <description></description>
      <id>398a7795-f0d0-4a0c-a64b-a8d958123f12</id>
      <masked>false</masked>
      <name>trxIn</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(4, 1)</defaultValue>
      <description></description>
      <id>f6464c1a-c432-4fd6-9582-e6d5e933a2be</id>
      <masked>false</masked>
      <name>compareMsaTotal</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(5, 1)</defaultValue>
      <description></description>
      <id>e8a8d861-7e58-49d6-89e2-9f4ed6dc5fca</id>
      <masked>false</masked>
      <name>compareFsaTotal</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(6, 1)</defaultValue>
      <description></description>
      <id>e8c916c4-cd11-4ae3-bcb8-aa1d5739010e</id>
      <masked>false</masked>
      <name>highestValueWithWop</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(7, 1)</defaultValue>
      <description></description>
      <id>c2bab120-e075-44f4-8db4-8f0b37b30a11</id>
      <masked>false</masked>
      <name>highestValueWithoutWop</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(8, 1)</defaultValue>
      <description></description>
      <id>e798ee64-83a3-46a9-94b5-8e6239590241</id>
      <masked>false</masked>
      <name>compareFsadeathAddccWithwop</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(9, 1)</defaultValue>
      <description></description>
      <id>48f999e1-6605-4ad4-a4ce-86adaba1140e</id>
      <masked>false</masked>
      <name>compareNewFsadeathFsaaddccNonwop</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(10, 1)</defaultValue>
      <description></description>
      <id>655333fd-0b0c-443e-85d0-4768a60bb535</id>
      <masked>false</masked>
      <name>increasingMsaDeath</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(11, 1)</defaultValue>
      <description></description>
      <id>752a0d47-1f6f-4105-abd4-0c0da3804c82</id>
      <masked>false</masked>
      <name>increasingMsaAddCC</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(12, 1)</defaultValue>
      <description></description>
      <id>b42523fe-c570-4951-acd4-5f8cdc880a6c</id>
      <masked>false</masked>
      <name>increasingFsaDeath</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(13, 1)</defaultValue>
      <description></description>
      <id>4ca604a6-3f56-47f9-aaff-fac744be35a0</id>
      <masked>false</masked>
      <name>increasingFsaAddCC</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(14, 1)</defaultValue>
      <description></description>
      <id>0887dd35-6704-4d3d-9aa6-2a73f2e16d91</id>
      <masked>false</masked>
      <name>addCCLimit</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(15, 1)</defaultValue>
      <description></description>
      <id>4d014444-65d7-449c-aeed-f930a62e4338</id>
      <masked>false</masked>
      <name>clientNo</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveMsaFsaResult').getValue(16, 1)</defaultValue>
      <description></description>
      <id>ee9b71a6-1d13-4119-8d68-d20ca11c69ee</id>
      <masked>false</masked>
      <name>role</name>
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
