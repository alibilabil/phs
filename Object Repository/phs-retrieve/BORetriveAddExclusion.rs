<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BORetriveAddExclusion</name>
   <tag></tag>
   <elementGuidId>7e2a8b7f-997c-4f78-b2fe-c3d2f0ec1953</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;CHDRNUM\&quot;: \&quot;${chdrnum}\&quot;,\n  \&quot;ZALTNUM\&quot;: \&quot;${zaltnum}\&quot;,\n  \&quot;FUPNO\&quot;: \&quot;${fupno}\&quot;,\n  \&quot;FUPCDE\&quot;: \&quot;${fupcde}\&quot;,\n  \&quot;HXCLLETTYP\&quot;: \&quot;\&quot;,\n  \&quot;HXCLNOTE1\&quot;: \&quot;Test test\&quot;,\n  \&quot;HXCLNOTE2\&quot;: \&quot;Tes Tes satu dua tiga 123 !@#$\&quot;,\n  \&quot;OBJID\&quot;: \&quot;${objid}\&quot;\n}&quot;,
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
   <restUrl>${endpoint}/phs-retrieve/rest/api/BORetriveAddExclusion</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>3a486d0e-7f2b-4ae2-b728-395db743fc73</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('BORetriveAddExclusion').getValue(1, 1)</defaultValue>
      <description></description>
      <id>fb13c1e5-21a0-4689-9258-7944e9ec2815</id>
      <masked>false</masked>
      <name>chdrnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('BORetriveAddExclusion').getValue(2, 1)</defaultValue>
      <description></description>
      <id>b650d08c-eed8-4e85-915d-083d208a5459</id>
      <masked>false</masked>
      <name>zaltnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('BORetriveAddExclusion').getValue(3, 1)</defaultValue>
      <description></description>
      <id>b3974d9d-d780-4f1e-961b-b476131123f6</id>
      <masked>false</masked>
      <name>fupno</name>
   </variables>
   <variables>
      <defaultValue>findTestData('BORetriveAddExclusion').getValue(4, 1)</defaultValue>
      <description></description>
      <id>b4d17a46-a129-41dc-ba2d-8805869ba467</id>
      <masked>false</masked>
      <name>fupcde</name>
   </variables>
   <variables>
      <defaultValue>findTestData('BORetriveAddExclusion').getValue(5, 1)</defaultValue>
      <description></description>
      <id>72cc3faa-8ecc-411f-8bfe-6a2ebeebe1ac</id>
      <masked>false</masked>
      <name>objid</name>
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
