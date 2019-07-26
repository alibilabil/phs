<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>phsParamSave</name>
   <tag></tag>
   <elementGuidId>1e184311-7c54-48a3-9963-35029e2e9124</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: ${id},\n  \&quot;param\&quot;: \&quot;${param}\&quot;,\n  \&quot;type\&quot;: \&quot;${type}\&quot;,\n  \&quot;value\&quot;: ${value}\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-masterdb/rest/api/phsParam/save</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>a294e5f8-a30b-47b0-bb0c-03ab7e81e54d</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/phsParamSave').getValue(1, 1)</defaultValue>
      <description></description>
      <id>36103653-aebd-4602-bcbd-66a89aa16b2f</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/phsParamSave').getValue(2, 1)</defaultValue>
      <description></description>
      <id>a50cf3ca-da85-4c76-ab98-f1a798c36479</id>
      <masked>false</masked>
      <name>param</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/phsParamSave').getValue(3, 1)</defaultValue>
      <description></description>
      <id>d36c1472-0104-474d-a112-202d79be33bd</id>
      <masked>false</masked>
      <name>type</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/phsParamSave').getValue(4, 1)</defaultValue>
      <description></description>
      <id>1a4499f7-12a7-49b4-8e50-04592c01e45f</id>
      <masked>false</masked>
      <name>value</name>
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
