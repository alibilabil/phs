<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BORetriveDeleteExclusionClause</name>
   <tag></tag>
   <elementGuidId>861cd2f0-c1da-4090-a0d1-34e1d4b0fbbc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;CHDRNUM\&quot;: \&quot;12118866\&quot;,\n  \&quot;FUPCDE\&quot;: \&quot;E28\&quot;,\n  \&quot;ZALTNUM\&quot;: \&quot;A0000014\&quot;,\n  \&quot;FUPNO\&quot;: \&quot;00061\&quot;,\n  \&quot;OBJID\&quot;: \&quot;PHDELEXCLS\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-retrieve/rest/api/BORetriveDeleteExclusionClause</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>03af6ea0-63d2-4086-9876-a3e6c23f9a95</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('BORetriveDeleteExclusionClause').getValue(1, 1)</defaultValue>
      <description></description>
      <id>74db807b-7ce9-46b4-af6e-9f4d02c51ce9</id>
      <masked>false</masked>
      <name>chdrnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('BORetriveDeleteExclusionClause').getValue(2, 1)</defaultValue>
      <description></description>
      <id>87a741ec-7712-4f01-9993-720a1a02bef1</id>
      <masked>false</masked>
      <name>fupcde</name>
   </variables>
   <variables>
      <defaultValue>findTestData('BORetriveDeleteExclusionClause').getValue(3, 1)</defaultValue>
      <description></description>
      <id>8891e347-8b18-4505-9002-51ac4983bb00</id>
      <masked>false</masked>
      <name>zaltnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('BORetriveDeleteExclusionClause').getValue(4, 1)</defaultValue>
      <description></description>
      <id>2d409608-716e-4942-af8c-43c57ee7ec81</id>
      <masked>false</masked>
      <name>fupno</name>
   </variables>
   <variables>
      <defaultValue>findTestData('BORetriveDeleteExclusionClause').getValue(5, 1)</defaultValue>
      <description></description>
      <id>5d5eec56-1022-4361-8846-dc8a95c9b53a</id>
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
