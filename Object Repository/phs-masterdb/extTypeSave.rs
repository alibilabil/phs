<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>extTypeSave</name>
   <tag></tag>
   <elementGuidId>cc2421e1-b995-4800-9d73-61a8496bbfa7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: ${id},\n  \&quot;mime\&quot;: \&quot;${mime}\&quot;,\n  \&quot;status\&quot;: \&quot;${status}\&quot;,\n  \&quot;userId\&quot;: \&quot;${userId}\&quot;,\n  \&quot;value\&quot;: \&quot;${value}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-masterdb/rest/api/extType/save</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>73c72ed6-c60e-4560-9013-4add3755df76</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/extTypeSave').getValue(1, 1)</defaultValue>
      <description></description>
      <id>85008b51-a638-4cc7-9932-3c415ad583d5</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/extTypeSave').getValue(2, 1)</defaultValue>
      <description></description>
      <id>de3fa843-ea14-4170-9a49-9a2c0655fea4</id>
      <masked>false</masked>
      <name>mime</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/extTypeSave').getValue(3, 1)</defaultValue>
      <description></description>
      <id>689fefa7-7011-4f27-a06b-40ac23fee7e7</id>
      <masked>false</masked>
      <name>status</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/extTypeSave').getValue(4, 1)</defaultValue>
      <description></description>
      <id>46d41eab-031b-4b72-bd17-5f52bf4ea5fe</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/extTypeSave').getValue(5, 1)</defaultValue>
      <description></description>
      <id>2eafd941-89aa-4779-aae4-1c584c83f204</id>
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
