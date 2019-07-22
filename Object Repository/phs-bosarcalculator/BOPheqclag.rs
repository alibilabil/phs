<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BOPheqclag</name>
   <tag></tag>
   <elementGuidId>fa9eec11-cbba-4cbb-9990-0dfab845ab24</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;objid\&quot;: \&quot;${objid}\&quot;,\n  \&quot;clntnum\&quot;: \&quot;${clntnum}\&quot;,\n  \&quot;currcd\&quot;: \&quot;${currcd}\&quot;,\n  \&quot;chdrnum\&quot;: \&quot;${chdrnum)\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-sarcalculator/rest/api/BOPheqclag</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>dd113290-6094-474f-9252-5f3e06fee029</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOPheqclag').getValue(1, 1)</defaultValue>
      <description></description>
      <id>fcf9f4a9-35e1-4c20-85ca-7f13948bba48</id>
      <masked>false</masked>
      <name>objid</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOPheqclag').getValue(2, 1)</defaultValue>
      <description></description>
      <id>3aaa5acb-e42f-42e6-97ae-47da4712b3f2</id>
      <masked>false</masked>
      <name>clntnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOPheqclag').getValue(3, 1)</defaultValue>
      <description></description>
      <id>884c27e0-140b-443f-9426-965ef5fa2066</id>
      <masked>false</masked>
      <name>currcd</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-bosarcalculator/BOPheqclag').getValue(4, 1)</defaultValue>
      <description></description>
      <id>ebca2c8c-1e16-46ff-a676-383c3512df84</id>
      <masked>false</masked>
      <name>chdrnum</name>
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
