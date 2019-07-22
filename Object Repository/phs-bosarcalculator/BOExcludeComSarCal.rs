<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BOExcludeComSarCal</name>
   <tag></tag>
   <elementGuidId>24decc40-767f-4a59-96b8-dec8e69f776e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;objid\&quot; : \&quot;${objid}\&quot;,\n  \&quot;clntnum\&quot; : \&quot;${clntnum}\&quot;,\n  \&quot;currcy\&quot; : \&quot;${currcy}\&quot;,\n  \&quot;chdrnum\&quot; : \&quot;${chdrnum}\&quot;,\n  \&quot;rider\&quot; : \&quot;${rider}\&quot;,\n  \&quot;crtable\&quot; : \&quot;${crtable}\&quot; \n}\n&quot;,
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
   <restUrl>${endpoint}/phs-sarcalculator/rest/api/BOExcludeComSarCal</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>d6e991a0-bd4e-4843-b148-6babf17e570d</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOExcludeComSarCal').getValue(1, 1)</defaultValue>
      <description></description>
      <id>1c87e862-5b09-4db9-901a-79840537d119</id>
      <masked>false</masked>
      <name>objid</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOExcludeComSarCal').getValue(2, 1)</defaultValue>
      <description></description>
      <id>3f747d83-9ab0-4fd1-ae75-cd4ba3d91e06</id>
      <masked>false</masked>
      <name>clntnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOExcludeComSarCal').getValue(3, 1)</defaultValue>
      <description></description>
      <id>af34cd13-ea70-4928-aa36-4e5bac458687</id>
      <masked>false</masked>
      <name>currcy</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOExcludeComSarCal').getValue(4, 1)</defaultValue>
      <description></description>
      <id>1534efb6-429d-44e6-8358-3cc36a6c3f01</id>
      <masked>false</masked>
      <name>chdrnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOExcludeComSarCal').getValue(5, 1)</defaultValue>
      <description></description>
      <id>4e01cad8-0f5b-4bed-84d1-676ee215d915</id>
      <masked>false</masked>
      <name>rider</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOExcludeComSarCal').getValue(6, 1)</defaultValue>
      <description></description>
      <id>a7a6a298-38cd-4ceb-8fd1-08c176c931c0</id>
      <masked>false</masked>
      <name>crtable</name>
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
