<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BOExclusionClause</name>
   <tag></tag>
   <elementGuidId>68c143ec-6e48-4105-9412-76ed8dd9973e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;chdrnum\&quot;: \&quot;${chdrnum}\&quot;,\n  \&quot;zaltnum\&quot;: \&quot;${zaltnum}\&quot;,\n  \&quot;fupno\&quot;: \&quot;${fupno}\&quot;,\n  \&quot;objid\&quot;: \&quot;${objid}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-retrieve/rest/api/BOExclusionClause</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>388059d8-988b-4f96-87cc-ece9d7fe38b7</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-retrieve/BOExclusionClause').getValue(1, 1)</defaultValue>
      <description></description>
      <id>2ca025cf-441b-40ed-89f0-5110a72b60d3</id>
      <masked>false</masked>
      <name>chdrnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-retrieve/BOExclusionClause').getValue(2, 1)</defaultValue>
      <description></description>
      <id>b5c3bd97-ee9c-4968-af9c-f20099d27043</id>
      <masked>false</masked>
      <name>zaltnum</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-retrieve/BOExclusionClause').getValue(3, 1)</defaultValue>
      <description></description>
      <id>5591bfbd-fc75-4305-8b50-86bd173df010</id>
      <masked>false</masked>
      <name>fupno</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-retrieve/BOExclusionClause').getValue(4, 1)</defaultValue>
      <description></description>
      <id>c46015e2-b4b0-45c7-8371-a7634fd87653</id>
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
