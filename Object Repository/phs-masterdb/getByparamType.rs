<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>getByparamType</name>
   <tag></tag>
   <elementGuidId>2fdbcf02-a01d-4c55-bed5-1eb3ba092f09</elementGuidId>
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
   <restUrl>${endpoint}/phs-masterdb/rest/api/phsParam/getByparamType</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>55855166-05ac-4588-9aae-b49eea8f8cce</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/getByparamType').getValue(1, 1)</defaultValue>
      <description></description>
      <id>f57fd385-7aa4-4b88-ad57-c35735b68166</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/getByparamType').getValue(2, 1)</defaultValue>
      <description></description>
      <id>b46bcf47-ec36-4ddc-885e-5f892defe497</id>
      <masked>false</masked>
      <name>param</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/getByparamType').getValue(3, 1)</defaultValue>
      <description></description>
      <id>893a6f96-0cbe-4133-b691-f9fa31115e9d</id>
      <masked>false</masked>
      <name>type</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-masterdb/getByparamType').getValue(4, 1)</defaultValue>
      <description></description>
      <id>6419daa1-bd66-40cd-99ee-9cb68954cdc3</id>
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
