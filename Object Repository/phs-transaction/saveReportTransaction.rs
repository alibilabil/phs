<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>saveReportTransaction</name>
   <tag></tag>
   <elementGuidId>b80010e1-56d9-4ef0-b3a1-9c6820167c16</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;no_spajt\&quot;: \&quot;${no_spajt}\&quot;,\n  \&quot;genId\&quot;: \&quot;${genId}\&quot;,\n  \&quot;needDedup\&quot;: \&quot;${needDedup}\&quot;,\n  \&quot;vendorName\&quot;: \&quot;${vendorName}\&quot;,\n  \&quot;add_new_life\&quot;: \&quot;${add_new_life}\&quot;,\n  \&quot;status\&quot;: \&quot;${status}\&quot;,\n  \&quot;manual_get_client\&quot;: \&quot;${manual_get_client}\&quot;,\n\&quot;statusCase\&quot;: \&quot;${statusCase}\&quot;,\n  \&quot;docIdType\&quot;: \&quot;${docIdType}\&quot;,\n  \&quot;no_payor\&quot;: \&quot;${no_payor}\&quot;,\n  \&quot;policyNo\&quot;: \&quot;${policyNo}\&quot;,\n  \&quot;needMagnum\&quot;: \&quot;${needMagnum}\&quot;\n}\n&quot;,
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
   <restUrl>${endpoint}/phs-transaction/rest/api/saveReportTransaction</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.endpoint</defaultValue>
      <description></description>
      <id>0ab70118-92f7-4203-8b7a-85bdb7a5b590</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveReportTransaction').getValue(1, 1)</defaultValue>
      <description></description>
      <id>cd2b75d8-d8df-4665-9e40-f43912abe4cf</id>
      <masked>false</masked>
      <name>no_spajt</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveReportTransaction').getValue(2, 1)</defaultValue>
      <description></description>
      <id>f43dd915-9301-4f13-b9af-d55bfbd90178</id>
      <masked>false</masked>
      <name>genId</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveReportTransaction').getValue(3, 1)</defaultValue>
      <description></description>
      <id>7393ab91-cf97-454d-a7f1-0cc46bf7d086</id>
      <masked>false</masked>
      <name>needDedup</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveReportTransaction').getValue(4, 1)</defaultValue>
      <description></description>
      <id>7ce6acac-c4f8-4469-920c-ff0f4ce652d8</id>
      <masked>false</masked>
      <name>vendorName</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveReportTransaction').getValue(5, 1)</defaultValue>
      <description></description>
      <id>7ecc9ce8-894c-45dd-848b-62f2a255fe2b</id>
      <masked>false</masked>
      <name>add_new_life</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveReportTransaction').getValue(6, 1)</defaultValue>
      <description></description>
      <id>40aabf68-7d78-425b-ab14-f057c1bb2413</id>
      <masked>false</masked>
      <name>status</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveReportTransaction').getValue(7, 1)</defaultValue>
      <description></description>
      <id>3c471686-fa35-4a90-8ad4-ba523dc06f79</id>
      <masked>false</masked>
      <name>manual_get_client</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveReportTransaction').getValue(8, 1)</defaultValue>
      <description></description>
      <id>e0c4eda4-8430-4ef0-97e1-498b7e8ea6da</id>
      <masked>false</masked>
      <name>statusCase</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveReportTransaction').getValue(9, 1)</defaultValue>
      <description></description>
      <id>bc27f2c9-6639-4079-8589-aac17a592a76</id>
      <masked>false</masked>
      <name>docIdType</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveReportTransaction').getValue(10, 1)</defaultValue>
      <description></description>
      <id>1490fc75-8c70-4127-ba53-0acdab999d13</id>
      <masked>false</masked>
      <name>no_payor</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveReportTransaction').getValue(11, 1)</defaultValue>
      <description></description>
      <id>0ea5664b-c91e-4d9a-abe0-8ba39c5f9a8f</id>
      <masked>false</masked>
      <name>policyNo</name>
   </variables>
   <variables>
      <defaultValue>findTestData('phs-transaction/saveReportTransaction').getValue(12, 1)</defaultValue>
      <description></description>
      <id>05ee101e-a55f-4bc7-9e5a-f9a1ffd507ef</id>
      <masked>false</masked>
      <name>needMagnum</name>
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
