<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>saveTaskDetail</name>
   <tag></tag>
   <elementGuidId>e6565a73-d5e9-4892-98ae-f27630c8ea89</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;trxLog\&quot;: {\n    \&quot;trxId\&quot;: ${trxId},\n    \&quot;genId\&quot;: \&quot;${genId}\&quot;,\n    \&quot;count\&quot;: ${count},\n    \&quot;statusDelete\&quot;: ${statusDelete},\n    \&quot;verify\&quot;: ${verify}\n  },\n  \&quot;taskDetail\&quot;: [\n    {\n      \&quot;tskdetailId\&quot;: ${tskdetailId},\n      \&quot;processId\&quot;: \&quot;${processId}\&quot;,\n      \&quot;processInstanceName\&quot;: \&quot;${processInstanceName}\&quot;,\n      \&quot;displayName\&quot;: \&quot;${displayName}\&quot;,\n      \&quot;taskId\&quot;: \&quot;${taskId}\&quot;,\n      \&quot;startTime\&quot;: ${startTime},\n      \&quot;trxIn\&quot;: ${trxIn},\n      \&quot;trxUpdate\&quot;: ${trxUpdate},\n      \&quot;processFlow\&quot;: \&quot;${processFlow}\&quot;,\n      \&quot;status\&quot;: \&quot;${status}\&quot;,\n      \&quot;role\&quot;: \&quot;${role}\&quot;,\n      \&quot;roleDistribution\&quot;: \&quot;${roleDistribution}\&quot;,\n      \&quot;taskName\&quot;: \&quot;${taskName}\&quot;,\n      \&quot;decision\&quot;: \&quot;${decision}\&quot;,\n      \&quot;username\&quot;: \&quot;${username}\&quot;\n    }\n  ],\n  \&quot;docId\&quot;: [${docId}]\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-transaction/rest/api/saveTaskDetail</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>50fdb396-44a7-4fe1-a636-51dd9d71185b</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(1, 1)</defaultValue>
      <description></description>
      <id>1eee668f-651a-4fa3-b6ca-73f0476186a8</id>
      <masked>false</masked>
      <name>trxId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(2, 1)</defaultValue>
      <description></description>
      <id>f8cd4239-04c3-4ee7-9da5-d280fa1ab2d0</id>
      <masked>false</masked>
      <name>genId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(3, 1)</defaultValue>
      <description></description>
      <id>6873f81e-679e-40f7-827d-259d4598f27d</id>
      <masked>false</masked>
      <name>count</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(4, 1)</defaultValue>
      <description></description>
      <id>d84a5e9f-9338-414e-b99d-143310e5eb08</id>
      <masked>false</masked>
      <name>statusDelete</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(5, 1)</defaultValue>
      <description></description>
      <id>9cf8fccd-afea-4d37-9c4e-3aad8384a919</id>
      <masked>false</masked>
      <name>verify</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(6, 1)</defaultValue>
      <description></description>
      <id>7684e552-75e6-4016-806b-a61400198019</id>
      <masked>false</masked>
      <name>tskdetailId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(7, 1)</defaultValue>
      <description></description>
      <id>5fccdd69-e294-4b09-9c34-1dec08ddd2bd</id>
      <masked>false</masked>
      <name>processId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(8, 1)</defaultValue>
      <description></description>
      <id>6033e289-31b5-4325-ac5c-65b49e680185</id>
      <masked>false</masked>
      <name>processInstanceName</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(9, 1)</defaultValue>
      <description></description>
      <id>98d76ba6-4fc7-4d81-b667-a86ac82563bd</id>
      <masked>false</masked>
      <name>displayName</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(10, 1)</defaultValue>
      <description></description>
      <id>6d9fd942-3294-40d5-bbda-51be5c882e8f</id>
      <masked>false</masked>
      <name>taskId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(11, 1)</defaultValue>
      <description></description>
      <id>e12a6fdc-a7ed-4a35-8968-72f797740cdd</id>
      <masked>false</masked>
      <name>startTime</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(12, 1)</defaultValue>
      <description></description>
      <id>9999a618-af43-4f46-8cf0-6efb50a6ddcd</id>
      <masked>false</masked>
      <name>trxIn</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(13, 1)</defaultValue>
      <description></description>
      <id>1c345f35-7957-4304-a53a-a52f3ba61205</id>
      <masked>false</masked>
      <name>trxUpdate</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(14, 1)</defaultValue>
      <description></description>
      <id>7e9bf533-042d-4fca-8ba5-355efe4790ce</id>
      <masked>false</masked>
      <name>processFlow</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(15, 1)</defaultValue>
      <description></description>
      <id>4f8ecc0a-d7fd-4b5f-a4ba-eea902b90006</id>
      <masked>false</masked>
      <name>status</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(16, 1)</defaultValue>
      <description></description>
      <id>ef10ff81-68cd-45ed-af89-81c38464909c</id>
      <masked>false</masked>
      <name>role</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(17, 1)</defaultValue>
      <description></description>
      <id>67a74731-2f39-4c97-8356-1a253898aa34</id>
      <masked>false</masked>
      <name>roleDistribution</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(18, 1)</defaultValue>
      <description></description>
      <id>77350cc7-0d27-4b37-86ea-29830f5f3928</id>
      <masked>false</masked>
      <name>taskName</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(19, 1)</defaultValue>
      <description></description>
      <id>3b59b235-0823-41ed-a20a-82f4ba32a730</id>
      <masked>false</masked>
      <name>decision</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(20, 1)</defaultValue>
      <description></description>
      <id>d66e7ca7-bd28-4f9d-b7d0-5b6957438657</id>
      <masked>false</masked>
      <name>username</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveTaskDetail').getValue(21, 1)</defaultValue>
      <description></description>
      <id>cc9827ba-c583-4f30-8ffc-a4f546069609</id>
      <masked>false</masked>
      <name>docId</name>
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
