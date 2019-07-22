<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BOModCompSarCal</name>
   <tag></tag>
   <elementGuidId>5606b731-6649-4b16-ab8b-6f06d78ad126</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;objid\&quot;: \&quot;${objid}\&quot;,\n  \&quot;clntnum\&quot;: \&quot;${clntnum}\&quot;,\n  \&quot;currcy\&quot;: \&quot;${currcy}\&quot;,\n  \&quot;crtable01\&quot;: \&quot;${crtable01}\&quot;,\n  \&quot;rcdate\&quot;: \&quot;${rcdate}\&quot;,\n  \&quot;rcesagenew\&quot;: \&quot;${rcesagenew}\&quot;,\n  \&quot;rcestrmnew\&quot;: \&quot;${rcestrmnew}\&quot;,\n  \&quot;suminsnew\&quot;: \&quot;${suminsnew}\&quot;,\n  \&quot;zsprm\&quot;: \&quot;${zsprm}\&quot;,\n  \&quot;chdrnum\&quot;: \&quot;${chdrnum}\&quot;,\n  \&quot;rider\&quot;: \&quot;${rider}\&quot;,\n  \&quot;crtable02\&quot;: \&quot;${crtable02}\&quot;\n}\n&quot;,
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
   <restUrl>{endpoint}/phs-sarcalculator/rest/api/BOModCompSarCal</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>6b2fecac-0536-4770-8cac-f6282dd90219</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOModCompSarCal').getValue(1, 1)</defaultValue>
      <description></description>
      <id>3f0c31e1-a1a0-4ecf-ba47-de0315b34155</id>
      <masked>false</masked>
      <name>objid</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOModCompSarCal').getValue(2, 1)</defaultValue>
      <description></description>
      <id>2b3c463e-71e3-445d-8783-2ca5f545fe52</id>
      <masked>false</masked>
      <name>clntnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOModCompSarCal').getValue(3, 1)</defaultValue>
      <description></description>
      <id>6a0f4ed1-8792-417d-b010-57e6c03a7378</id>
      <masked>false</masked>
      <name>currcy</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOModCompSarCal').getValue(4, 1)</defaultValue>
      <description></description>
      <id>d4cc8570-1f63-4510-9331-1c5125d9bbd2</id>
      <masked>false</masked>
      <name>crtable01</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOModCompSarCal').getValue(5, 1)</defaultValue>
      <description></description>
      <id>1a016d52-9201-47d5-938c-5a9d7b9e68a3</id>
      <masked>false</masked>
      <name>rcdate</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOModCompSarCal').getValue(6, 1)</defaultValue>
      <description></description>
      <id>a21e0196-fa71-40e6-bddd-cd3686f476a7</id>
      <masked>false</masked>
      <name>rcesagenew</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOModCompSarCal').getValue(7, 1)</defaultValue>
      <description></description>
      <id>f45e1b9d-46ec-4f1e-83e7-57217c36e584</id>
      <masked>false</masked>
      <name>rcestrmnew</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOModCompSarCal').getValue(8, 1)</defaultValue>
      <description></description>
      <id>767bf41b-f315-49d8-a470-ffb7732b68ab</id>
      <masked>false</masked>
      <name>suminsnew</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOModCompSarCal').getValue(9, 1)</defaultValue>
      <description></description>
      <id>c635a7db-d160-4f53-b2cd-dc5e350e511a</id>
      <masked>false</masked>
      <name>zsprm</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOModCompSarCal').getValue(10, 1)</defaultValue>
      <description></description>
      <id>9e5945aa-6bca-4940-a839-e458f4877886</id>
      <masked>false</masked>
      <name>chdrnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOModCompSarCal').getValue(11, 1)</defaultValue>
      <description></description>
      <id>b57a53bf-3638-4154-a259-f026f1467b02</id>
      <masked>false</masked>
      <name>rider</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOModCompSarCal').getValue(12, 1)</defaultValue>
      <description></description>
      <id>3d36d863-9e40-49a6-822d-de035b72c876</id>
      <masked>false</masked>
      <name>crtable02</name>
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
