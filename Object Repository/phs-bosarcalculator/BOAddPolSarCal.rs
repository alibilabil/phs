<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BOAddPolSarCal</name>
   <tag></tag>
   <elementGuidId>b8d72348-b874-48d7-8bf8-a4202c08c4ac</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;objid\&quot;: \&quot;${objid}\&quot;,\n  \&quot;clntnum\&quot;: \&quot;${clntnum}\&quot;,\n  \&quot;currcy\&quot;: \&quot;${currcy}\&quot;,\n  \&quot;chdrnum\&quot;: \&quot;${chdrnum}\&quot;,\n  \&quot;crtable\&quot;: \&quot;${crtable}\&quot;,\n  \&quot;rcdate\&quot;: \&quot;${rcdate}\&quot;,\n  \&quot;rcesagenew\&quot;: \&quot;${rcesagenew}\&quot;,\n  \&quot;rcestrmnew\&quot;: \&quot;${rcestrmnew}\&quot;,\n  \&quot;suminsnew\&quot;: \&quot;${suminsnew}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-bosarcalculator/rest/api/BOAddPolSarCal</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>25299225-6894-4f57-9911-35acf6def6b1</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddPolSarCal').getValue(1, 1)</defaultValue>
      <description></description>
      <id>cfa7a010-71f2-4a17-8755-ab7d11531fbe</id>
      <masked>false</masked>
      <name>objid</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddPolSarCal').getValue(2, 1)</defaultValue>
      <description></description>
      <id>2ed0f6fd-7b56-4c64-a41a-c4ad452214c8</id>
      <masked>false</masked>
      <name>clntnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddPolSarCal').getValue(3, 1)</defaultValue>
      <description></description>
      <id>8a56c03d-4a67-4783-8a86-8270ac23bec4</id>
      <masked>false</masked>
      <name>currcy</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddPolSarCal').getValue(4, 1)</defaultValue>
      <description></description>
      <id>e3b63793-4df8-4952-b3ac-f64fb99e0727</id>
      <masked>false</masked>
      <name>chdrnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddPolSarCal').getValue(5, 1)</defaultValue>
      <description></description>
      <id>ac532d68-34f6-424f-8586-9ae1d64e2286</id>
      <masked>false</masked>
      <name>crtable</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddPolSarCal').getValue(6, 1)</defaultValue>
      <description></description>
      <id>81e40e4e-c6af-4ddf-96a8-9da8b1bec4bd</id>
      <masked>false</masked>
      <name>rcdate</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddPolSarCal').getValue(7, 1)</defaultValue>
      <description></description>
      <id>c195488c-e8bc-4beb-ad9c-8043e64c22bc</id>
      <masked>false</masked>
      <name>rcesagenew</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddPolSarCal').getValue(8, 1)</defaultValue>
      <description></description>
      <id>cac48910-6e34-4bec-a5ad-0025c335b0b1</id>
      <masked>false</masked>
      <name>rcestrmnew</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOAddPolSarCal').getValue(9, 1)</defaultValue>
      <description></description>
      <id>8a5a93b0-8cf2-4b3b-b171-f398918f9e7b</id>
      <masked>false</masked>
      <name>suminsnew</name>
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
